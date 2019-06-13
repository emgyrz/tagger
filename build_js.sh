#!/bin/bash

# Windows
cross build --release --target x86_64-pc-windows-gnu && \
cp ./target/x86_64-pc-windows-gnu/release/tagger.exe ./mjs/tagger-win64/ && \

# Linux
cargo build --release && \
cp ./target/release/tagger ./mjs/tagger-linux64/


# OSX
# TODO
