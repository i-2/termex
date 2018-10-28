#!/bin/bash

PLAT=$(uname)
BINNAME=""

function download_and_extract(){
    plat=$1
    mkdir ${HOME}/.termex
    cd ${HOME}/.termex
    url=$(curl -s https://api.github.com/repos/i-2/termex/releases/latest | jq -r .assets[].browser_download_url | grep $plat)
    wget -io $url
    fname=`echo ${url} | cut -d / -f 9`
    echo "File name: $fname"
    tar -xvf $fname
    echo -e '\033[0;32mPlease copy below lines into the .bash_profile or .bash_rc\033[0m'
    echo -e '\033[0;32mexport PATH="\${PATH}:\${HOME}/.termex"\033[0m'
    exit 0
}

case $PLAT in
   Darwin)
     echo -e "\033[0;32mDownloading osx binary\033[0m"
     download_and_extract osx
     ;;
   linux)
     echo -e "\033[0;32mDownloading linux binary\033[0m"
     download_and_extract linux
     ;;
    *)
     echo -e "\033[0;31mCannot find the binary\033[0m"
     exit 1
     ;;
esac