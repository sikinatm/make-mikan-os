[参考](https://www.webcyou.com/?p=10931)

イメージを作成

```
$ qemu-img create -f raw disk.img 200M
$ mkfs.fat -n 'MIKAN OS' -s 8 -f 2 -R 32 disk.img

```

実行可能ファイルを修正

```
$ hdiutil attach -mountpoint mnt disk.img
$ cp hello.efi mnt/EFI/BOOT/BOOTX64.EFI
$ hdiutil detach mnt
```

qemu 起動

```
$ qemu-system-x86_64 -drive if=pflash,file=OVMF_CODE.fd -drive if=pflash,file=OVMF_VARS.fd -hda disk.img
```

cargo build -Zbuild-std -Zbuild-std-features=compiler-builtins-mem --target x86_64-unknown-uefi
../run_qemu.sh target/x86_64-unknown-uefi/bootloader.efi
