run:
	qemu-system-x86_64 -drive if=pflash,file=OVMF_CODE.fd -drive if=pflash,file=OVMF_VARS.fd -hda disk.img

boot-loader:
	cd boot-loader && cargo build -Zbuild-std -Zbuild-std-features=compiler-builtins-mem --target x86_64-unknown-uefi

deploy:
	hdiutil attach -mountpoint mnt disk.img
	cp boot-loader/target/x86_64-unknown-uefi/debug/make-mikan-os-boot-loader.efi mnt/EFI/BOOT/BOOTX64.EFI
	cp kernel/target/x86_64-unknown-none/release/make-mikan-os-kernel mnt/kernel.elf
	hdiutil detach mnt