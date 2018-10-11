#!/bin/bash

PLAT=$(uname)
BINNAME=""

function download_and_extract(){
    plat=$1
    mkdir ~/.termex
    cd ~/.termex
    url=$(curl -s https://api.github.com/repos/i-2/termex/releases/latest | jq -r .assets[].browser_download_url | grep $plat)
    wget -io $url
    fname=`echo ${url} | cut -d / -f 9`
    echo "File name: $fname"
    tar -xvf $fname
    echo "Please copy below lines into the .bash_profile or .bash_rc"
    echo "export PATH='\${PATH}:\${HOME}/.termex'"
    exit 0
}

case $PLAT in
   Darwin)
     echo "Downloading osx binary"
     download_and_extract osx
     ;;
   linux)
     echo "Downloading linux binary"
     download_and_extract linux
     ;;
    *)
     echo "Cannot find the binary"
     exit 1
     ;;
esac