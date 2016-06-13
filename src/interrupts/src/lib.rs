#![feature(asm)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]
#![no_std]

#[macro_use]
extern crate vga;
#[macro_use]
extern crate lazy_static;
extern crate keyboard;
extern crate pic;

use keyboard::Keyboard;
use core::intrinsics;

macro_rules! make_idt_entry {
    ($name:ident, $body:expr) => {{
        fn body() {
            $body
        }

        #[naked]
        unsafe extern fn $name() {
            asm!("push rbp
                  push r15
                  push r14
                  push r13
                  push r12
                  push r11
                  push r10
                  push r9
                  push r8
                  push rsi
                  push rdi
                  push rdx
                  push rcx
                  push rbx
                  push rax

                  mov rsi, rsp
                  push rsi
                  
                  call $0

                  add rsp, 8

                  pop rax
                  pop rbx
                  pop rcx
                  pop rdx
                  pop rdi
                  pop rsi
                  pop r8
                  pop r9
                  pop r10
                  pop r11
                  pop r12
                  pop r13
                  pop r14
                  pop r15
                  pop rbp

                  iretq" :: "s"(body as fn()) :: "volatile", "intel");
            intrinsics::unreachable();
        }

        IdtEntry::new($name)
    }}
}

#[derive(Copy,Clone,Debug)]
#[repr(packed,C)]
struct IdtEntry {
    base_low: u16,
    selector: u16,
    zero: u8,
    flags: u8,
    base_mid: u16,
    base_high: u32,
    reserved: u32,
}

impl IdtEntry {
    fn new(f: unsafe extern fn()) -> IdtEntry {
        let base = f as u64;

        IdtEntry {
            base_low: base as u16,
            selector: 0x8,
            zero: 0,
            flags: 0x8e,
            base_mid: (base >> 16) as u16,
            base_high: (base >> 32) as u32,
            reserved: 0,
        }
    }
}

#[derive(Debug)]
#[repr(packed,C)]
pub struct IdtPointer {
    limit: u16,
    base: u64,
}

#[repr(packed,C)]
struct Idt {
    entries: [IdtEntry; 256],
}

impl Idt {
    fn install(&'static self) {
        let idt_pointer = IdtPointer {
            limit: core::mem::size_of::<Idt>() as u16,
            base: self as *const _ as u64,
        };

        unsafe {
            load_idt(&idt_pointer);
        }
    }

    fn set_isr(&mut self, num: u8, entry: IdtEntry) {
        self.entries[num as usize] = entry;
    }
}

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt {
            entries: [IdtEntry {
                base_low: 0,
                selector: 0,
                zero: 0,
                flags: 0,
                base_mid: 0,
                base_high: 0,
                reserved: 0,
            }; 256]
        };

        idt.set_isr(0, make_idt_entry!(isr0, {
            // do nothing for now
            send_eoi_for(0);
            enable();
        }));

        idt.set_isr(1, make_idt_entry!(isr1, {
            // do nothing for now
            send_eoi_for(1);
            enable();
        }));

        idt.set_isr(2, make_idt_entry!(isr2, {
            // do nothing for now
            send_eoi_for(2);
            enable();
        }));

        idt.set_isr(3, make_idt_entry!(isr3, {
            // do nothing for now
            send_eoi_for(3);
            enable();
        }));

        idt.set_isr(4, make_idt_entry!(isr4, {
            // do nothing for now
            send_eoi_for(4);
            enable();
        }));

        idt.set_isr(5, make_idt_entry!(isr5, {
            // do nothing for now
            send_eoi_for(5);
            enable();
        }));

        idt.set_isr(6, make_idt_entry!(isr6, {
            // do nothing for now
            send_eoi_for(6);
            enable();
        }));

        idt.set_isr(7, make_idt_entry!(isr7, {
            // do nothing for now
            send_eoi_for(7);
            enable();
        }));

        idt.set_isr(8, make_idt_entry!(isr8, {
            // do nothing for now
            send_eoi_for(8);
            enable();
        }));

        idt.set_isr(9, make_idt_entry!(isr9, {
            // do nothing for now
            send_eoi_for(9);
            enable();
        }));

        idt.set_isr(10, make_idt_entry!(isr10, {
            // do nothing for now
            send_eoi_for(10);
            enable();
        }));

        idt.set_isr(11, make_idt_entry!(isr11, {
            // do nothing for now
            send_eoi_for(11);
            enable();
        }));

        idt.set_isr(12, make_idt_entry!(isr12, {
            // do nothing for now
            send_eoi_for(12);
            enable();
        }));

        idt.set_isr(13, make_idt_entry!(isr13, {
            // do nothing for now
            send_eoi_for(13);
            enable();
        }));

        idt.set_isr(14, make_idt_entry!(isr14, {
            // do nothing for now
            send_eoi_for(14);
            enable();
        }));

        idt.set_isr(15, make_idt_entry!(isr15, {
            // do nothing for now
            send_eoi_for(15);
            enable();
        }));

        idt.set_isr(16, make_idt_entry!(isr16, {
            // do nothing for now
            send_eoi_for(16);
            enable();
        }));

        idt.set_isr(17, make_idt_entry!(isr17, {
            // do nothing for now
            send_eoi_for(17);
            enable();
        }));

        idt.set_isr(18, make_idt_entry!(isr18, {
            // do nothing for now
            send_eoi_for(18);
            enable();
        }));

        idt.set_isr(19, make_idt_entry!(isr19, {
            // do nothing for now
            send_eoi_for(19);
            enable();
        }));

        idt.set_isr(20, make_idt_entry!(isr20, {
            // do nothing for now
            send_eoi_for(20);
            enable();
        }));

        idt.set_isr(21, make_idt_entry!(isr21, {
            // do nothing for now
            send_eoi_for(21);
            enable();
        }));

        idt.set_isr(22, make_idt_entry!(isr22, {
            // do nothing for now
            send_eoi_for(22);
            enable();
        }));

        idt.set_isr(23, make_idt_entry!(isr23, {
            // do nothing for now
            send_eoi_for(23);
            enable();
        }));

        idt.set_isr(24, make_idt_entry!(isr24, {
            // do nothing for now
            send_eoi_for(24);
            enable();
        }));

        idt.set_isr(25, make_idt_entry!(isr25, {
            // do nothing for now
            send_eoi_for(25);
            enable();
        }));

        idt.set_isr(26, make_idt_entry!(isr26, {
            // do nothing for now
            send_eoi_for(26);
            enable();
        }));

        idt.set_isr(27, make_idt_entry!(isr27, {
            // do nothing for now
            send_eoi_for(27);
            enable();
        }));

        idt.set_isr(28, make_idt_entry!(isr28, {
            // do nothing for now
            send_eoi_for(28);
            enable();
        }));

        idt.set_isr(29, make_idt_entry!(isr29, {
            // do nothing for now
            send_eoi_for(29);
            enable();
        }));

        idt.set_isr(30, make_idt_entry!(isr30, {
            // do nothing for now
            send_eoi_for(30);
            enable();
        }));

        idt.set_isr(31, make_idt_entry!(isr31, {
            // do nothing for now
            send_eoi_for(31);
            enable();
        }));

        idt.set_isr(32, make_idt_entry!(isr32, {
            // timer, do nothing for now
            pic::eoi_for(32);
            enable();
        }));

        idt.set_isr(33, make_idt_entry!(isr33, {
            let scancode = unsafe { inb(0x60) };
            Keyboard.handle_keys(scancode as usize);

            send_eoi_for(33);
            enable();
        }));

        idt
    };
}

fn send_eoi_for(interrupt: isize) {
    pic::eoi_for(interrupt);
}

pub fn install() {
    IDT.install();
}

pub fn enable() {
    unsafe {
        asm!("sti" :::: "volatile");
    }
}

unsafe fn load_idt(ptr: &IdtPointer) {
    asm!("lidt $0"::"*m"(ptr)::"volatile");
}

#[no_mangle]
pub extern "C" fn interrupt_handler(interrupt_number: isize, error_code: isize) {
    match interrupt_number {
        32 => {}, // timer
        _ => panic!("interrupt {} with error code {:x}", interrupt_number, error_code),
    }

    pic::eoi_for(interrupt_number);

    enable();
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let ret : u8;
    asm!("inb $1, $0" : "={ax}"(ret) : "{dx}N"(port) : : "volatile");
    return ret;
}
