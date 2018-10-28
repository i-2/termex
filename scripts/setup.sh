#!/bin/bash

cp sync.sh $HOME/.termex
cp exec.sh $HOME/.termex

if [ ! -f $HOME/.bash_profile ]
then
  echo -e "\033[1;33m Creating .bash_profile \033[0m"
  touch $HOME/.bash_profile
fi

echo "source $HOME/.termex/exec.sh" >> $HOME/.bash_profile
echo "source $HOME/.termex/sync.sh" >> $HOME/.bash_profile

source $HOME/.bash_profile
