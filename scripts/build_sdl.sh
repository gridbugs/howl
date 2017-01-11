#!/bin/bash

set -e

# build sdl2 from source
wget https://www.libsdl.org/release/SDL2-2.0.5.tar.gz
tar xzf SDL2-2.0.5.tar.gz
pushd SDL2-2.0.5
./configure --prefix=$SDL_ROOT
make
make install
popd

# build sdl2_image from source
wget https://www.libsdl.org/projects/SDL_image/release/SDL2_image-2.0.1.tar.gz
tar xzf SDL2_image-2.0.1.tar.gz
pushd SDL2_image-2.0.1
./configure --prefix=$SDL_ROOT
make
make install
popd
