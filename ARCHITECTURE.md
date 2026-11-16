# RustOS Architecture & Technical Design Document

## 1. System Overview
RustOS is an educational, bare-metal 64-bit operating system kernel developed entirely in modern Rust (`no_std`, `no_main`). Designed for the x86_64 architecture, RustOS emphasizes memory safety without garbage collection, strong modular isolation, hardware driver implementation, and process scheduling demonstrations.

```
+-------------------------------------------------------------------------+
|                  User Space / Interactive Environments                  |
|  +--------------------+  +--------------------+  +-------------------+  |
|  | RustOS Shell (CLI) |  | Space Shooter Game |  | Arcade Snake Game |  |
|  +--------------------+  +--------------------+  +-------------------+  |
|  +--------------------+  +--------------------+  +-------------------+  |
|  |  APT Package Tool  |  | SnapStore Catalog  |  | Benchmarks Suite  |  |
|  +--------------------+  +--------------------+  +-------------------+  |
+-------------------------------------------------------------------------+
|                      Kernel Subsystems & Services                       |
|  +--------------------------+  +-------------------------------------+  |
|  | CPU Scheduler (FCFS,     |  | Realtek RTL8139 Ethernet Driver     |  |
|  | SJF, Priority, RR)       |  | & PCI Configuration Manager         |  |
|  +--------------------------+  +-------------------------------------+  |
|  +--------------------------+  +-------------------------------------+  |
|  | Boot-Time Telemetry      |  | Cooperative Task Executor           |  |
|  | Profiler (TSC rdtsc)     |  | & Ringbuffer Input Queue            |  |
|  +--------------------------+  +-------------------------------------+  |
+-------------------------------------------------------------------------+
|                       Memory Management Subsystem                       |
|  +-------------------------------------------------------------------+  |
|  | 4-Level Paging (CR3) | 4 KiB Physical Frames | Linked-List Heap   |  |
|  +-------------------------------------------------------------------+  |
+-------------------------------------------------------------------------+
|                     Hardware Abstraction Layer (HAL)                    |
|  +---------------+  +---------------+  +---------------+  +----------+  |
|  | GDT & TSS IST |  | IDT & Traps   |  | Dual 8259 PIC |  | PIT (PIT)|  |
|  +---------------+  +---------------+  +---------------+  +----------+  |
|  +---------------+  +---------------+  +---------------+  +----------+  |
|  | 16550 UART    |  | PS/2 Keyboard |  | VGA Text Mode |  | PCI Bus  |  |
|  +---------------+  +---------------+  +---------------+  +----------+  |
+-------------------------------------------------------------------------+
|                        x86_64 Bare-Metal Hardware                       |
+-------------------------------------------------------------------------+
```

## 2. Boot Sequence & Execution Phases
1. **BIOS / MBR Stage**: BIOS loads the first sector into `0x7C00`. The stage 1 bootloader initializes 16-bit real mode, prepares the disk packet structure, and loads stage 2.
2. **Protected & Long Mode Switch**: Enables A20 gate, sets up preliminary page tables, loads Global Descriptor Table (GDT), activates PAE and long mode in the `EFER` model-specific register, and jumps into 64-bit Long Mode.
3. **Rust Entry Point (`kernel_main`)**:
   - `serial::init()`: Enables COM1 serial port at `0x3F8` for headless telemetry.
   - `gdt::init()`: Sets up GDT with a dedicated Interrupt Stack Table (IST) for Double Fault exception recovery.
   - `interrupts::init_idt()`: Configures 32 CPU exception vectors and hardware IRQ lines.
   - `interrupts::PICS`: Remaps the 8259 Programmable Interrupt Controller (PIC1 to vector 32, PIC2 to vector 40) to prevent conflicts with CPU exceptions.
   - `memory::init()`: Traverses the 4-level paging hierarchy and initializes the bootloader memory map.
   - `allocator::init_heap()`: Maps 256 physical 4 KiB frames to virtual address `0x4444_4444_0000` to establish a 1024 KiB dynamic kernel heap.
   - `net::init_network()`: Scans PCI configuration space, detects the Realtek RTL8139 NIC, programs DMA buffers, and enables hardware Tx/Rx.
   - `apt::init()` & `snapstore::init()`: Seeds the package management catalogue and application databases.
   - `boottime`: Evaluates cycle counters across all milestones.
   - `shell::run_shell()`: Enters the interactive CLI loop.

## 3. Memory Architecture
- **Paging Model**: 4-level recursive/offset paging (PML4 -> PDPT -> Page Directory -> Page Table -> 4 KiB Physical Frame).
- **Control Register CR3**: Holds the physical address of the active Level 4 Page Table.
- **Kernel Heap**:
  - Base Virtual Address: `0x0000_4444_4444_0000`
  - Total Size: `1024 KiB` (`1,048,576 bytes`)
  - Allocator Algorithm: Intrusive Linked List Allocator with safe Spinlock mutual exclusion. Supports dynamic Rust primitives (`Box`, `Vec`, `String`, `VecDeque`).

## 4. Interrupt & Exception Handling
- **Double Fault Handling**: Protected via IST index 0. If a kernel stack overflow occurs, the CPU automatically switches to a clean 20 KiB dedicated emergency stack, preventing triple-fault hardware reboots.
- **Timer (IRQ 0)**: Programmable Interval Timer (PIT) calibrated to ~100 Hz (10 ms period), driving `uptime_ms()` and scheduler quantum slices.
- **Keyboard (IRQ 1)**: PS/2 Keyboard controller on port `0x60`, feeding an atomic circular ringbuffer (`KeyBuffer`) decoded via `pc-keyboard`.
- **Serial (COM1)**: Monitored via Line Status Register (LSR) port `0x3FD` bit 0, providing unified serial terminal and GUI keyboard input.