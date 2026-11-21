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

### 5
