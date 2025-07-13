{
  description = "Combined flake with Rust/Go projects and static Graphviz build";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.outputs.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Rust and Go packages

        rustToolchain = pkgs.rust-bin.stable.latest.default;

        rustPackage = pkgs.rustPlatform.buildRustPackage {
          pname = "graphviz-rs";
          version = "0.1.0";
          nativeBuildInputs = [
            staticGraphviz pkgs.pkg-config
          ];
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };

        # Static Graphviz build derivation

        fullVersion = pkgs.llvmPackages.libclang.lib.version;
        majorVersion =
          let matches = builtins.match "^([0-9]+)" fullVersion;
          in if matches == null then fullVersion else matches[0];

        staticGraphviz = pkgs.stdenv.mkDerivation rec {
          pname = "graphviz-libs-onll";
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
            pkgs.pkg-config
            pkgs.flex
          ];

          buildInputs = [
            pkgs.libpng
            pkgs.libjpeg
            pkgs.pkg-config
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

            "--enable-gvc"
            "--enable-plugin"

            "--enable-ast"
            "--enable-common"
            "--enable-fdpgen"
            "--enable-label"
            "--enable-mingle"
            "--enable-pack"
            "--enable-sfdpgen"
            "--enable-twopigen"
            "--enable-xdot"

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
        packages = {
          # Combined default package: Graphviz
          default = pkgs.symlinkJoin {
            name = "graphviz-rs";
            paths = [ rustPackage staticGraphviz ];
          };

          # Individual packages available
          static-graphviz = staticGraphviz;
        };

        devShells = {
          default = pkgs.mkShell {
            buildInputs = [
              rustToolchain
              pkgs.go
              pkgs.apacheHttpd
              pkgs.pkg-config
              pkgs.openssl
              pkgs.gorm-gentool
              staticGraphviz
              pkgs.pkg-config
              pkgs.libclang
              pkgs.clang
              pkgs.zlib
              pkgs.libxml2
              pkgs.expat
              pkgs.llvmPackages.libclang
            ];

            shellHook = ''
              echo "Rust and Go dev environment loaded with Static Graphviz support."
              echo "Using clang version ${pkgs.llvmPackages.libclang.lib.version}"
              export LIBCLANG_PATH=${pkgs.llvmPackages.libclang.lib}/lib
              export INCLUDE_DIR=${staticGraphviz}/include/
              export PKG_CONFIG_PATH=${staticGraphviz}/lib/pkgconfig
              export BINDGEN_EXTRA_CLANG_ARGS="$(pkg_config_exec --cflags libgvc) \
                -I${pkgs.llvmPackages.libclang.lib}/lib/clang/19/include \
                -I${pkgs.glibc.dev}/include"
            '';
          };
        };
      }
    );
}