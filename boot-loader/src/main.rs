#![no_std]
#![no_main]

// まだ未使用
// #[macro_use]
extern crate alloc;
extern crate linked_list_allocator;

use core::arch::asm;
use core::fmt::{Write};
use core::panic::PanicInfo;
use linked_list_allocator::LockedHeap;
use uefi::boot::{AllocateType, MemoryType};
use uefi::data_types::PhysicalAddress;
use uefi::prelude::*;
use uefi::proto::media::file::{File, FileAttribute, FileInfo, FileMode};
use uefi::{boot, println, CStr16};
use uefi::proto::console::gop::{GraphicsOutput};
use make_mikan_os_common::frame_buffer_config::{FrameBufferConfig, PixelFormat};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

type EntryPointType = extern "sysv64" fn(frame_buffer_config: FrameBufferConfig);

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

    println!("Loading kernel file...");
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
    println!("Complete load kernel file...");

    let mut file_info_buffer = [0u8; 512];

    let file_info: &FileInfo = kernel_file.get_info(&mut file_info_buffer)
        .expect("Failed to get kernel file info");

    let kernel_file_size = file_info.file_size() as usize;

    let kernel_base_addr = 0x100000;

    println!("Start to allocate page...");
    // メモリ確保
    bs.allocate_pages(
        AllocateType::Address(kernel_base_addr as PhysicalAddress),
        MemoryType::LOADER_DATA,
        // ページサイズに値を切り上げるために0xFFFを足す
        (kernel_file_size + 0xFFF) / UEFI_PAGE_SIZE,
    ).expect("Failed to allocate pages for kernel.");

    println!("Start to allocate kernel file...");

    // プログラムをメモリに展開
    kernel_file.read(unsafe { core::slice::from_raw_parts_mut(kernel_base_addr as *mut u8, kernel_file_size) })
        .expect("Failed to read kernel into memory");

    println!("Start to set up GOP handle...");
    // GOP
    let gop_handle = match boot::get_handle_for_protocol::<GraphicsOutput>() {
        Ok(result) => result,
        _ => {
            writeln!(st.stdout(), "Failed to get graphics handle.").unwrap();
            halt();
        }
    };
    println!("Start to set up GOP...");
    let mut gop = match boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle) {
        Ok(result) => result,
        _ => {
            writeln!(st.stdout(), "Failed to get graphics output").unwrap();
            halt();
        }
    };
    println!("Start to detect Pixel Format...");
    let pixel_format: PixelFormat = match gop.current_mode_info().pixel_format() {
        uefi::proto::console::gop::PixelFormat::Rgb => PixelFormat::PixelRGBResv8BitPerColor,
        uefi::proto::console::gop::PixelFormat::Bgr => PixelFormat::PixelBGRResv8BitPerColor,
        _ => {
            writeln!(st.stdout(), "Unsupported pixel format").unwrap();
            halt();
        }
    };
    println!("Start to exit boot service...");
    let config = FrameBufferConfig {
        frame_buffer: gop.frame_buffer().as_mut_ptr() as u64,
        frame_buffer_size: gop.frame_buffer().size() as u64,
        horizontal_resolution: gop.current_mode_info().resolution().0 as u64,
        vertical_resolution: gop.current_mode_info().resolution().1 as u64,
        pixel_format,
    };

    unsafe {
        let _ = st.exit_boot_services(MemoryType::LOADER_DATA);
    }

    // 64ビット用のELFファイルのエントリポイントのアドレスは、オフセット24バイトの位置から8バイト整数で書かれている
    let entry_addr_offset = (kernel_base_addr + 24) as *mut u64;
    let entry_addr = unsafe { *entry_addr_offset };

    // エントリーポイントの型にキャスト
    let entry_point: EntryPointType  = unsafe { core::mem::transmute(entry_addr as *mut u8) };

    // エントリーポイント関数を呼び出し、カーネルを起動
    entry_point(config);

    println!("end");

    Status::SUCCESS
}

fn halt () -> ! {
    loop {
        unsafe { asm!("hlt") };
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // ここでエラーメッセージを表示するか、無限ループ
    loop {}
}