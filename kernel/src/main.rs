#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![allow(invalid_reference_casting)]

pub extern crate alloc;
use core::panic::PanicInfo;
use alloc::*;

pub const STACK_SIZE: usize = 4 * 1024 * 1024; // 4 MiB

const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = STACK_SIZE as u64;
    config.mappings.physical_memory = Some(bootloader_api::config::Mapping::Dynamic);
    config
};


bootloader_api::entry_point!(_kernel_main, config = &CONFIG);
fn _kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! { unsafe { kernel_main(boot_info); } }

pub mod drivers;
pub mod cpu;
pub mod allocator;
pub mod memory;
pub mod psf;
pub mod stacktrace;

unsafe fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    cpu::gdt::init();
    cpu::idt::init();

    let phys_mem_offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    let mut mapper = memory::init(phys_mem_offset);
    let mut frame_allocator = memory::BootInfoFrameAllocator::init(&boot_info.memory_regions);

    allocator::init(&mut mapper, &mut frame_allocator).unwrap();

    drivers::video::init(boot_info);

    let i = drivers::video::get_framebuffer_info();
    let mut f = vec![drivers::video::Color::default(); i.width * i.height];

    for y in 0..i.height {
        for x in 0..i.width {
            f[x + y * i.width] = drivers::video::Color::new(
                (x as f32 / i.width as f32 * 255.0) as u8, (y as f32 / i.height as f32 * 255.0) as u8, 0
            )
        }
    }

    drivers::video::batch_draw(0, 0, i.width, i.height, &f);

    for i in 0x0..=0xF {
        for j in 0x0..=0xF {
            let i = i << 4 | j;
            print!("{i:04x}: {} ", char::from_u32(i).unwrap());
        }
        println!();
    }

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crate::stacktrace::print_stacktrace();
    serial_println!("{info}");
    println!("{info}");
    println!("The full info is written to serial.");
    loop {}
}
