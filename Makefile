run:
	qemu-system-x86_64 -drive if=pflash,file=OVMF_CODE.fd -drive if=pflash,file=OVMF_VARS.fd -hda disk.img

build:
	cargo build -Zbuild-std -Zbuild-std-features=compiler-builtins-mem --target x86_64-unknown-uefi

deploy:
	hdiutil attach -mountpoint mnt disk.img
	cp target/x86_64-unknown-uefi/debug/make-mikan-os.efi mnt/EFI/BOOT/BOOTX64.EFI
	hdiutil detach mnt