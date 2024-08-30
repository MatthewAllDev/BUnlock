#!/bin/bash

cd ..
cargo build --release
if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi
mkdir -p release
cp target/release/bunlock release/
cp tools/install.sh release/
cp tools/uninstall.sh release/

VERSION=$(grep '^version = ' Cargo.toml | sed -E 's/version = "([^"]+)"/\1/')
ARCHIVE_NAME="bunlock-v${VERSION}.tar.gz"
tar -czvf $ARCHIVE_NAME -C release .
rm -rf release

echo "Release archive created: $ARCHIVE_NAME"