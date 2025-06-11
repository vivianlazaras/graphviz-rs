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

        fullVersion = pkgs.llvmPackages.libclang.lib.version; # e.g. "19.1.17"
        majorVersion = 
          let
            # Match digits before the first dot
            matches = builtins.match "^(\\\\d+)" fullVersion;
          in
            if matches == null then fullVersion else matches[0];
        static-graphviz = pkgs.graphviz.overrideAttrs (old: {
          pname = "static-graphviz-libs-only";

          configureFlags = (old.configureFlags or []) ++ [
            "--enable-static"
            "--disable-shared"
            "--disable-ltdl"
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
          
          postFixup = ''
          # no-op
        '';
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
            pkgs.libclang
            pkgs.clang
            pkgs.zlib
            pkgs.libxml2
            pkgs.expat
            pkgs.llvmPackages.libclang
          ];

          shellHook = ''
            echo "Static Graphviz with libgvc and libcgraph ready for FFI."
            echo "using clang version ${pkgs.llvmPackages.libclang.lib.version}"
            export LIBCLANG_PATH=${pkgs.llvmPackages.libclang.lib}/lib
            export INCLUDE_DIR=${static-graphviz}/include/
            export PKG_CONFIG_PATH=${static-graphviz}/lib/pkgconfig
            export BINDGEN_EXTRA_CLANG_ARGS="$(pkg-config --cflags libgvc) \
              -I${pkgs.llvmPackages.libclang.lib}/lib/clang/19/include \
              -I${pkgs.glibc.dev}/include"
          '';
        };
      });
}