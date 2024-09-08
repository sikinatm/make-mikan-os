run:
	qemu-system-x86_64 -drive if=pflash,file=OVMF_CODE.fd -drive if=pflash,file=OVMF_VARS.fd -hda disk.img

build:
	clang -target x86_64-pc-win32-coff -mno-red-zone -fno-stack-protector -fshort-wchar -Wall -c hello.c
	lld-link /subsystem:efi_application /entry:EfiMain /out:hello.efi hello.o

deploy:
	hdiutil attach -mountpoint mnt disk.img
	cp hello.efi mnt/EFI/BOOT/BOOTX64.EFI
	hdiutil detach mnt