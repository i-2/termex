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
    TERMEX_IMPORT="${BIN_DIR}termex_import"

    mkdir -p /tmp/stage/
    cp $TERMEX_CLI /tmp/stage/
    cp $TERMEX_SYNC /tmp/stage/
    cp $TERMEX_IMPORT /tmp/stage/
    # now tar the package and upload
    cd /tmp/stage
    tar -cvf ${CRATE_NAME}-${os_type}-${TRAVIS_TAG}.tar.gz *

}


main
