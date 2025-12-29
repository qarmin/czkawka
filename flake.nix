{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11-small";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        cargoToml = (builtins.fromTOML (builtins.readFile ./czkawka_core/Cargo.toml));
      in
      {
        packages = (import ./misc/nix/packages.nix { 
          inherit self pkgs crane;
          msrvRust = pkgs.rust-bin.stable.${cargoToml.package.rust-version}.minimal;
          buildInputs = with pkgs; [
            atk
            cairo
            gdk-pixbuf
            glib
            gtk4
            pango
          ];
          nativeBuildInputs = with pkgs; [
            pkg-config
            gobject-introspection
            gsettings-desktop-schemas
            wrapGAppsHook4
          ];
        });
      }
    );
}
