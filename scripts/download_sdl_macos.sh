#!/bin/bash

set -e

pushd $MACOS_FRAMEWORKS

wget https://www.libsdl.org/release/SDL2-2.0.5.dmg
wget https://www.libsdl.org/projects/SDL_image/release/SDL2_image-2.0.1.dmg

mkdir -p mnt

hdiutil attach SDL2-2.0.5.dmg -mountpoint mnt
cp -rv mnt/SDL2.framework .
hdiutil detach mnt

hdiutil attach SDL2_image-2.0.1.dmg -mountpoint mnt
cp -rv mnt/SDL2_image.framework .
hdiutil detach mnt

popd
