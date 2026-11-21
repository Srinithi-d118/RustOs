# RustOS – Final Project Submission and Defense Summary

## 1. Project Overview

**Project Title:** RustOS – An Educational Operating System Kernel in Rust

**Programming Language:** Rust

**Target Architecture:** x86_64

**Development Environment:** Ubuntu / Linux (WSL2)

RustOS is an educational operating system project developed using Rust. The main objective is to understand the internal working of an operating system through kernel development and implementation of fundamental OS concepts.

The project explores memory management, interrupt handling, CPU scheduling, command-line interfaces, networking concepts, and package management.

## 2. Main Features

### 2.1 Kernel Architecture
- Educational 64-bit kernel architecture.
- Global Descriptor Table (GDT).
- Interrupt Descriptor Table (IDT).
- Interrupt handling mechanisms.
- Low-level system programming using Rust.

### 2.2 Memory Management
- Paging concepts.
- Physical memory frame allocation.
- Kernel heap allocation.
- Memory management techniques.

### 2.3 CPU Scheduling

The project includes implementations of four CPU scheduling algorithms:

1. First Come First Served (FCFS)
2. Shortest Job First (SJF)
3. Priority Scheduling
4. Round Robin (RR)

These algorithms are compared using waiting time and turnaround time.

### 2.4 Command-Line Interface

RustOS includes a command-line interface that allows users to explore available commands and system-related information.

### 2.5 Package Management

The project includes educational APT and SnapStore-style package management prototypes.

These demonstrate package listing, searching, installation, removal, and application management concepts.

### 2.6 Networking Concepts

The networking module explores PCI device detection, network interfaces, and Ethernet driver development.

### 2.7 Games

The project includes two educational arcade game modules:

- Space Shooter
- Snake Classic

### 2.8 Boot-Time Analysis

A boot-time measurement module is included to study kernel initialization stages and timing information.

## 3. Technologies Used

- Rust programming language
- HTML
- CSS
- JavaScript
- Ubuntu / Linux
- Git and GitHub
- Hugging Face Spaces

## 4. Browser-Based Demonstration

An interactive browser-based demonstration is hosted on Hugging Face Spaces.

**Demo Link:** https://huggingface.co/spaces/srinithikannan/RustOS

The demonstration provides a terminal-style interface where users can enter commands and view sample outputs.

**Important:** The browser demonstration is a visual simulation. It does not directly boot or execute the actual RustOS kernel.

## 5. Building the Project

### Prerequisites

- Rust Nightly toolchain
- rust-src
- llvm-tools-preview
- bootimage
- Ubuntu or a suitable Linux environment

### Install Required Components

```bash
rustup default nightly
rustup component add rust-src llvm-tools-preview
cargo install bootimage
```

### Build the Kernel

```bash
cargo build
```

### Generate the Bootable Image

```bash
cargo bootimage
```

The generated output depends on the project configuration.

## 6. Important Operating System Concepts

The project helps demonstrate and understand:

- Kernel architecture
- Memory allocation and paging
- Interrupt handling
- CPU scheduling
- Process management concepts
- Networking concepts
- Package management
- Command-line interfaces
- System initialization and boot-time analysis

## 7. Possible Viva Questions and Answers

### Q1. Why did you choose Rust for operating system development?

Rust provides memory safety features and helps prevent common programming errors. It also provides low-level control, making it suitable for systems programming.

### Q2. What is a kernel?

A kernel is the core part of an operating system. It manages hardware resources and provides essential services to programs.

### Q3. What is memory management?

Memory management is the process of allocating, tracking, and managing computer memory.

### Q4. What is CPU scheduling?

CPU scheduling is the method used by an operating system to decide which process gets CPU time.

### Q5. What is FCFS scheduling?

First Come First Served is a scheduling algorithm in which the process that arrives first gets executed first.

### Q6. What is Round Robin scheduling?

Round Robin gives each process a fixed amount of CPU time called a time quantum.

### Q7. What is an interrupt?

An interrupt is a signal that temporarily stops the current execution so the processor can handle an event that needs attention.

### Q8. What is the purpose of a command-line interface?

A command-line interface allows users to interact with a system by entering text commands.

### Q9. What is the purpose of the Hugging Face demonstration?

It provides a browser-based terminal interface to showcase sample RustOS commands and their outputs. It is a visual simulation, not actual kernel execution.

## 8. Conclusion

RustOS is an educational operating system project developed to explore the fundamentals of operating system design using Rust.

It combines kernel source code, memory management, CPU scheduling, interrupt handling, networking concepts, package management prototypes, technical documentation, and a browser-based demonstration.

The project provides practical learning about operating system architecture and the role of different components in managing computer resources.
