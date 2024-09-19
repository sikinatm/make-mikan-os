#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]

extern crate linked_list_allocator;

// まだ未使用
#[macro_use]
extern crate alloc;

use core::fmt::Write;
use uefi::prelude::*;
use linked_list_allocator::LockedHeap;
use core::panic::PanicInfo;
use uefi::boot::{AllocateType, MemoryType};
use uefi::CStr16;
use uefi::data_types::PhysicalAddress;
use uefi::proto::media::file::{FileAttribute, FileHandle, FileInfo, FileMode, FileType, File};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

fn init_heap() {
    // ヒープ領域を初期化する必要があります
    // 例えば、アドレスとサイズを指定して設定
    let heap_start = 0x1000;
    let heap_size = 1024 * 1024; // 1MB
    unsafe {
        ALLOCATOR.lock().init(heap_start, heap_size);
    }
}

#[entry]
fn efi_main(handle: Handle, mut st: SystemTable<Boot>) -> Status {
    // logging, memory allocationの初期化
    uefi::helpers::init().unwrap();

    st.stdout().reset(false).unwrap();

    writeln!(st.stdout(), "Starting Mikan OS...").unwrap();

    let bs = st.boot_services();

    let mut root_dir = bs.get_image_file_system(handle).unwrap().open_volume().unwrap();

    let mut buf = [0u16; 260];
    let kernel_path = CStr16::from_str_with_buf("\\kernel.elf", &mut buf).unwrap();

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

    let kernel_base_addr = 0x100000 as *mut u8;
    bs.allocate_pages(
        AllocateType::Address(kernel_base_addr as PhysicalAddress),
        MemoryType::LOADER_DATA,
        (kernel_file_size + 0xFFF) / 0x1000,
    ).expect("Failed to allocate pages for kernel.");

    kernel_file.read(unsafe { core::slice::from_raw_parts_mut(kernel_base_addr, kernel_file_size) })
        .expect("Failed to read kernel into memory");

    Status::SUCCESS
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // ここでエラーメッセージを表示するか、無限ループ
    loop {}
}