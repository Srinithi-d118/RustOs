# RustOS College Project Demonstration Guide

This guide is designed for presentations, professor evaluations, and live technical demonstrations.

## 1. Quick Demonstration Steps

### Step 1: Boot RustOS in QEMU
Run the following command to boot the kernel with full network simulation:
```bash
qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-rust_os.bin \
    -netdev user,id=net0 \
    -device rtl8139,netdev=net0 \
    -serial stdio
```

### Step 2: Key Commands for Examiners

| Command | What to Explain to Professor / Examiner |
| :--- | :--- |
| `help` | Demonstrates the unified CLI with 17 built-in system commands. |
| `uname -a` | Proves standalone 64-bit Long Mode execution with no host OS dependencies. |
| `cpuinfo` | Shows host processor architecture detection (Intel Tiger Lake Core i3-1125G4). |
| `mem` | Explains 4-level paging (CR3 register), 4 KiB frames, and 1024 KiB dynamic heap. |
| `boottime` | Displays hardware TSC cycle telemetry and ASCII breakdown of boot milestones. |
| `sched --all` | Runs the multi-algorithm CPU benchmark (FCFS vs SJF vs Priority vs Round Robin). |
| `sched --paper` | Explains the OS research paper connection (convoy effect, starvation, dynamic aging). |
| `pci` | Enumerates the PCI bus and shows detection of the Realtek RTL8139 network card. |
| `ifconfig` | Displays network interfaces (`lo0`, `eth0`, `wlan0`), hardware MAC address, and I/O port `0xC000`. |
| `ping 192.168.1.1` | Proves live packet transmission through the physical RTL8139 I/O descriptors. |
| `wifi` | Explains Wi-Fi hardware audit and why proprietary firmware requires the WSL bridge. |
| `apt list` | Lists Debian-style package repository metadata and installed kernel packages. |
| `apt install snake-arcade` | Demonstrates dynamic package installation, dependency resolution, and shell command activation. |
| `snake` | Launches the newly installed Retro Arcade Snake Classic game! |
| `game` | Launches the built-in RustOS Space Shooter arcade game! |
| `snapstore` | Displays the graphical application store and AppArmor confinement security profiles. |
| `bench` | Runs CPU cycle benchmarks (box allocations, frame copy throughput ~3,200 MB/s). |

---

## 2. Interactive Game Controls
- **Space Shooter (`game`)**:
  - `[A]` or `[Left Arrow]`: Move ship left
  - `[D]` or `[Right Arrow]`: Move ship right
  - `[Spacebar]`: Fire laser cannon
  - `[R]`: Restart after Game Over
  - `[Q]`: Exit back to shell
- **Arcade Snake (`snake`)**:
  - `[W]`, `[A]`, `[S]`, `[D]` or Arrow Keys: Steer snake
  - `[R]`: Restart after Game Over
  - `[Q]`: Exit back to shell