# RustOS Package Management Architecture (APT & SnapStore)

## 1. Overview & Pedagogical Objective
RustOS provides two package management interfaces:
1. **APT (Advanced Package Tool)**: Demonstrates traditional Debian-style package management, dependency resolution, package indices, and command activation hooks.
2. **SnapStore / Snapd**: Demonstrates modern containerized application distribution, application ratings, multi-channel releases, and AppArmor security confinement profiles (`strict` vs `classic`).

> **Honest Engineering Note**: These subsystems are educational demonstration layers implemented on bare-metal Rust. To execute full third-party Debian `.deb` binaries or Ubuntu `.snap` packages requiring `glibc` and `systemd`, RustOS integrates with the host WSL2 / Debian Linux supporting environment via `tools/support_env.sh`.

---

## 2. APT Subsystem Design & Dependency Resolution
The APT database maintains metadata, file size, installation state, section, and dependency lists for kernel packages.

### Package Catalogue:
- `rust-core`: Core no_std kernel libraries, memory paging, and IDT exception vectors (Dependency root).
- `net-tools`: Networking utilities (`ifconfig`, `ping`, PCI scanner).
- `sys-bench`: Micro-benchmark suite (`bench` command).
- `sched-sim`: Process scheduling simulator (`sched` command).
- `rustos-shooter`: Retro arcade Space Shooter mini-game (`game` command).
- `snake-arcade`: Classic grid-based Arcade Snake game (`snake` command).
- `snap-core`: Snapd container runtime interface.

### Dynamic Shell Command Activation:
When a package is installed via `apt install <name>`, RustOS activates the associated shell command. If uninstalled via `apt remove <name>`, the command hook is deactivated:
```
rustos> snake
Command 'snake' not found, but can be installed with:
  apt install snake-arcade

rustos> apt install snake-arcade
Reading package lists... Done
Building dependency tree... Done
The following NEW package will be installed:
  snake-arcade
0 upgraded, 1 newly installed, 0 to remove.
Need to get 48 KB of archives.
Unpacking snake-arcade (1.0.0-rustos1) ...
Setting up snake-arcade ...
Activating command hooks in RustOS shell... OK
Installation of 'snake-arcade' completed successfully.

rustos> snake
[Launches Playable Arcade Snake Game]
```

---

## 3. SnapStore Architecture & Confinement
SnapStore provides application sandboxing visualization:
- **Strict Confinement**: Complete security isolation (used by games and untrusted tools).
- **Classic Confinement**: Direct system and hardware access (used by low-level profiling tools like `sys-bench`).

```
rustos> snap list
Name             Version    Rev    Tracking       Publisher          Notes     
--------------------------------------------------------------------------------
rustos-shooter   v1.2.0     14     latest/stable  RustOS Core Team   strict    
sys-bench        v2.0.1     42     latest/stable  Performance Lab    classic   
sched-sim        v2.0.0     56     latest/stable  Systems Engineering strict    
netmon           v1.4.2     70     latest/stable  NetOps             strict    
```