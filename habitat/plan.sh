pkg_name='risq'
pkg_version="0.1.0"
pkg_origin='risq'
pkg_maintainer="Justin Carter"
pkg_license=('AGPL-3.0')
pkg_deps=(core/busybox-static
          core/glibc
          core/gcc-libs
          core/openssl)
pkg_build_deps=(core/coreutils
                core/rust
                core/gcc)
pkg_bin_dirs=(bin)

do_prepare() {

  # Can be either `--release` or `--debug` to determine cargo build strategy
  build_type="--release"
  build_line "Building artifacts with \`${build_type#--}' mode"

  export rustc_target="x86_64-unknown-linux-gnu"
  build_line "Setting rustc_target=$rustc_target"

  export LD_LIBRARY_PATH=$(pkg_path_for gcc)/lib

  export OPENSSL_LIB_DIR=$(pkg_path_for openssl)/lib
  export OPENSSL_INCLUDE_DIR=$(pkg_path_for openssl)/include

  export CARGO_TARGET_DIR="$HAB_CACHE_SRC_PATH/$pkg_dirname"
  build_line "Setting CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
}

do_build() {
  cargo build ${build_type} --features "all" --target=$rustc_target --verbose
}

do_install() {
  install -v -D "$CARGO_TARGET_DIR"/$rustc_target/${build_type#--}/$pkg_name \
    "$pkg_prefix"/bin/$pkg_name
}
