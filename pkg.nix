{ rustPlatform, lib, ... }:

rustPlatform.buildRustPackage {
  pname = "mini-nix";
  version = "1.0";

  src = ./.;

  cargoHash = "sha256-PhW6MYsFFho0vXP56s7tc3SuIDcBzt6Mi28qccNqiog=";

  meta = with lib; {
    description = "A small nix interpreter";
    license = licenses.gpl2;
    platforms = platforms.all;
  };
}
