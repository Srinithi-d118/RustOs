# RustOS – Hardware Compatibility and Laptop Audit Report

## 1. Target Hardware Configuration

RustOS was developed and studied using the following laptop configuration.

| Hardware Component | Specification |
|---|---|
| Laptop Model | HP Laptop 15s-du3xxx |
| Processor | Intel Core i3-1125G4 |
| Processor Generation | 11th Generation Intel |
| CPU Cores | 4 Cores, 8 Threads |
| Installed RAM | 8 GB DDR4 |
| Integrated Graphics | Intel UHD Graphics |
| Ethernet Adapter | Realtek PCIe GbE Family Controller |
| Wireless Adapter | Realtek RTL8822CE 802.11ac |
| Development Environment | Ubuntu through WSL2 |

## 2. Processor Architecture

**Processor:** Intel Core i3-1125G4

The processor supports the x86-64 architecture, which is the target architecture used for RustOS development.

The project explores 64-bit operating system concepts, including kernel initialization, memory management, interrupts, and CPU timing measurements.

## 3. Ethernet Compatibility

**Network Adapter:** Realtek PCIe GbE Family Controller.

RustOS includes an educational networking module that explores PCI device identification and Ethernet driver concepts.

The compatibility of a particular physical network controller depends on its device ID, register layout, driver implementation, and hardware testing.

Full physical-device compatibility should not be assumed without successful testing.

## 4. Wi-Fi Compatibility

**Wireless Adapter:** Realtek RTL8822CE 802.11ac.

Supporting Wi-Fi directly in a small educational operating system is complex because wireless adapters require firmware, device-specific drivers, authentication, and network protocols.

RustOS documents these challenges as part of its hardware compatibility study.

The project does not claim complete native Wi-Fi support.

## 5. BIOS and UEFI Compatibility

Modern computers commonly use UEFI firmware, while some systems also support legacy BIOS booting.

A bootable operating system must use a bootloader and disk image format compatible with the target firmware.

RustOS explores the boot process and boot-image generation. Compatibility with individual BIOS and UEFI systems requires separate
