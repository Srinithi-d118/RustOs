use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use pic8259::ChainedPics;
use spin::{Lazy, Mutex};
use core::sync::atomic::{AtomicU64, Ordering};
use crate::gdt;
use crate::println;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard = PIC_1_OFFSET + 1,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    unsafe {
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }
    idt.page_fault.set_handler_fn(page_fault_handler);
    idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);

    idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
    idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);

    idt
});

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("\n[EXCEPTION: BREAKPOINT]\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!("\n[EXCEPTION: DOUBLE FAULT (code: {})]\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("\n[EXCEPTION: GENERAL PROTECTION FAULT (code: {})]\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("\n[EXCEPTION: PAGE FAULT]");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    loop {
        x86_64::instructions::hlt();
    }
}

// Timer management
pub static TICKS: AtomicU64 = AtomicU64::new(0);

pub fn ticks() -> u64 {
    TICKS.load(Ordering::Relaxed)
}

pub fn uptime_ms() -> u64 {
    // Standard PIT frequency ~100Hz = ~10ms per tick
    ticks() * 10
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    TICKS.fetch_add(1, Ordering::Relaxed);

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

// Keyboard input buffer
const KEYBOARD_BUF_SIZE: usize = 128;
pub struct KeyBuffer {
    buffer: [u8; KEYBOARD_BUF_SIZE],
    read_pos: usize,
    write_pos: usize,
}

impl KeyBuffer {
    const fn new() -> Self {
        KeyBuffer {
            buffer: [0; KEYBOARD_BUF_SIZE],
            read_pos: 0,
            write_pos: 0,
        }
    }

    pub fn push(&mut self, scancode: u8) {
        let next_write = (self.write_pos + 1) % KEYBOARD_BUF_SIZE;
        if next_write != self.read_pos {
            self.buffer[self.write_pos] = scancode;
            self.write_pos = next_write;
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.read_pos == self.write_pos {
            None
        } else {
            let scancode = self.buffer[self.read_pos];
            self.read_pos = (self.read_pos + 1) % KEYBOARD_BUF_SIZE;
            Some(scancode)
        }
    }
}

pub static KEY_BUFFER: Mutex<KeyBuffer> = Mutex::new(KeyBuffer::new());

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    KEY_BUFFER.lock().push(scancode);

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
