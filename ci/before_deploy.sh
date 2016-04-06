set -ex

mktempd() {
  echo $(mktemp -d 2>/dev/null || mktemp -d -t tmp)
}

mk_artifacts() {
  cargo build --release
}

mk_tarball() {
  local temp_dir=$(mktempd)
  local out_dir=$(pwd)

  ls .
  ls target
  ls target/release

  cp target/release/moka $temp_dir
  cp -r target/release/resources/* $temp_dir/resources

  pushd $temp_dir

  tar czf $out_dir/${PROJECT_NAME}-${TRAVIS_TAG}-x64-linux.tar.gz *

  popd $temp_dir
  rm -r $temp_dir
}

main() {
  mk_artifacts
  mk_tarball
}

main
