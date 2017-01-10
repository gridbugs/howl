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

function build_deps_linux {
    mkdir -pv $DEPS_BUILD
    pushd $DEPS_BUILD

    mkdir -pv $SDL_ROOT
    source $DIR/build_sdl.sh

    popd

    export LIBRARY_PATH=$SDL_LIB:$LIBRARY_PATH
}

function build_app {
    target=$1
    os=$2
    machine=$3

    cargo test --release --verbose --target=$target
    cargo build --release --verbose --target=$target

    full_name=$APP_NAME-$os-$machine-$SUFFIX
    mkdir -pv $full_name

    cp -rv $RESOURCES $full_name
    cp -v target/$target/release/$APP_NAME $full_name/$APP_NAME

    zip -rv $full_name.zip $full_name

    mkdir -pv $UPLOADS
    mv -v $full_name.zip $UPLOADS
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
    build_app x86_64-unknown-linux-gnu linux x86_64

elif [[ "$TRAVIS_OS_NAME" == "osx" ]]; then

    build_app x86_64-apple-darwin macos x86_64

fi
