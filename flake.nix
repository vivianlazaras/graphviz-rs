{
  description = "Static Graphviz build for Rust FFI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        static-graphviz = pkgs.graphviz.overrideAttrs (old: {
  pname = "static-graphviz";

  patches = (old.patches or []) ++ [
    # Patch out vimdot from Makefile.am and Makefile.in
    (pkgs.runCommand "disable-vimdot" { } ''
      mkdir -p $out
      cat > $out/disable-vimdot.patch <<EOF
--- a/plugin/Makefile.am
+++ b/plugin/Makefile.am
@@ -1,7 +1,6 @@
 SUBDIRS = \
-  vimdot \\
   core \
   layout \
   neato_layout \
@@ -20,7 +19,6 @@
 
 SUBDIRS = \
-  vimdot \\
   core \
   layout \
   neato_layout \
EOF
    '')
  ];

  configureFlags = (old.configureFlags or []) ++ [
    "--enable-static"
    "--disable-shared"
    "--disable-tcl"
    "--disable-java"
    "--disable-php"
    "--disable-python"
    "--disable-guile"
    "--disable-gtk"
    "--disable-qt"
    "--disable-x"
    "--without-x"
    "--without-tclsh"
    "--without-wish"
  ];

  buildInputs = (old.buildInputs or []);
  nativeBuildInputs = (old.nativeBuildInputs or []);
  doCheck = false;
  doInstallCheck = false;
});
      in {
        packages.default = static-graphviz;

        devShells.default = pkgs.mkShell {
          name = "graphviz-static-dev";
          packages = [
            static-graphviz
            pkgs.pkg-config
          ];

          shellHook = ''
            echo "Static Graphviz with libgvc and libcgraph ready for FFI."
          '';
        };
      });
}