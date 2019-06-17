#!/bin/bash

# Windows
cross build --release --target x86_64-pc-windows-gnu && \
mkdir -p ./mjs/tagger-win64 && \
cp ./target/x86_64-pc-windows-gnu/release/tagger.exe ./mjs/tagger-win64/ && \
echo "Binary for Windows (x64) created" && \

# Linux
cargo build --release && \
mkdir -p ./mjs/tagger-linux64 && \
cp ./target/release/tagger ./mjs/tagger-linux64/ && \
echo "Binary for Linux (x64) created" && \

# OSX
# TODO


# README
cp README.md ./mjs/ && \
echo "README file copied" && \


# SHASUM256
cd mjs && \
rm -f SHASUM256.txt && \
sha256sum tagger-win64/tagger.exe >> SHASUM256.txt && \
sha256sum tagger-linux64/tagger >> SHASUM256.txt && \
echo "SHASUM256.txt created"
