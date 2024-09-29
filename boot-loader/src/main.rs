#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]

// まだ未使用
#[macro_use]
extern crate alloc;
extern crate linked_list_allocator;

use core::fmt::{Debug, Write};
use core::panic::PanicInfo;
use linked_list_allocator::LockedHeap;
use uefi::boot::{AllocateType, MemoryType};
use uefi::data_types::PhysicalAddress;
use uefi::prelude::*;
use uefi::proto::media::file::{File, FileAttribute, FileInfo, FileMode};
use uefi::{println, CStr16};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

type EntryPointType = extern "sysv64" fn();

const UEFI_PAGE_SIZE: usize = 0x1000;

#[entry]
fn efi_main(handle: Handle, mut st: SystemTable<Boot>) -> Status {
    // logging, memory allocationの初期化
    uefi::helpers::init().unwrap();

    st.stdout().reset(false).unwrap();

    writeln!(st.stdout(), "Starting MikanOS...").unwrap();

    let bs = st.boot_services();

    let mut root_dir = bs.get_image_file_system(handle).unwrap().open_volume().unwrap();

    let mut buf = [0u16; 260];
    let kernel_path = CStr16::from_str_with_buf("\\kernel.elf", &mut buf).unwrap();

    // プログラムのファイルを読み込む
    let mut kernel_file = match root_dir.open(
        kernel_path,
        FileMode::Read,
        FileAttribute::empty(),
    ) {
        Ok(file) => file.into_regular_file().unwrap(),
        _ => {
            writeln!(st.stdout(), "Failed to open kernel file.").unwrap();
            return Status::LOAD_ERROR;
        }
    };

    let mut file_info_buffer = [0u8; 512];

    let file_info: &FileInfo = kernel_file.get_info(&mut file_info_buffer)
        .expect("Failed to get kernel file info");

    let kernel_file_size = file_info.file_size() as usize;

    let kernel_base_addr = 0x100000;

    // メモリ確保
    bs.allocate_pages(
        AllocateType::Address(kernel_base_addr as PhysicalAddress),
        MemoryType::LOADER_DATA,
        // ページサイズに値を切り上げるために0xFFFを足す
        (kernel_file_size + 0xFFF) / UEFI_PAGE_SIZE,
    ).expect("Failed to allocate pages for kernel.");

    // プログラムをメモリに展開
    kernel_file.read(unsafe { core::slice::from_raw_parts_mut(kernel_base_addr as *mut u8, kernel_file_size) })
        .expect("Failed to read kernel into memory");

    // 一旦エントリポイントのアドレスを直指定する
    let entry_addr = 0x0000000000100210;

    // エントリーポイントの型にキャスト
    let entry_point: EntryPointType  = unsafe { core::mem::transmute(entry_addr as *mut u8) };

    // エントリーポイント関数を呼び出し、カーネルを起動
    entry_point();

    println!("end");

    Status::SUCCESS
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // ここでエラーメッセージを表示するか、無限ループ
    loop {}
}