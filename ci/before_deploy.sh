# This script takes care of building your crate and packaging it for release

set -ex

os_type="$1"

main(){

    # delete all
    TAG=$TRAVIS_TAG
    BIN_DIR="target/release/"
    # target release
    TERMEX_CLI="${BIN_DIR}termex_cli"
    TERMEX_SYNC="${BIN_DIR}termex_sync"

    mkdir -p /tmp/stage/
    cp $TERMEX_CLI /tmp/stage/
    cp $TERMEX_SYNC /tmp/stage/
    # now tar the package and upload
    tar -cvf ${CRATE_NAME}-${os_type}-${TRAVIS_TAG}.tar.gz /tmp/stage/

}


main
