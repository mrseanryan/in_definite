#!/bin/bash
set -e
cargo install cargo-release

echo "Adding cargo to the \$PATH variable"
echo export PATH="\$HOME/.cargo/bin:\$PATH" >> $HOME/.bashrc
echo "To add cargo to *this* terminal's PATH, type this:"
echo "source \$HOME/.bashrc"
