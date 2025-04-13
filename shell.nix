{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell {
buildInputs = [rustup termdown]; # your dependencies here
}
