#!/bin/bash

set -e

if [[ "$TRAVIS_OS_NAME" == "linux" ]]; then

    sh ~/rust-installer/rustup.sh \
        --prefix=$HOME/rust \
        --add-target=x86_64-unknown-linux-gnu \
        -y \
        --disable-sudo

elif [[ "$TRAVIS_OS_NAME" == "osx" ]]; then

    sh ~/rust-installer/rustup.sh \
        --prefix=$HOME/rust \
        --add-target=x86_64-apple-darwin \
        -y \
        --disable-sudo

    brew update
    brew install sdl2
    brew install sdl2_image

fi
