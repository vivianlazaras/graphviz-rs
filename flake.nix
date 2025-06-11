{
  description = "Statically compiled Graphviz for Rust FFI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (final: prev: {
              # Custom graphviz with static build flags
              static-graphviz = prev.graphviz.overrideAttrs (old: {
                pname = "static-graphviz";
                buildInputs = (old.buildInputs or []) ++ [ prev.pkg-config ];
                configureFlags = (old.configureFlags or []) ++ [
                  "--enable-static"
                  "--disable-shared"
                  "--without-x"
                ];
                makeFlags = (old.makeFlags or []) ++ [ "LDFLAGS=-static" ];
              });
            })
          ];
        };

      in {
        packages.default = pkgs.static-graphviz;

        devShells.default = pkgs.mkShell {
          name = "graphviz-static-dev";
          packages = [
            pkgs.static-graphviz
            pkgs.pkg-config
            pkgs.graphviz
          ];

          shellHook = ''
            echo "Static Graphviz available. Use pkg-config to locate libraries."
            echo "You can now link libgvc, libcgraph, etc. from your Rust build."
          '';
        };
      });
}