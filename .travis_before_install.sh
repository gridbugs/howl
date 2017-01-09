#!/bin/bash

if [[ "$TRAVIS_OS_NAME" == "linux" ]]; then

    sh ~/rust-installer/rustup.sh --prefix=$HOME/rust --add-target=x86_64-unknown-linux-gnu -y --disable-sudo

    cd ~

    # build sdl2 from source
    wget https://www.libsdl.org/release/SDL2-2.0.5.tar.gz
    tar xzf SDL2-2.0.5.tar.gz
    pushd SDL2-2.0.5
    ./configure
    make
    sudo make install
    popd

    # build sdl2_image from source
    wget https://www.libsdl.org/projects/SDL_image/release/SDL2_image-2.0.1.tar.gz
    tar xzf SDL2_image-2.0.1.tar.gz
    pushd SDL2_image-2.0.1
    ./configure
    make
    sudo make install
    popd

elif [[ "$TRAVIS_OS_NAME" == "osx" ]]; then

    sh ~/rust-installer/rustup.sh --prefix=$HOME/rust --add-target=x86_64-apple-darwin -y --disable-sudo

    brew update
    brew install sdl2
    brew install sdl2_image

fi
