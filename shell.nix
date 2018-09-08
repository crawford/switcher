with import <nixpkgs> {
  overlays = [ (import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz)) ];
};

stdenv.mkDerivation {
  name = "poe";
  buildInputs = [
    ((rustChannelOf {
      date = "2018-09-08";
      channel = "nightly";
    }).rust.override {
      targets = [ "thumbv7m-none-eabi" ];
      extensions = [ "rust-std" "rustfmt-preview" "clippy-preview" ];
    })
  ];
}
