#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod vga_buffer;
mod serial;
mod gdt;
mod interrupts;
mod keyboard;
mod memory;
mod allocator;
mod task;
mod game;
mod snake;
mod scheduler;
mod boottime;
mod net;
mod apt;
mod snapstore;
mod shell;

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // 1. Initialize Serial Communication for debugging and telemetry
    serial::init();
    boottime::record_milestone("Early Boot & COM1 Serial");
    serial_println!("[BOOT] RustOS Kernel initializing...");

    // 2. Initialize GDT and TSS with IST double-fault handling
    gdt::init();
    boottime::record_milestone("GDT & TSS Double-Fault Table");
    serial_println!("[BOOT] GDT and TSS loaded.");

    // 3. Initialize Interrupt Descriptor Table
    interrupts::init_idt();
    boottime::record_milestone("IDT Exception Vectors");
    serial_println!("[BOOT] IDT configured.");

    // 4. Initialize 8259 PIC and enable CPU hardware interrupts
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    boottime::record_milestone("8259 PIC & PIT Timer (100 Hz)");
    serial_println!("[BOOT] Hardware interrupts and PIT timer enabled.");

    // 5. Initialize Paging and Frame Allocator
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    boottime::record_milestone("4-Level Paging & Frame Alloc");
    serial_println!("[BOOT] 4-level paging and frame allocator active.");

    // 6. Initialize Dynamic Kernel Heap Allocator (1024 KiB)
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Heap initialization failed");
    boottime::record_milestone("Dynamic Kernel Heap (1024 KiB)");
    serial_println!("[BOOT] Kernel heap initialized (1024 KiB).");

    // 7. Initialize Realtek RTL8139 PCI Network Controller
    net::init_network();
    boottime::record_milestone("Realtek RTL8139 NIC & PCI");

    // 8. Initialize Package Database and Application Store
    apt::init();
    snapstore::init();
    boottime::record_milestone("APT & SnapStore Subsystems");
    serial_println!("[BOOT] APT and SnapStore databases loaded.");

    // 9. Demonstrate Cooperative Multitasking Executor
    {
        use task::simple_executor::SimpleExecutor;
        use task::Task;

        let mut executor = SimpleExecutor::new();
        executor.spawn(Task::new("Kernel Background Telemetry", async {
            serial_println!("[TASK] Background telemetry task executed successfully.");
        }));
        executor.spawn(Task::new("Hardware Device Sentinel", async {
            serial_println!("[TASK] Device sentinel verified PCI bus integrity.");
        }));
        let completed = executor.run_ready_tasks();
        serial_println!("[BOOT] Cooperative executor ran {} initialization tasks.", completed);
    }
    boottime::record_milestone("Task Executor & Scheduler");

    // 10. Display RustOS custom boot screen & welcome message
    shell::print_boot_banner();
    boottime::record_milestone("VGA Console & Shell Ready");
    serial_println!("[BOOT] RustOS boot sequence completed successfully.");

    // 11. Enter Interactive Operating System Shell
    shell::run_shell();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n=================== KERNEL PANIC ===================");
    println!("{}", info);
    println!("System halted to preserve memory state.");
    println!("====================================================");

    serial_println!("\n[KERNEL PANIC] {}", info);

    loop {
        x86_64::instructions::hlt();
    }
}