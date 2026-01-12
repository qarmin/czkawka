{ self, pkgs, crane, msrvRust, buildInputs, nativeBuildInputs }:
let
  craneLib = (crane.mkLib pkgs).overrideToolchain (p: msrvRust);
  src = ../..;
  doCheck = false;
in
rec {
  default = czkawka-gui-wayland;
  czkawka-gui = let
    cargoToml = "${self}/../../czkawka_gui/Cargo.toml";
    cargoTomlConfig = builtins.fromTOML (builtins.readFile cargoToml);
    version = cargoTomlConfig.package.version;
  in
  craneLib.buildPackage {
    inherit version src cargoToml buildInputs nativeBuildInputs doCheck;
    name = "czkawka-gui";
    cargoExtraArgs = "--bin czkawka_gui";
    cargoArtifacts = craneLib.buildDepsOnly {
      inherit version src cargoToml buildInputs nativeBuildInputs doCheck;
      name = "czkawka-gui";
      cargoExtraArgs  = "--bin czkawka_gui";
    };
  };
  wrapped-czkawka-gui = pkgs.writeShellScriptBin "wrapped-czkawka-gui" ''
    export GSETTINGS_SCHEMA_DIR ="${pkgs.gtk4}/share/gsettings-schemas/gtk4-${pkgs.gtk4.version}/glib-2.0/schemas";
    exec ${czkawka-gui}/bin/czkawka_gui "$@"
  '';
  czkawka-gui-wayland = let
    cargoToml = "${self}/../../czkawka_gui/Cargo.toml";
    cargoTomlConfig = builtins.fromTOML (builtins.readFile cargoToml);
    version = cargoTomlConfig.package.version;
    waylandBuildInputs = buildInputs ++ [ pkgs.wayland ];
  in
  craneLib.buildPackage {
    inherit version src cargoToml nativeBuildInputs doCheck;
    buildInputs = waylandBuildInputs;
    name = "czkawka-gui";
    cargoExtraArgs = "--bin czkawka_gui";
    cargoArtifacts = craneLib.buildDepsOnly {
      inherit version src cargoToml nativeBuildInputs doCheck;
      name = "czkawka-gui";
      cargoExtraArgs  = "--bin czkawka_gui";
    };
  };
  czkawka-cli = let
    cargoToml = "${self}/../../czkawka_cli/Cargo.toml";
    cargoTomlConfig = builtins.fromTOML (builtins.readFile cargoToml);
    version = cargoTomlConfig.package.version;
  in
  craneLib.buildPackage {
    inherit version src cargoToml doCheck;
    buildInputs = [];
    nativeBuildInputs = [];
    name = "czkawka-cli";
    cargoExtraArgs = "--bin czkawka_cli";
    cargoArtifacts = craneLib.buildDepsOnly {
      inherit version src cargoToml buildInputs nativeBuildInputs doCheck;
      name = "czkawka-cli";
      cargoExtraArgs  = "--bin czkawka_cli";
    };
  };
}
