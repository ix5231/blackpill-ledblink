#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

// MAGIC
const RCC_BORDER: usize = 0x4002_1000;
const RCC_APB2ENR: *mut usize = (RCC_BORDER + 0x18usize) as *mut usize;
const GPIOB_BORDER: usize = 0x4001_0C00usize;
const GPIOB_CRH: *mut usize = (GPIOB_BORDER + 0x04usize) as *mut usize;
const GPIOB_BSRR: *mut usize = (GPIOB_BORDER + 0x10usize) as *mut usize;

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    use core::ptr;

    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }

    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);

    extern "Rust" {
        fn main() -> !;
    }

    main();
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[no_mangle]
fn main() -> ! {
    use core::ptr::write_volatile;

    unsafe {
        write_volatile(RCC_APB2ENR, *RCC_APB2ENR | (1 << 3));
        write_volatile(GPIOB_CRH, *GPIOB_CRH ^ (0b0110 << ((12 - 8) * 4)));
    }

    let mut led = true;
    loop {
        for i in 0..10000 {
            if i == 0 {
                unsafe {
                    if led {
                        write_volatile(GPIOB_BSRR, *GPIOB_BSRR ^ 1 << 12);
                    } else {
                        write_volatile(GPIOB_BSRR, *GPIOB_BSRR ^ 1 << (12 + 16));
                    }
                }
                led = !led;
            }
        }
    }
}
