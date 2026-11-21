# RustOS – Network Subsystem and Testing Report

## 1. Introduction

The RustOS networking subsystem is an educational implementation designed to explore how operating systems communicate with network hardware.

It covers PCI device identification, Ethernet driver architecture, network interfaces, packet transmission concepts, and loopback communication.

## 2. Network Driver Architecture

RustOS includes networking-related source code for studying the Realtek RTL8139 Fast Ethernet controller.

The networking architecture can be understood through the following components:

1. **PCI Device Discovery:** Identifies hardware devices using PCI configuration information.
2. **Ethernet Driver:** Explores communication with an Ethernet controller through hardware registers.
3. **Network Buffer:** Provides memory space for storing network data.
4. **Loopback Interface:** Demonstrates communication within the same system.
5. **Network Interface Layer:** Organizes network-related information and commands.

## 3. RTL8139 Hardware Concepts

The RTL8139 controller uses several hardware registers to manage network operations.

| Register | Purpose |
|---|---|
| CR (0x37) | Controls receiver and transmitter |
| CONFIG_1 (0x52) | Controls device power configuration |
| RBSTART (0x30) | Specifies the receive buffer address |
| TSAD0–3 | Stores transmit buffer addresses |
| TSD0–3 | Controls packet transmission |
| IMR (0x3C) | Interrupt mask register |
| RCR (0x44) | Receive configuration register |

These registers are studied as part of understanding low-level Ethernet driver development.

## 4. PCI Device Identification

PCI (Peripheral Component Interconnect) allows the operating system to identify and communicate with hardware devices.

The Realtek vendor ID is `0x10EC`, and the RTL8139 device ID is `0x8139`.

RustOS includes networking source code that explores PCI identification and hardware configuration.

Actual physical-device operation depends on the hardware and driver support available.

## 5. Network Interfaces

Operating systems commonly use different network interfaces.

### Ethernet Interface (eth0)

An Ethernet interface represents a wired network connection.

### Loopback Interface (lo0)

The loopback interface allows a system to communicate with itself.

Its commonly used IPv4 address is `127.0.0.1`.

### Wireless Interface (wlan0)

A wireless interface represents a Wi-Fi connection.

Supporting Wi-Fi requires additional firmware, device drivers, authentication, and wireless networking protocols.

## 6. Network Testing Concepts

The following concepts are relevant to evaluating a network subsystem:

- PCI device identification
- Network interface information
- Ethernet frame structure
- MAC addresses
- Receive and transmit buffers
- Loopback communication
- ICMP and ping
- Network packet transmission

These concepts provide a foundation for understanding how an operating system manages network communication.

## 7. Testing Limitations

This report describes the networking architecture and concepts explored in RustOS.

The presence of networking source code does not by itself prove successful communication with physical network hardware.

Live Ethernet transmission, packet reception, external ping connectivity, and Wi-Fi bridging require separate implementation and testing.

No QEMU-based testing is required for this university submission.

## 8. Conclusion

The RustOS networking subsystem provides an educational introduction to network hardware and operating system communication.

It explores PCI identification, Ethernet registers, network interfaces, buffers, and packet transmission concepts.

This module helps demonstrate the challenges involved in implementing networking support in a low-level operating system.
