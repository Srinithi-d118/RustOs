# RustOS Compilation Pipeline & Bootable ISO Creation

## 1. From Rust Source Code to Bootable Media
Creating an operating system from source code involves a multi-stage compilation and linking pipeline:

```
+-------------------------------------------------------------------------+
| [1] Rust Source Code (*.rs) + Cargo.toml + x86_64-blog_os.json          |
+-------------------------------------------------------------------------+
                                   |
                                   v  (rustc with build-std: core, alloc)
+-------------------------------------------------------------------------+
| [2] Kernel ELF Binary (target/x86_64-blog_os/debug/rust_os)             |
|     - Architecture: 64-bit Long Mode ELF                                |
|     - Redzone disabled (-mmx, -sse, +soft-float)                        |
+-------------------------------------------------------------------------+
                                   |
                                   v  (cargo bootimage)
+-------------------------------------------------------------------------+
| [3] Bootimage Disk Binary (bootimage-rust_os.bin)                       |
|     - Prepends 16-bit Real Mode MBR (Stage 1)                           |
|     - Prepends 32-bit Protected Mode Loader (Stage 2)                   |
|     - Appends Kernel 64-bit ELF Payload                                 |
+-------------------------------------------------------------------------+
                                   |
                                   v  (xorriso -as mkisofs)
+-------------------------------------------------------------------------+
| [4] Hybrid Bootable ISO Image (target/rust_os.iso)                      |
|     - El Torito Boot Record & Volume Header                             |
|     - Bootable on CD-ROM / Optical Media & USB Drives                   |
+-------------------------------------------------------------------------+
```

---

## 2. Bootloader Architecture Details
1. **Target Specification (`x86_64-blog_os.json`)**:
   RustOS specifies `"os": "none"` and `"panic-strategy": "abort"`, compiling a freestanding kernel without standard OS runtime dependencies.
2. **Bootloader Crate (`bootloader 0.9`)**:
   - The bootloader executes in 16-bit Real Mode upon CPU reset.
   - Sets up a flat Global Descriptor Table and switches the CPU to 32-bit Protected Mode.
   - Configures 4-level paging and switches the CPU into 64-bit Long Mode (`EFER.LME`).
   - Parses the kernel ELF header, copies sections to their virtual addresses, creates a `BootInfo` struct with physical memory map and frame information, and jumps to `kernel_main`.

---

## 3. ISO Creation with `xorriso`
The script `tools/build_iso.sh` automates ISO packaging:
```bash
xorriso -as mkisofs \
    -hard-disk-boot \
    -b boot/bootimage.bin \
    -V "RUSTOS_BOOT" \
    -o target/rust_os.iso \
    target/iso
```

### Deploying to Real Hardware (USB Flash Drive):
To boot on physical hardware (e.g. HP Laptop 15s-du3xxx):
- **On Linux**:
  ```bash
  sudo dd if=target/x86_64-blog_os/debug/bootimage-rust_os.bin of=/dev/sdX bs=4M status=progress
  ```
- **On Windows**:
  Use **Rufus**, select `bootimage-rust_os.bin` or `rust_os.iso`, choose **"DD Image Mode"**, and write to USB flash drive.