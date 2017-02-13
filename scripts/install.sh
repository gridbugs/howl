#!/bin/bash

set -e

if [[ "$TRAVIS_OS_NAME" == "linux" ]]; then

    sudo apt-get update -yq
    sudo apt-get install -yq libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev

elif [[ "$TRAVIS_OS_NAME" == "osx" ]]; then

    if ! which wget; then
        brew update
        brew install wget
    fi

fi
