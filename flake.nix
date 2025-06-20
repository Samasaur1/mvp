{
  description = "Sort files into prefix-based directories by regex";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
  };

  outputs = { self, nixpkgs, ... }:
    let
      allSystems = nixpkgs.lib.systems.flakeExposed;
      forAllSystems = nixpkgs.lib.genAttrs allSystems;
      define = f: forAllSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            config = {
            };
            overlays = [
            ];
          };
        in
          f pkgs
      );
    in {
      packages = define (pkgs: {
        default = pkgs.callPackage ./. { };
      });

      devShells = define (pkgs: {
        default = pkgs.mkShell {
          name = "mvp dev shell";

          buildInputs = [
            pkgs.cargo
            pkgs.rustc
          ];
        };
      });
    };
}
