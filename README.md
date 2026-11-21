# RustOS – Educational Operating System Kernel in Rust

## Project Overview

RustOS is an educational operating system project developed using the Rust programming language. It explores important Operating System concepts such as kernel architecture, memory management, interrupts, process scheduling, command-line interfaces, and basic networking.

The project is designed to demonstrate how operating system components work through a combination of Rust source code, technical documentation, and a browser-based demonstration.

## Technologies Used

- **Programming Language:** Rust
- **Kernel Development:** `no_std` and `no_main`
- **Target Architecture:** x86_64
- **Development Environment:** Linux / Ubuntu (WSL2)
- **Web Demonstration:** HTML, CSS and JavaScript
- **Version Control:** Git and GitHub

## Key Features

### 1. Kernel Architecture
- Educational 64-bit kernel structure.
- Global Descriptor Table (GDT).
- Interrupt Descriptor Table (IDT).
- Basic interrupt handling.
- Low-level system programming using Rust.

### 2. Memory Management
- Demonstration of paging concepts.
- Physical memory frame allocation.
- Kernel heap allocation.
- Memory management concepts.

### 3. Process Scheduling
Includes educational implementations of common CPU scheduling algorithms:

- First Come First Served (FCFS)
- Shortest Job First (SJF)
- Priority Scheduling
- Round Robin (RR)

Scheduling results can be compared using waiting time and turnaround time.

### 4. Command-Line Interface
RustOS includes a command-line interface for exploring available system commands and educational features.

### 5. Package Management
Includes educational APT and SnapStore-style package management prototypes to demonstrate package operations and application management concepts.

### 6. Networking Concepts
Includes code and documentation exploring PCI device detection, network interface concepts, and Ethernet hardware driver development.

### 7. Games
Includes educational arcade game modules such as:
- Space Shooter
- Snake

### 8. Boot-Time Analysis
Includes a boot-time measurement module to explore timing information and kernel initialization stages.

## Browser-Based Demonstration

An interactive browser-based demonstration is available through Hugging Face Spaces.

**Demo:** https://huggingface.co/spaces/srinithikannan/RustOS

The browser demonstration provides a terminal-style interface where users can enter commands and view sample outputs.

**Note:** The browser demonstration is a visual simulation. It does not boot or execute the actual RustOS kernel.

## Repository Structure

The repository contains the following main files:

- `main.rs` – Kernel entry point
- `allocator.rs` – Memory allocation module
- `memory.rs` – Memory management module
- `gdt.rs` – Global Descriptor Table
- `interrupts.rs` – Interrupt handling
- `keyboard.rs` – Keyboard input handling
- `scheduler.rs` – CPU scheduling algorithms
- `boottime.rs` – Boot-time measurement
- `net.rs` – Networking-related code
- `apt.rs` – Package management prototype
- `snapstore.rs` – Application management prototype
- `shell.rs` – Command-line interface
- `snake.rs` – Snake game module
- `Cargo.toml` – Rust project configuration
- `Cargo.lock` – Dependency lock file
- `README.md` – Project documentation
- `index.html` – Browser-based demonstration page

Additional documentation and supporting files are included in the repository.

## Building the Project

### Prerequisites

- Rust Nightly toolchain
- `rust-src`
- `llvm-tools-preview`
- `bootimage`
- Ubuntu or another suitable Linux development environment

### Install Required Rust Components

```bash
rustup default nightly
rustup component add rust-src llvm-tools-preview
cargo install bootimage
```

### Build the Kernel

From the project directory, run:

```bash
cargo build
```

### Create the Bootable Image

```bash
cargo bootimage
```

The exact build output depends on the project configuration.

## Project Objective

The main objective of RustOS is to understand the internal working of an operating system by exploring kernel development, memory management, CPU scheduling, interrupt handling, and other fundamental operating system concepts using Rust.

## Conclusion

RustOS is an educational operating system project that combines low-level Rust programming, operating system concepts, technical documentation, and an interactive browser-based demonstration.

It provides an opportunity to understand how different components of an operating system work together and how Rust can be used for systems programming.
