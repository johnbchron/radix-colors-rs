//! `radix-colors-rs` is a library that provides color constants from the Radix
//! UI Colors library.
//!
//! All constant names are generated from the palette and color names in the
//! JSON file. Having all the constants loose like this is great for
//! cherry-picking, but if you need to use all the colors, consider directly
//! using the JSON file we're generating these from.
//!
//! Every color constant name is the palette name taken from the JS library
//! (`blueDarkP3`, `pinkA`) combined with the color name (`orangeA7`, `mint2`)
//! like `{palette}_{color}`, and uppercased.
//!
//! Regular colors, alpha colors, and P3 colors are given separate types.

radix_colors_derive::generate_color_constants!(include_str!("../colors.json"));
