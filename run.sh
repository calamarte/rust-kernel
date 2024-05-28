#!/bin/bash

target=thumbv7em-none-eabihf

echo -ne "Building $target...\n\n"
cargo build --target $target
