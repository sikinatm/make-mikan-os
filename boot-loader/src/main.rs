#![no_std]
#![no_main]

extern crate alloc;
extern crate linked_list_allocator;

use alloc::vec;
use core::arch::asm;
use core::panic::PanicInfo;

use linked_list_allocator::LockedHeap;
use uefi::boot::{allocate_pages, exit_boot_services, get_image_file_system, image_handle, AllocateType, MemoryType};
use uefi::data_types::PhysicalAddress;
use uefi::prelude::{entry, Status};
use uefi::proto::media::file::{File, FileAttribute, FileInfo, FileMode};
use uefi::{boot, println, CStr16};
use uefi::proto::console::gop::GraphicsOutput;

use make_mikan_os_common::frame_buffer_config::{FrameBufferConfig, PixelFormat};

// elfクレートのインポート
// no_std
use elf::endian::AnyEndian;
use elf::ElfBytes;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

const HEAP_SIZE: usize = 10 * 1024 * 1024; // 1 MiB
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

// ヒープ領域を確保する
// staticでmutableな変数の参照を使わざるを得ないので、warningを抑制しておく
#[allow(static_mut_refs)]
fn init_heap() {
    unsafe {
        ALLOCATOR.lock().init(HEAP.as_mut_ptr(), HEAP_SIZE);
    }
}

type EntryPointType = extern "sysv64" fn(FrameBufferConfig);

const UEFI_PAGE_SIZE: usize = 0x1000;

#[entry]
fn efi_main() -> Status {
    init_heap();
    uefi::helpers::init().unwrap();

    println!("Starting MikanOS...");

    // kernel.elfを開く
    let handle = image_handle();
    let mut root_dir = get_image_file_system(handle).unwrap().open_volume().unwrap();
    let mut buf = [0u16; 260];
    let kernel_path = CStr16::from_str_with_buf("\\kernel.elf", &mut buf).unwrap();

    println!("Loading kernel file...");
    let mut kernel_file = match root_dir.open(kernel_path, FileMode::Read, FileAttribute::empty()) {
        Ok(file) => file.into_regular_file().unwrap(),
        _ => {
            println!("Failed to open kernel file.");
            return Status::LOAD_ERROR;
        }
    };
    println!("Complete load kernel file...");

    let mut file_info_buffer = [0u8; 512];
    let file_info: &FileInfo = kernel_file.get_info(&mut file_info_buffer)
        .expect("Failed to get kernel file info");
    let kernel_file_size = file_info.file_size() as usize;
    println!("Complete load kernel file info...");

    // kernelファイルをメモリ上に展開
    let mut kernel_buffer = vec![0u8; kernel_file_size];
    kernel_file.read(kernel_buffer.as_mut_slice())
        .expect("Failed to read kernel into memory");
    println!("Complete allocate kernel_buffer to memory...");

    // ELFファイル解析
    let file = ElfBytes::<AnyEndian>::minimal_parse(&kernel_buffer)
        .expect("Failed to parse ELF file");

    // 全プログラムヘッダ取得
    let phdrs = file.segments().expect("Failed to get section headers");
    println!("Complete parse elf file　{}", phdrs.len());

    // LOADセグメント領域を計算
    let mut min_addr = u64::MAX;
    let mut max_addr = 0u64;
    for ph in phdrs {
        if ph.p_type != elf::abi::PT_LOAD { continue; }
        println!("ph.sh_addr: {:#x}, ph.sh_size: {:#x}, ph.ph_offset: {:#x}", ph.p_vaddr, ph.p_filesz, ph.p_offset);
        let start = ph.p_vaddr;
        let end = ph.p_vaddr + ph.p_filesz;
        if start < min_addr { min_addr = start; }
        if end > max_addr { max_addr = end; }
    }

    let kernel_mem_size = (max_addr - min_addr) as usize;

    println!("Allocating memory for kernel... min_addr: {:#x}, kernel_mem_size: {:#x}", min_addr, kernel_mem_size);
    allocate_pages(
        AllocateType::Address(min_addr as PhysicalAddress),
        MemoryType::LOADER_DATA,
        (kernel_mem_size + UEFI_PAGE_SIZE - 1) / UEFI_PAGE_SIZE,
    ).expect("Failed to allocate pages for kernel.");

    // LOADセグメントコピー
    for ph in phdrs {
        if ph.p_type != elf::abi::PT_LOAD { continue; }

        let segment_addr = ph.p_vaddr as *mut u8;
        let file_offset = ph.p_offset as usize;
        let file_size = ph.p_filesz as usize;
        let mem_size = ph.p_memsz as usize;

        // ファイル領域をコピー
        unsafe {
            core::ptr::copy_nonoverlapping(
                kernel_buffer.as_ptr().add(file_offset),
                segment_addr,
                file_size
            );
        }

        // bssなど0初期化領域
        if mem_size > file_size {
            unsafe {
                core::ptr::write_bytes(segment_addr.add(file_size), 0, mem_size - file_size);
            }
        }
    }

    println!("Finished loading ELF segments.");

    // 64ビット用のELFファイルのエントリポイントのアドレスは、オフセット24バイトの位置から8バイト整数で書かれている
    let entry_addr_offset = (min_addr + 24) as *mut u64;
    let entry_addr = unsafe { *entry_addr_offset };
    let entry_point: EntryPointType = unsafe { core::mem::transmute(entry_addr as *const u8) };

    // GOP設定
    println!("Set up GOP handle...");
    let gop_handle = match boot::get_handle_for_protocol::<GraphicsOutput>() {
        Ok(result) => result,
        _ => {
            println!("Failed to get graphics handle.");
            halt();
        }
    };
    println!("Set up GOP...");
    let mut gop = match boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle) {
        Ok(result) => result,
        _ => {
            println!( "Failed to get graphics output");
            halt();
        }
    };
    println!("Detecting Pixel Format...");
    let pixel_format: PixelFormat = match gop.current_mode_info().pixel_format() {
        uefi::proto::console::gop::PixelFormat::Rgb => PixelFormat::PixelRGBResv8BitPerColor,
        uefi::proto::console::gop::PixelFormat::Bgr => PixelFormat::PixelBGRResv8BitPerColor,
        _ => {
            println!("Unsupported pixel format");
            halt();
        }
    };

    println!("Exiting boot services...");

    unsafe {
        let _ = exit_boot_services(MemoryType::LOADER_DATA);
    }

    let config = FrameBufferConfig {
        frame_buffer: gop.frame_buffer().as_mut_ptr() as u64,
        frame_buffer_size: gop.frame_buffer().size() as u64,
        horizontal_resolution: gop.current_mode_info().resolution().0 as u64,
        vertical_resolution: gop.current_mode_info().resolution().1 as u64,
        pixels_per_scan_line: gop.current_mode_info().stride() as u64,
        pixel_format,
    };

    // カーネル起動
    entry_point(config);

    println!("end");

    Status::SUCCESS
}

fn halt() -> ! {
    loop {
        unsafe { asm!("hlt") };
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}