# RustOS: A 64-Bit Educational Operating System Kernel in Rust

[![Rust](https://img.shields.io/badge/Language-Rust_2024_Nightly-orange.svg)](https://www.rust-lang.org/)
[![Target](https://img.shields.io/badge/Target-x86__64--unknown--none-blue.svg)](https://os.phil-opp.com/)
[![License](https://img.shields.io/badge/License-MIT%20%2F%20Apache--2.0-green.svg)](LICENSE-MIT)
[![Build Status](https://img.shields.io/badge/Build-Passing%20(Zero%20Warnings)-brightgreen.svg)]()

RustOS is a functional, educational 64-bit operating system kernel written in Rust (`no_std`, `no_main`) targeting the `x86_64` architecture. Developed as a comprehensive college operating systems project and research platform, RustOS demonstrates bare-metal hardware programming, memory management, PCI device drivers, process scheduling, package management, and retro gaming.

---

## Key Highlights & Implemented Features

1. **Bootable 64-Bit Microkernel Architecture**:
   - Boots via a 2-stage BIOS bootloader into 64-bit Long Mode.
   - Global Descriptor Table (GDT) with dedicated Interrupt Stack Table (IST) for Double Fault exception recovery.
   - Interrupt Descriptor Table (IDT) handling 32 CPU exceptions and hardware IRQs.
   - Dual 8259 PIC remapped to vectors 32–47; PIT hardware timer calibrated to ~100 Hz.
   - 4-level paging (PML4 -> PDPT -> PD -> PT) and 4 KiB physical frame allocator.
   - 1024 KiB dynamic kernel heap managed by a spinlocked intrusive linked-list allocator.
   - **Zero Compiler Warnings**: Verified build on `nightly-x86_64-unknown-linux-gnu`.

2. **Bare-Metal Ethernet Hardware Driver (Realtek RTL8139)**:
   - Scans PCI configuration space (`0xCF8`/`0xCFC`) and detects Vendor `0x10EC` / Device `0x8139`.
   - Programs PCI Bus Mastering and I/O base port (`0xC000`).
   - Reads hardware MAC address directly from card registers (`52:54:00:12:34:56`).
   - Allocates 9 KiB DMA receive ringbuffer and 4 physical transmit descriptors (`TSAD0..3`).
   - Genuinely transmits raw IEEE 802.3 Ethernet frames for live `ping` testing.

3. **Hardware Compatibility & Laptop Audit**:
   - Evaluated on host hardware: **HP Laptop 15s-du3xxx** (11th Gen Intel Core i3-1125G4 @ 2.00GHz, Realtek RTL8822CE 802.11ac Wi-Fi, Realtek PCIe GbE Family Controller, Intel UHD Graphics, 8 GB RAM).
   - Detailed technical audit explaining Wi-Fi closed-source microcode firmware dependencies and protocol complexity, paired with a Linux Supporting Environment Bridge (WSL2 / TAP virtual switch).

4. **Process Scheduling Subsystem & Research Paper Connection**:
   - Full Process Control Block (PCB) process simulator with metrics: Turnaround Time ($TAT = CT - AT$), Waiting Time ($WT = TAT - BT$), and CPU Utilization.
   - Implementations of:
     - **First-Come First-Served (FCFS)**: Demonstrates the Convoy Effect.
     - **Shortest Job First (SJF)**: Proves minimum average waiting time; demonstrates starvation.
     - **Priority Scheduling**: Features **Dynamic Aging** to guarantee bounded waiting times.
     - **Round Robin (RR)**: Features configurable time quanta ($q=2$, $q=4$) and ASCII Gantt charts.
   - Dedicated shell command `sched --paper` presenting academic evaluation matching our OS research paper.

5. **Integrated APT Package Manager & SnapStore**:
   - **APT**: `update`, `list`, `search`, `show`, `install`, `remove` with dependency resolution.
   - Installing packages activates live kernel commands: installing `snake-arcade` enables `snake`; installing `sys-bench` enables `bench`!
   - **SnapStore**: Demonstrates containerized application distribution with ratings, channels, and AppArmor security confinement profiles (`strict` vs `classic`).

6. **Playable Retro Arcade Games**:
   - **Space Shooter (`game`)**: Fast-paced VGA text-mode space defender with collision detection, multi-wave enemy scaling, laser fire, and score tracking.
   - **Arcade Snake (`snake`)**: Classic grid-based snake game with dynamic speed levels, score tracking, and food generation.

7. **Boot-Time Telemetry & Profiler**:
   - Zero-allocation profiler using CPU Time Stamp Counter (`rdtsc`) and PIT ticks.
   - Live breakdown of all 10 boot milestones with cycle counts, delta percentages, and ASCII bar charts (`boottime` command).

8. **Bootable ISO & Disk Images**:
   - Generates `target/x86_64-blog_os/debug/bootimage-rust_os.bin` for hard drives / USB flash drives.
   - Automated script `tools/build_iso.sh` producing hybrid bootable El Torito ISO images (`target/rust_os.iso`).

---

## Project Structure

```
rust_os/
├── .cargo/
│   └── config.toml               # Cargo target & build-std configuration
├── docs/
│   ├── ARCHITECTURE.md           # Deep architectural documentation
│   ├── HARDWARE_COMPATIBILITY.md # Host laptop hardware audit & Wi-Fi analysis
│   ├── NETWORK_TESTING.md        # Realtek RTL8139 hardware packet driver report
│   ├── PACKAGE_MANAGEMENT.md     # APT & SnapStore package management design
│   ├── BOOT_TIME_ANALYSIS.md     # Microsecond boot telemetry report & graph
│   ├── ISO_CREATION.md           # Source-to-binary-to-ISO compilation pipeline
│   ├── SCHEDULING_RESEARCH_PAPER.md # FCFS, SJF, Priority, RR theoretical analysis
│   ├── DEMO_GUIDE.md             # Walkthrough script for college examiner defense
│   └── FINAL_SUBMISSION.md       # Final project submission & defense summary
├── src/
│   ├── main.rs                   # Kernel entry point & boot milestone orchestrator
│   ├── vga_buffer.rs             # 80x25 VGA text buffer driver & serial mirroring
│   ├── serial.rs                 # 16550 UART COM1 serial driver (Tx & Rx)
│   ├── gdt.rs                    # Global Descriptor Table & TSS Double Fault IST
│   ├── interrupts.rs             # IDT exception vectors, 8259 PIC & PIT timer (~100 Hz)
│   ├── keyboard.rs               # PS/2 keyboard ringbuffer & unified terminal input
│   ├── memory.rs                 # 4-level paging & physical frame allocator
│   ├── allocator.rs              # 1024 KiB intrusive linked-list heap allocator
│   ├── net.rs                    # PCI scanner, Realtek RTL8139 NIC driver & Wi-Fi audit
│   ├── scheduler.rs              # CPU process scheduler (FCFS, SJF, Priority, RR)
│   ├── boottime.rs               # High-precision TSC rdtsc milestone telemetry profiler
│   ├── apt.rs                    # Advanced Package Tool (APT) engine & dependency tree
│   ├── snapstore.rs              # SnapStore application catalog & confinement model
│   ├── game.rs                   # Built-in Space Shooter arcade game
│   ├── snake.rs                  # Built-in Arcade Snake Classic game
│   ├── shell.rs                  # Interactive command-line shell (17 commands)
│   └── task/                     # Cooperative async executor & Waker primitives
├── tools/
│   ├── build_iso.sh              # Automated bootable El Torito ISO generator
│   ├── test_qemu.py              # Automated headless QEMU integration test suite
│   └── support_env.sh            # WSL2 / Linux supporting environment bridge
├── Cargo.toml                    # Package dependencies & kernel metadata
├── x86_64-blog_os.json           # Freestanding x86_64 target specification
└── README.md                     # Master documentation
```

---

## Building and Running

### Prerequisites
- **Rust Nightly**: with `rust-src` and `llvm-tools-preview`
  ```bash
  rustup default nightly
  rustup component add rust-src llvm-tools-preview
  cargo install bootimage
  ```
- **QEMU**: `qemu-system-x86_64`
- **xorriso**: For ISO generation (`sudo apt install xorriso`)

### Build the Kernel
```bash
cargo build
```

### Build the Bootable Disk Image
```bash
cargo bootimage
# Output binary: target/x86_64-blog_os/debug/bootimage-rust_os.bin
```

### Build the Bootable ISO Image
```bash
chmod +x tools/build_iso.sh
./tools/build_iso.sh
# Output ISO: target/rust_os.iso
```

### Run in QEMU
```bash
# Graphical display with Realtek RTL8139 NIC:
qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-rust_os.bin \
    -netdev user,id=net0 \
    -device rtl8139,netdev=net0 \
    -serial stdio

# Headless / Terminal Mode:
qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-rust_os.bin \
    -netdev user,id=net0 \
    -device rtl8139,netdev=net0 \
    -serial stdio -display none
```

---

## Interactive Shell Commands

| Command | Description |
| :--- | :--- |
| `help` | Display list of all 17 available shell commands |
| `uname [-a]` | Display OS name, kernel build, and target architecture |
| `cpuinfo` | Display processor model, logical cores, and execution flags |
| `mem` | Display physical memory detected, 4-level paging, and heap allocation |
| `uptime` | Display elapsed system uptime in minutes, seconds, and PIT ticks |
| `boottime` | Display kernel boot milestone timeline, delta cycles, and ASCII graph |
| `sched [options]` | Run process scheduler benchmark (`--all`, `--fcfs`, `--sjf`, `--priority`, `--rr`, `--paper`) |
| `game` | Launch built-in RustOS Space Shooter arcade game |
| `snake` | Launch Retro Arcade Snake Classic game (managed via APT) |
| `pci` | Scan PCI configuration space and list detected controllers |
| `ifconfig` / `net` | Display network interfaces (`lo0`, `eth0`, `wlan0`), MAC addresses, and packet counters |
| `ping <target>` | Transmit ICMP packets (via live Realtek RTL8139 hardware descriptors) |
| `wifi` | Display host Wi-Fi hardware compatibility audit & driver limitations |
| `apt <subcmd>` | Advanced Package Tool (`update`, `list`, `install`, `remove`, `show`, `search`) |
| `snap <subcmd>` | Snap application manager (`list`, `find`, `install`, `remove`, `info`) |
| `snapstore` | Launch interactive SnapStore Application Center with confinement badges |
| `bench` | Run kernel performance micro-benchmarks (heap, throughput, frame copy, PIT jitter) |
| `reboot` | Issue soft reset to keyboard system controller |