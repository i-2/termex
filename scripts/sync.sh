#!/bin/bash

source exec.sh

preexec(){
    termex_sync $1
}

