# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    # TODO Update this to build the artifacts that matter to you
    cd termex_cli/
    cross rustc --bin termex_cli --target $TARGET --release -- -C lto

    cd ../termex_sync/
    cross rustc --bin termex_sync --target $TARGET --release -- -C lto

    # TODO Update this to package the right artifacts
    cd ..
    cp target/$TARGET/release/termex_cli $stage/
    cp target/$TARGET/release/termex_sync $stage

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
