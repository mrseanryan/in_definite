#!/bin/bash
set -e  # exit when any command fails

./format-code.sh
./doc.sh
./test.sh
