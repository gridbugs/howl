#!/bin/bash

set -e

if [[ "$TRAVIS_OS_NAME" == "linux" ]]; then

    sh ~/rust-installer/rustup.sh \
        --prefix=$HOME/rust \
        --add-target=x86_64-unknown-linux-gnu \
        -y \
        --disable-sudo

    sudo apt-get update -yq

    sudo apt-get install -yq libsdl2-dev libsdl2-image-dev


elif [[ "$TRAVIS_OS_NAME" == "osx" ]]; then

    sh ~/rust-installer/rustup.sh \
        --prefix=$HOME/rust \
        --add-target=x86_64-apple-darwin \
        -y \
        --disable-sudo

    brew update
    brew install wget

fi
