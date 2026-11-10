{ self, pkgs, crane, msrvRust, buildInputs, nativeBuildInputs }:
let
  craneLib = (crane.mkLib pkgs).overrideToolchain (p: msrvRust);
  src = ../..;
  doCheck = false;
in
rec {
  default = czkawka-cli;
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
