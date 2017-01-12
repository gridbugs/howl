#!/bin/bash

set -e

cargo build \
    --release \
    --verbose \
    --target=$TARGET

FULL_NAME=$APP_NAME-$OS-$MACHINE-$SUFFIX
mkdir -pv $FULL_NAME

cp -rv $RESOURCES $FULL_NAME
cp -v target/$TARGET/release/$APP_NAME $FULL_NAME/$APP_NAME

zip -rv $FULL_NAME.zip $FULL_NAME

mv -v $FULL_NAME.zip $UPLOADS

rm -rf $FULL_NAME
