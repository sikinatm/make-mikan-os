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
use uefi::table::runtime::ResetType;
use linked_list_allocator::LockedHeap;
use core::panic::PanicInfo;

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
fn efi_main(_image: Handle, mut st: SystemTable<Boot>) -> Status {
    // logging, memory allocationの初期化
    uefi::helpers::init().unwrap();

    st.stdout().reset(false).unwrap();

    // log::info!("Hello, World!"); とロギングもできる
    writeln!(st.stdout(), "Hello, World! With Rust!").unwrap();

    st.boot_services().stall(3_000_000);

    st.stdout().reset(false).unwrap();

    st.runtime_services()
        .reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // ここでエラーメッセージを表示するか、無限ループ
    loop {}
}