#!/bin/bash

set -ex

main() {
    local td=$(mktemp -d)

    mkdir -p $td/boot/grub
    cp grub.cfg $td/boot/grub/
    cp target/x86_64/release/kernel $td/boot/
    grub-mkrescue -o x86.iso $td

    rm -rf $td
}

main
