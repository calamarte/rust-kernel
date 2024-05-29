#!/bin/bash

cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust-os/debug/bootimage-rust-kernel.bin
