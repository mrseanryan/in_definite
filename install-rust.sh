curl https://sh.rustup.rs -sSf | sh

source $HOME/.cargo/env

echo export PATH="\$HOME/.cargo/env:\$PATH" >> $HOME/.bashrc

rustup update

rustc --version
