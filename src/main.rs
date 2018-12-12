#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;

// MAGIC
const RCC_BORDER: usize = 0x4002_1000;
const RCC_APB2ENR: *mut usize = (RCC_BORDER + 0x18usize) as *mut usize;
const GPIOB_BORDER: usize = 0x4001_0C00usize;
const GPIOB_CRH: *mut usize = (GPIOB_BORDER + 0x04usize) as *mut usize;
const GPIOB_ODR: *mut usize = (GPIOB_BORDER + 0x0Cusize) as *mut usize;

#[entry]
fn main() -> ! {
    use core::ptr::write_volatile;

    unsafe {
        write_volatile(RCC_APB2ENR, *RCC_APB2ENR | (1 << 3));
        write_volatile(GPIOB_CRH, *GPIOB_CRH ^ (0b0110 << ((12 - 8) * 4)));
    }

    loop {
        for i in 0..10000 {
            if i == 0 {
                unsafe {
                    write_volatile(GPIOB_ODR, *GPIOB_ODR ^ 1 << 12);
                }
            }
        }
    }
}
