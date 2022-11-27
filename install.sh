#!/bin/sh

cargo build --release
if [[ "$OSTYPE" == "linux-gnu" ]]; then
    cp target/release/depploy /usr/local/bin/depploy
else
    cp target/release/depploy /usr/local/bin/depploy
fi