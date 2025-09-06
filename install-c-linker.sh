#!/bin/bash
set -e
echo installing gcc linker and compiler...

sudo apt-get update
sudo apt-get upgrade
sudo apt-get install build-essential
gcc -v
make -v

echo [done]
