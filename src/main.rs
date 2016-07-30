#![feature(asm)]
#![feature(lang_items)]
#![no_main]
#![no_std]

use core::ptr;

#[no_mangle]
pub fn _start() {
    unsafe {
        ptr::write_volatile(0xb8000 as *mut u16, 0x0248); // H
        ptr::write_volatile(0xb8002 as *mut u16, 0x0265); // e
        ptr::write_volatile(0xb8004 as *mut u16, 0x026c); // l
        ptr::write_volatile(0xb8006 as *mut u16, 0x026c); // l
        ptr::write_volatile(0xb8008 as *mut u16, 0x026f); // o
        ptr::write_volatile(0xb800a as *mut u16, 0x022c); // ,
        ptr::write_volatile(0xb800c as *mut u16, 0x0220); //
        ptr::write_volatile(0xb800e as *mut u16, 0x0277); // w
        ptr::write_volatile(0xb8010 as *mut u16, 0x026f); // o
        ptr::write_volatile(0xb8012 as *mut u16, 0x0272); // r
        ptr::write_volatile(0xb8014 as *mut u16, 0x026c); // l
        ptr::write_volatile(0xb8016 as *mut u16, 0x0264); // d
        ptr::write_volatile(0xb8018 as *mut u16, 0x0221); // !
    }

    asm::hlt();
}

mod asm {
    pub fn hlt() {
        unsafe {
            asm!("hlt");
        }
    }
}

#[lang = "eh_personality"]
fn eh_personality() {}

#[lang = "panic_fmt"]
fn panic_fmt() {}
