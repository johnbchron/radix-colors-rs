use std::fs;

use proc_macro::TokenStream;
use quote::quote;
use serde_json::Value;
use syn::{LitStr, parse_macro_input};

enum ParsedColor {
  RGB(u8, u8, u8),
  RGBA(u8, u8, u8, u8),
  P3(f32, f32, f32),
}

#[proc_macro]
pub fn generate_color_constants(input: TokenStream) -> TokenStream {
  // Parse the input as a string literal containing the file path
  let file_path = parse_macro_input!(input as LitStr).value();

  // Read the JSON file
  let json_content =
    fs::read_to_string(file_path).expect("Failed to read color constants file");

  // Parse the JSON content
  let colors: Value =
    serde_json::from_str(&json_content).expect("Failed to parse JSON content");

  // Generate the color structs and implementations
  let mut color_tokens = quote! {};

  // Helper function to convert hex to RGB
  fn hex_to_rgb(hex: &str) -> Option<ParsedColor> {
    if hex.len() == 9 && hex.starts_with('#') {
      // Handle RGBA format
      let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0);
      let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0);
      let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(0);
      let a = u8::from_str_radix(&hex[7..9], 16).unwrap_or(0);
      Some(ParsedColor::RGBA(r, g, b, a))
    } else if hex.len() == 7 && hex.starts_with('#') {
      // Handle RGB format
      let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0);
      let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0);
      let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(0);
      Some(ParsedColor::RGB(r, g, b))
    }
    // color(display-p3 0.082 0.07 0.05)
    else if hex.starts_with("color(display-p3") {
      let parts: Vec<&str> = hex
        .trim_start_matches("color(display-p3")
        .trim_end_matches(")")
        .split_whitespace()
        .collect();
      if parts.len() == 3 {
        let r = parts[0].parse::<f32>().unwrap();
        let g = parts[1].parse::<f32>().unwrap();
        let b = parts[2].parse::<f32>().unwrap();
        Some(ParsedColor::P3(r, g, b))
      } else {
        None
      }
    } else {
      None
    }
  }

  // Process each color palette
  if let Value::Object(palettes) = colors {
    for (palette_name, palette_values) in palettes {
      if let Value::Object(colors) = palette_values {
        // Generate constant getters for each color
        for (color_name, color_value) in colors {
          if let Value::String(hex) = color_value {
            let Some(color) = hex_to_rgb(&hex) else {
              continue;
            };

            let color_ident = syn::Ident::new(
              &format!(
                "{}_{}",
                palette_name.to_uppercase(),
                color_name.to_uppercase()
              ),
              proc_macro2::Span::call_site(),
            );

            match color {
              ParsedColor::RGB(r, g, b) => {
                color_tokens.extend(quote! {
                  pub const #color_ident: ColorU8 = ColorU8 { r: #r, g: #g, b: #b };
                });
              }
              ParsedColor::RGBA(r, g, b, a) => {
                color_tokens.extend(quote! {
                  pub const #color_ident: ColorU8A = ColorU8A { r: #r, g: #g, b: #b, a: #a };
                });
              }
              ParsedColor::P3(r, g, b) => {
                color_tokens.extend(quote! {
                  pub const #color_ident: ColorP3 = ColorP3 { r: #r, g: #g, b: #b };
                });
              }
            }
          }
        }
      }
    }
  }

  // Generate the final output
  let output = quote! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ColorU8 {
      pub r: u8,
      pub g: u8,
      pub b: u8,
    }

    impl ColorU8 {
      pub fn hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
      }
      pub fn u8(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
      }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ColorU8A {
      pub r: u8,
      pub g: u8,
      pub b: u8,
      pub a: u8,
    }

    impl ColorU8A {
      pub fn hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
      }
      pub fn u8(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
      }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct ColorP3 {
      pub r: f32,
      pub g: f32,
      pub b: f32,
    }

    impl ColorP3 {
      pub fn html(&self) -> String {
        format!("color(display-p3 {} {} {})", self.r, self.g, self.b)
      }
      pub fn f32(&self) -> (f32, f32, f32) {
        (self.r, self.g, self.b)
      }
    }

    #color_tokens
  };

  output.into()
}
