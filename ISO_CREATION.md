# RustOS – Compilation Pipeline and Bootable Image Creation

## 1. Introduction

Creating an operating system involves converting source code into a kernel binary and preparing the required boot files.

RustOS uses the Rust programming language and Cargo build system to organize and compile its kernel source code.

## 2. Compilation Pipeline

The general compilation process consists of the following stages:

### Stage 1: Rust Source Code

The project contains Rust source files such as:

- `main.rs`
- `memory.rs`
- `interrupts.rs`
- `gdt.rs`
- `scheduler.rs`
- `net.rs`
- `shell.rs`

The `Cargo.toml` file contains the project's package and build configuration.

### Stage 2: Kernel Compilation

The Rust compiler converts the source code into a compiled kernel binary.

RustOS uses low-level Rust programming features such as `no_std` and `no_main` for kernel development.

### Stage 3: Boot Image Generation

The boot-image generation process prepares the compiled kernel with the required boot components.

The project includes boot-image configuration and build-related scripts.

### Stage 4: ISO Image Preparation

An ISO image is a disk image format that can contain operating system boot files.

ISO preparation involves organizing the required boot files and creating the image using suitable tools.

## 3. Bootloader Concepts

A bootloader is a small program that starts the operating system.

Its main responsibilities include:

1. Initializing the system environment.
2. Preparing the processor for kernel execution.
3. Setting up the required memory structures.
4. Loading the kernel into memory.
5. Transferring control to the kernel.

These steps explain the general relationship between the bootloader and the operating system kernel.

## 4. Build Tools Used

The RustOS development process uses the following tools:

| Tool | Purpose |
|---|---|
| Rust | Kernel programming language |
| Cargo | Build and dependency management |
| rustc | Rust compiler |
| bootimage | Boot-image generation |
| xorriso | ISO image creation |
| Ubuntu / WSL2 | Development environment |

## 5. Build Commands

The following commands describe the general build process, subject to the project's configuration.

Install the required Rust components:

```bash
rustup default nightly
rustup component add rust-src llvm-tools-preview
```

Install bootimage:

```bash
cargo install bootimage
```

Compile the project:

```bash
cargo build
```

Generate the boot image, if the project is configured for bootimage:

```bash
cargo bootimage
```

## 6. Build and Testing Considerations

The compilation process and successful booting are separate stages.

A successful compilation confirms that the source code has passed the relevant build checks. It does not automatically confirm that the kernel can boot on every physical computer.

Actual boot compatibility depends on the bootloader, firmware, disk-image format, and hardware configuration.

## 7. Conclusion

The RustOS compilation pipeline demonstrates how Rust source code is converted into a kernel binary and how boot-image preparation works.

This process helps explain the role of compilers, build tools, bootloaders, and ISO image formats in operating system development.
