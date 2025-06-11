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

        remove-vimdot-plugin-makefile-am = pkgs.writeText "remove-vimdot-plugin-makefile-am.patch" ''
        --- a/plugin/Makefile.am
        +++ b/plugin/Makefile.am
        @@ -1,7 +1,6 @@
        -SUBDIRS = \
        -  vimdot \
        -  core \
        -  layout \
        -  neato_layout \
        -  fdp_layout \
        -  sfdp_layout \
        -  circo_layout \
        -  tred_layout
        +SUBDIRS = \
        +  core \
        +  layout \
        +  neato_layout \
        +  fdp_layout \
        +  sfdp_layout \
        +  circo_layout \
        +  tred_layout
      '';

      remove-vimdot-plugin-makefile-in = pkgs.writeText "remove-vimdot-plugin-makefile-in.patch" ''
        --- a/plugin/Makefile.in
        +++ b/plugin/Makefile.in
        @@ -1,7 +1,6 @@
        -SUBDIRS = \
        -  vimdot \
        -  core \
        -  layout \
        -  neato_layout \
        -  fdp_layout \
        -  sfdp_layout \
        -  circo_layout \
        -  tred_layout
        +SUBDIRS = \
        +  core \
        +  layout \
        +  neato_layout \
        +  fdp_layout \
        +  sfdp_layout \
        +  circo_layout \
        +  tred_layout
      '';

        static-graphviz = pkgs.graphviz.overrideAttrs (old: {
          pname = "static-graphviz-libs-only";

          patches = (old.patches or []) ++ [
            remove-vimdot-plugin-makefile-am
            remove-vimdot-plugin-makefile-in
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
            "--disable-tools"           # <-- key: disable all tools
            "--disable-vim"             # <-- disables vim and vimdot
          ];

          # Skip building tests and install checks
          doCheck = false;
          doInstallCheck = false;

          # Optional: prevent building tools if extra flags needed
          postConfigure = ''
            # Remove tools from Makefile or disable their build if needed
            sed -i '/^SUBDIRS =.*vimdot/d' plugin/Makefile.am || true
          '';

          # Ensure nativeBuildInputs & buildInputs are preserved
          buildInputs = old.buildInputs or [];
          nativeBuildInputs = old.nativeBuildInputs or [];
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