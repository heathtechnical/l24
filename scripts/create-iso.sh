#!/bin/bash

# rm -rf limine
# git clone https://github.com/limine-bootloader/limine.git --branch=v8.x-binary --depth=1
# $(MAKE) -C limine

rm -rf iso_root
mkdir -p iso_root/boot
cp -v target/custom/debug/l24r iso_root/boot/
mkdir -p iso_root/boot/limine
cp -v limine.conf iso_root/boot/limine/
mkdir -p iso_root/EFI/BOOT

cp -v limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin iso_root/boot/limine/
cp -v limine/BOOTX64.EFI iso_root/EFI/BOOT/
cp -v limine/BOOTIA32.EFI iso_root/EFI/BOOT/
xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
    -no-emul-boot -boot-load-size 4 -boot-info-table \
    --efi-boot boot/limine/limine-uefi-cd.bin \
    -efi-boot-part --efi-boot-image --protective-msdos-label \
    iso_root -o l24r.iso
./limine/limine bios-install l24r.iso

qemu-system-x86_64 \
    -M q35 \
	-boot d \
    -cdrom l24r.iso \
    -monitor telnet:127.0.0.1:5555,server,nowait \
    -serial stdio \
	-m 2G
    # -s -S \