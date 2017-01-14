#!/bin/bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

APP_NAME=howl
SUFFIX=latest
RESOURCES=resources
UPLOADS=uploads

DEPS_BUILD=`pwd`/deps_build
MACOS_FRAMEWORKS=$DEPS_BUILD/Frameworks

mkdir -pv $UPLOADS
mkdir -pv $DEPS_BUILD

function build_deps_macos {
    pushd $DEPS_BUILD

    mkdir -p $MACOS_FRAMEWORKS
    source $DIR/download_sdl_macos.sh

    popd
}

function build_nix {
    OS=$1
    MACHINE=$2

    source $DIR/build_nix.sh
}

function build_macos {
    OS=$1
    MACHINE=$2

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

cargo test --release --verbose --no-default-features

if [[ "$TRAVIS_OS_NAME" == "linux" ]]; then

    build_nix linux x86_64

elif [[ "$TRAVIS_OS_NAME" == "osx" ]]; then

    build_deps_macos
    build_macos macos x86_64

fi
