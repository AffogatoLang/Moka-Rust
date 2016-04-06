set -ex

# NOTE Workaround for rust-lang/rust#31907 - disable doc tests when cross compiling
# This has been fixed in the nightly channel but it would take a while to reach the other channels
disable_cross_doctests() {
  local host

  case "$TRAVIS_OS_NAME" in
    linux)
      host=x86_64-unknown-linux-gnu
      ;;
    osx)
      host=x86_64-apple-darwin
      ;;
  esac

  if [ "$host" != "$TARGET" ] && [ "$CHANNEL" != "nightly" ]; then
    if [ "$TRAVIS_OS_NAME" = "osx" ]; then
      brew install gnu-sed --default-names
    fi

    find src -name '*.rs' -type f | xargs sed -i -e 's:\(//.\s*```\):\1 ignore,:g'
  fi
}
## TODO modify maybe
run_test_suite() {
  case $TARGET in
    arm*-unknown-linux-gnueabihf)
      export QEMU_LD_PREFIX=/usr/arm-linux-gnueabihf
      ;;
    *)
      ;;
  esac

  if [ ! -z "$QEMU_LD_PREFIX" ]; then
    export RUST_TEST_THREADS=1
  fi

  cargo build --target $TARGET --verbose
  cargo run --target $TARGET
  cargo test --target $TARGET

  file target/$TARGET/debug/moka
}

main() {
  disable_cross_doctests
  run_test_suite
}

main
