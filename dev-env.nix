let
  pkgs    = import <nixpkgs> {};
  stdenv  = pkgs.stdenv;
  lib     = pkgs.lib;

in rec {
  devEnv = stdenv.mkDerivation rec {
    name = "rust-web-framework-comparison-dev-env";
    src = ./.;
    buildInputs = with pkgs; [
      git
      rustPlatform.rustc
      cargo
      pkgconfig
      openssl
    ];
  };
}
