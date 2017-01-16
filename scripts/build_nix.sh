#!/bin/bash

set -e

cargo build \
    --release \
    --verbose

FULL_NAME=$APP_NAME-$OS-$MACHINE-$SUFFIX

BRANCH=`git rev-parse --abbrev-ref HEAD`
if [ "$BRANCH" != "master" ]; then
    FULL_NAME="$FULL_NAME-$BRANCH"
fi

mkdir -pv $FULL_NAME

cp -rv $RESOURCES $FULL_NAME
cp -v target/release/$APP_NAME $FULL_NAME/$APP_NAME

$FULL_NAME/$APP_NAME --help

zip -rv $FULL_NAME.zip $FULL_NAME

mv -v $FULL_NAME.zip $UPLOADS

rm -rf $FULL_NAME
