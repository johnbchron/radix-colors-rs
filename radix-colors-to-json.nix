{ pkgs }: let
    src = ./js;

  js2nix = pkgs.callPackage (pkgs.fetchgit {
    url = "https://github.com/canva-public/js2nix";
    hash = "sha256-Bmv0ERVeb6vjYzy4MuCDgSiz9fSm/Bhg+Xk3AxPisBw=";
  }) { };

  node-env = (js2nix {
    package-json = src + "/package.json";
    yarn-lock = src + "/yarn.lock";
  }).nodeModules;

  
in
  pkgs.stdenv.mkDerivation {
    name = "radix-colors-json";
  
    inherit src;

    nativeBuildInputs = with pkgs; [
      nodejs
    ];

    buildPhase = ''
      mkdir -p $out

      ln -s ${node-env} ./node_modules
      ls -alh

      # Run the conversion script
      node index.js
    '';

    installPhase = ''
      cp colors.json $out/
    '';
  }
