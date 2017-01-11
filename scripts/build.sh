#!/bin/bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

APP_NAME=howl
SUFFIX=latest
RESOURCES=resources
UPLOADS=uploads

DEPS_BUILD=`pwd`/deps_build
SDL_ROOT=$DEPS_BUILD/sdl_root
SDL_LIB=$SDL_ROOT/lib
MACOS_FRAMEWORKS=$DEPS_BUILD/Frameworks

mkdir -pv $UPLOADS
mkdir -pv $DEPS_BUILD

function build_deps_linux {
    pushd $DEPS_BUILD

    mkdir -pv $SDL_ROOT
    source $DIR/build_sdl_linux.sh

    popd

    export LIBRARY_PATH=$SDL_LIB:$LIBRARY_PATH
}

function build_deps_macos {
    pushd $DEPS_BUILD

    mkdir -p $MACOS_FRAMEWORKS
    source $DIR/download_sdl_macos.sh

    popd
}

function build_nix {
    TARGET=$1
    OS=$2
    MACHINE=$3

    source $DIR/build_nix.sh
}

function build_macos {
    TARGET=$1
    OS=$2
    MACHINE=$3

    source $DIR/build_macos.sh
}

if [ -z ${TRAVIS_OS_NAME+x} ]; then
    case `uname -s` in
        Linux)
            TRAVIS_OS_NAME=linux
            ;;
        Darwin)
            TRAVIS_OS_NAME=osx
            ;;
        *)
            echo "Unknown OS"
            exit 1
    esac
fi

if [[ "$TRAVIS_OS_NAME" == "linux" ]]; then

    build_deps_linux
    build_nix x86_64-unknown-linux-gnu linux x86_64

elif [[ "$TRAVIS_OS_NAME" == "osx" ]]; then

    build_deps_macos
    build_macos x86_64-apple-darwin macos x86_64

fi
