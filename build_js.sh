#!/bin/bash

# Windows
cross build --release --target x86_64-pc-windows-gnu && \
cp ./target/x86_64-pc-windows-gnu/release/tagger.exe ./mjs/tagger-win64/ && \
echo "Binary for Windows (x64) created" && \

# Linux
cargo build --release && \
cp ./target/release/tagger ./mjs/tagger-linux64/ && \
echo "Binary for Linux (x64) created" && \

# OSX
# TODO


# README
cp README.md ./mjs/ && \
echo "README file copied"

