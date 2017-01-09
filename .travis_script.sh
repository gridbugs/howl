#!/bin/bash

set -e

APP_NAME=howl
SUFFIX=latest
RESOURCES=resources
UPLOADS=uploads

function build {
    target=$1
    os=$2
    machine=$3

    cargo test --release --verbose --target=$target
    cargo build --release --verbose --target=$target

    full_name=$APP_NAME-$os-$machine-$SUFFIX
    mkdir -p $full_name
    cp -v target/$target/release/$APP_NAME $full_name/$full_name
    cp -rv $RESOURCES $full_name
    zip -rv $full_name.zip $full_name
    mv -v $full_name.zip $UPLOADS
}

mkdir -pv $UPLOADS

if [[ "$TRAVIS_OS_NAME" == "linux" ]]; then

    build x86_64-unknown-linux-gnu linux x86_64

elif [[ "$TRAVIS_OS_NAME" == "osx" ]]; then

    build x86_64-apple-darwin macos x86_64

fi
