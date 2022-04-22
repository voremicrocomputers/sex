#!/bin/bash
uefi-run -b /usr/share/qemu/ovmf-x86_64-ms-code.bin -q /usr/bin/qemu-system-x86_64 ./target/x86_64-unknown-uefi/debug/sex.efi -- -device AC97