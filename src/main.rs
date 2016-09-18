#![feature(asm)]
#![feature(lang_items)]
#![no_main]
#![no_std]

use core::ptr;

#[no_mangle]
pub unsafe fn start() {
    const NENTRIES: usize = 512;
    const HUGE_PAGE_SIZE: u64 = 2 * 1024 * 1024; // 2 MiB

    extern "C" {
        static mut p2_table: [u64; NENTRIES];
        static mut p3_table: [u64; NENTRIES];
        static mut p4_table: [u64; NENTRIES];
    }

    // Link up page tables
    p4_table[0] = &p3_table[0] as *const _ as usize as u64 | 0b11;
    p3_table[0] = &p2_table[0] as *const _ as usize as u64 | 0b11;

    for (entry, i) in p2_table.iter_mut().zip(0..) {
        *entry = i * HUGE_PAGE_SIZE | 0b10000011;
    }

    // move page table address to cr3
    asm!("mov eax, p4_table
          mov cr3, eax" :::: "intel");

    // enable PAE
    asm!("mov eax, cr4
          or eax, 1 << 5
          mov cr4, eax" :::: "intel");

    // set long mode bit
    asm!("mov ecx, 0xC0000080
          rdmsr
          or eax, 1 << 8
          wrmsr" :::: "intel");

    // enable paging
    asm!("mov eax, cr0
          or eax, 1 << 31
          or eax, 1 << 16
          mov cr0, eax" :::: "intel");

    // load the GDT (Global Descriptor Table)
    asm!("lgdt [gdt64.pointer]");

    // update selectors
    asm!("mov ax, gdt64.data.offset" :::: "intel");

    // stack selector
    asm!("mov ss, ax" :::: "intel");
    // data selector
    asm!("mov ds, ax" :::: "intel");
    // extra selector
    asm!("mov es, ax" :::: "intel");

    // FIXME Is this the right syntax for a far jump?
    asm!("jmp [long_mode_start]");

    // Because this is a syntax error
    // asm!("jmp gdt64.code:long_mode_start" :::: "intel");
    //                     ^
    // error: <inline asm>:2:16: error: unexpected token in argument list
}

#[export_name = "long_mode_start"]
pub unsafe fn main() {
    // FIXME we are in 64-bit mode now BUT this whole crate is compiled for a 32-bit target so the
    // _compiler_ won't let use 64-bit instructions.
    // asm!("mov rax, 0x2f592f412f4b2f4f" :::: "intel");
    // asm!("mov qword [0xb8000], rax" :::: "intel");

    // Note that writing that ^ as ptr::write_volatile will emit 32-bit instructions.
    ptr::write_volatile(0xb8000 as *mut u64, 0x2f592f412f4b2f4f);

    asm::hlt();
}

mod asm {
    pub fn hlt() {
        unsafe {
            asm!("hlt");
        }
    }
}

#[lang = "panic_fmt"]
fn panic_fmt() {}
