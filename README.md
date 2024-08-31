[参考](https://www.webcyou.com/?p=10931)

実行可能ファイルを修正

```
$ hdiutil attach -mountpoint mnt disk.img
$ mkdir -p mnt/EFI/BOOT
$ hdiutil detach mnt
```

qemu 起動

```
$ qemu-system-x86_64 -drive if=pflash,file=OVMF_CODE.fd -drive if=pflash,file=OVMF_VARS.fd -hda disk.img
```
