{

  description = "Static Graphviz build for Rust FFI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        fullVersion = pkgs.llvmPackages.libclang.lib.version;
        majorVersion =
          let matches = builtins.match "^([0-9]+)" fullVersion;
          in if matches == null then fullVersion else matches[0];

        static-graphviz = pkgs.stdenv.mkDerivation rec {
          pname = "static-graphviz-libs-only";
          version = "13.0.0";

          src = pkgs.fetchFromGitLab {
            owner = "graphviz";
            repo = "graphviz";
            rev = version;
            hash = "sha256-wDjTtI/TyrpXgN4Jk5m0Q9tCNr1lsDQ69nxMi24JWpE=";
          };

          nativeBuildInputs = [
            pkgs.autoreconfHook
            pkgs.pkg-config
            pkgs.makeWrapper
            pkgs.python3
            pkgs.bison
            pkgs.flex
          ];

          buildInputs = [
            pkgs.libpng
            pkgs.libjpeg
            pkgs.expat
            pkgs.fontconfig
            pkgs.gd
            pkgs.gts
            pkgs.pango
          ];

          preAutoreconf = ''
            ./autogen.sh
          '';

          configureFlags = [
            "--with-pic"
            "--without-x"
            "--disable-x"

            # core features
            "--enable-gvc"
            "--enable-plugin"

            # known working options
            "--enable-ast"
            "--enable-common"
            "--enable-fdpgen"
            "--enable-label"
            "--enable-mingle"
            "--enable-pack"
            "--enable-sfdpgen"
            "--enable-twopigen"
            "--enable-xdot"

            # converted from directory names
            "--enable-cdt"
            "--enable-dotgen"
            "--enable-glcomp"
            "--enable-neatogen"
            "--enable-patchwork"
            "--enable-sfio"
            "--enable-util"
            "--enable-cgraph"
            "--enable-edgepaint"
            "--enable-ortho"
            "--enable-pathplan"
            "--enable-sparse"
            "--enable-vmalloc"
            "--enable-circogen"
            "--enable-expr"
            "--enable-gvpr"
            "--enable-osage"
            "--enable-rbtree"
            "--enable-topfish"
            "--enable-vpsc"
          ];

          doCheck = false;
          doInstallCheck = false;

          postInstall = ''
            mkdir -p $out/lib
            find . -name '*.a' -exec cp -v {} $out/lib \;

            mkdir -p $out/include
            find . -name '*.h' -exec cp -v --parents {} $out/include \;
          '';

          enableParallelBuilding = true;
        };
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
            echo "Using clang version ${pkgs.llvmPackages.libclang.lib.version}"
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