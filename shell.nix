{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = [
    pkgs.elixir
    pkgs.erlang
    pkgs.rustc
    pkgs.cargo
  ];
}
