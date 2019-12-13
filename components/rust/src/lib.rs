#![no_std]
#![feature(lang_items, core_intrinsics, asm)]

// - panic handler ------------------------------------------------------------

use core::intrinsics;
use core::panic::PanicInfo;

#[lang = "panic_impl"]
extern fn rust_begin_panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}


// - c imports ----------------------------------------------------------------

extern "C" {
    fn write(fd: i32, data: *const u8, size: usize) -> usize;
}


// - global constants ---------------------------------------------------------

const CORE_HZ: u32 = 40_000_000;

const GPIO_ENABLE_W1TS_REG: u32 = 0x3FF44024;
const GPIO_OUT_W1TS_REG: u32 = 0x3FF44008;
const GPIO_OUT_W1TC_REG : u32 = 0x3FF4400C;
const GPIO_FUNCX_OUT_BASE: u32 = 0x3FF44530;
const GPIO_FUNCX_OUT_SEL_CFG: u32 = GPIO_FUNCX_OUT_BASE + (BLINKY_GPIO * 4);

const BLINKY_GPIO: u32 = 2;


// - entry-point --------------------------------------------------------------

#[no_mangle]
fn rust_main() -> ! {
    let buffer = "Hello Rust!\n".as_bytes();
    unsafe {
        write(0, buffer.as_ptr(), buffer.len());
    }

    configure_pin_as_output(BLINKY_GPIO);

    loop {
        set_led(BLINKY_GPIO, true);
        delay(CORE_HZ);
        set_led(BLINKY_GPIO, false);
        delay(CORE_HZ);
    }
}


// - implementation -----------------------------------------------------------

pub fn set_led(idx: u32, val: bool) {
    if val {
        unsafe {
            core::ptr::write_volatile(GPIO_OUT_W1TS_REG as *mut u32, 0x1 << idx);
        }
    } else {
       unsafe {
            core::ptr::write_volatile(GPIO_OUT_W1TC_REG as *mut u32, 0x1 << idx);
        }
    }
}



pub fn configure_pin_as_output(gpio: u32) {
    unsafe {
        core::ptr::write_volatile(GPIO_ENABLE_W1TS_REG as *mut _, 0x1 << gpio);
        core::ptr::write_volatile(GPIO_FUNCX_OUT_SEL_CFG as *mut _, 0x100);
    }
}


pub fn delay(clocks: u32) {
    let target = get_ccount() + clocks;
    loop {
        if get_ccount() > target {
            break;
        }
    }
}


pub fn get_ccount() -> u32 {
    let x: u32;
    unsafe {
        asm!("rsr.ccount a2" : "={a2}" (x))
    };
    x
}
