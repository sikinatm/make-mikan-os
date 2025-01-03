boot-loader:
	cd boot-loader && cargo build -Zbuild-std -Zbuild-std-features=compiler-builtins-mem --target x86_64-unknown-uefi

kernel:
	cd tools && cargo run -q -- shnm8x16a.bdf ../font.bin
	mv font.bin kernel/font.bin
	cd kernel && cargo build

run:
	hdiutil attach -mountpoint mnt disk.img
	cp boot-loader/target/x86_64-unknown-uefi/debug/make_mikan_os_boot_loader.efi mnt/EFI/BOOT/BOOTX64.EFI
	cp kernel/target/x86_64-unknown-none/debug/make_mikan_os_kernel mnt/kernel.elf
	hdiutil detach mnt
	qemu-system-x86_64 \
	    -m 1G -drive if=pflash,format=raw,readonly,file=OVMF_CODE.fd \
	    -drive if=pflash,format=raw,file=OVMF_VARS.fd \
	    -drive if=ide,index=0,media=disk,format=raw,file=disk.img \
	    -device nec-usb-xhci,id=xhci \
	    -device usb-mouse \
	    -device usb-kbd \
	    -monitor stdio
