{
  description = "My Rust project with Trunk";

  inputs = {
		flake-utils.url = "github:numtide/flake-utils";
		rust-overlay.url = "github:oxalica/rust-overlay";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

	outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default;
      in {
        devShell = pkgs.mkShell {
          buildInputs =
          	[ 
							(rustVersion.override { extensions = [ "rust-src" ]; }) 
							pkgs.cmake
							pkgs.pkg-config
							pkgs.glib
							pkgs.fontconfig
							pkgs.atk
						];
					enableTrunk = true;

        };
      });
}

