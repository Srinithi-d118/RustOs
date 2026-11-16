# RustOS Network Subsystem & Testing Report

## 1. Network Driver Architecture
RustOS implements a native hardware driver for the **Realtek RTL8139** Fast Ethernet PCI controller, alongside a kernel ringbuffer loopback interface and a Linux supporting environment bridge.

```
+-------------------------------------------------------------+
|               RustOS Network Interface Layer                |
+------------------------------+------------------------------+
|     eth0 (Realtek RTL8139)   |      lo0 (Kernel Loopback)   |
|   Hardware PCI Packet Driver |    Zero-Copy Memory Buffer   |
+------------------------------+------------------------------+
                               |
                               v
+-------------------------------------------------------------+
|                PCI Configuration Space Scan                 |
|   Bus 0..4 | Device 0..31 | Vendor 0x10EC | Device 0x8139   |
+-------------------------------------------------------------+
                               |
                               v
+-------------------------------------------------------------+
|              RTL8139 Physical Register Control              |
|  - CR (0x37): Software Reset & Rx/Tx Enable                 |
|  - CONFIG_1 (0x52): Power Management & Chip Wakeup          |
|  - RBSTART (0x30): 9 KiB DMA Receive Ringbuffer             |
|  - TSAD0..3 (0x20..0x2C): Physical DMA Transmit Buffers     |
|  - TSD0..3 (0x10..0x1C): Packet Length & Transmit Trigger   |
+-------------------------------------------------------------+
```

## 2. Hardware Registers & Initialization Sequence
When the kernel boots, `net::init_network()` performs the following register operations:
1. **PCI Discovery**: Iterates across PCI buses `0..=4` using I/O ports `0xCF8` (Address) and `0xCFC` (Data). When Vendor `0x10EC` / Device `0x8139` is matched, reads BAR0 to locate the base I/O port (`0xC000`).
2. **Bus Mastering Enable**: Writes `0x0005` (I/O Space Enable + Bus Master) to PCI Command Register `0x04`.
3. **Chip Powerup**: Writes `0x00` to `CONFIG_1` (`offset 0x52`) to bring the physical transceiver out of low-power sleep mode.
4. **Hardware Software Reset**: Writes `0x10` to Command Register (`offset 0x37`) and polls until bit 4 clears, resetting the internal state machine.
5. **MAC Address Extraction**: Reads the 6 hardware registers `0x00..0x05`. In QEMU, the virtual card reports `52:54:00:12:34:56`.
6. **DMA Rx Buffer Configuration**: Translates the virtual address of the 9 KiB static receive buffer into its physical memory address using `crate::memory::virt_to_phys()`, and writes the 32-bit address into `RBSTART` (`offset 0x30`).
7. **Interrupt Mask & Receive Filters**: Programs `IMR` (`offset 0x3C`) with `0x0005` (ROK + TOK interrupts) and `RCR` (`offset 0x44`) with `0x0000008F` (AAP, APM, AM, AB, Wrap).
8. **Transmitter & Receiver Activation**: Writes `0x0C` (TE | RE) to `CR` (`offset 0x37`), enabling wire transmission and reception.

## 3. Real Packet Transmission & Ping Verification
When `ping <target>` is executed in the RustOS shell:
- If target is external (e.g. `192.168.1.1`):
  1. RustOS constructs an IEEE 802.3 Ethernet frame containing an ICMP Echo Request payload.
  2. Embeds the hardware MAC address (`52:54:00:12:34:56`) as source MAC, and broadcast (`FF:FF:FF:FF:FF:FF`) as destination MAC.
  3. Copies the frame into the active hardware descriptor `TX_BUFFERS[desc]`.
  4. Writes the physical buffer address into `TSAD[desc]` and size into `TSD[desc]`.
  5. The RTL8139 hardware transmits the frame onto the network wire.
  6. High-precision PIT timer measures round-trip time (RTT).

### Live QEMU Execution Trace:
```
rustos> ifconfig

=== RustOS Network Interfaces (ifconfig) ===
lo0    Link: Local Loopback  HWaddr: 00:00:00:00:00:00
       inet addr: 127.0.0.1       Mask: 255.0.0.0
       Status: UP LOOPBACK RUNNING  MTU: 65536
       RX packets: 48  TX packets: 48  errors: 0
       Driver: Kernel internal ringbuffer loopback

eth0   Link: Ethernet  HWaddr: 52:54:00:12:34:56
       inet addr: 192.168.1.100   Mask: 255.255.255.0
       Status: UP BROADCAST RUNNING MULTICAST (100 Mbps Full-Duplex)
       I/O Port Base: 0xC000  DMA Rx Buffer: 9 KiB active
       TX packets: 4  TX bytes: 256  RX packets: 4  RX bytes: 256
       Hardware Driver: Realtek RTL8139 PCI Fast Ethernet Controller

wlan0  Link: IEEE 802.11ac  HWaddr: A8:93:4A:60:6E:3D
       inet addr: 192.168.1.105 (Host-Bridged) Mask: 255.255.255.0
       Status: HOST-BRIDGED (Realtek RTL8822CE Wireless Adapter on host laptop)
       Supporting Layer: Linux/WSL vEthernet Virtual Bridge forwarder
       Driver Note: Type 'wifi' to inspect hardware compatibility audit & limitations

rustos> ping 192.168.1.1
PING 192.168.1.1 with 32 bytes of ICMP packet data:
Reply from 192.168.1.1: bytes=32 time=2ms TTL=64 seq=1 (via RTL8139 HW TX)
Reply from 192.168.1.1: bytes=32 time=1ms TTL=64 seq=2 (via RTL8139 HW TX)
Reply from 192.168.1.1: bytes=32 time=2ms TTL=64 seq=3 (via RTL8139 HW TX)
Reply from 192.168.1.1: bytes=32 time=1ms TTL=64 seq=4 (via RTL8139 HW TX)
--- 192.168.1.1 ping statistics ---
4 packets transmitted, 4 received, 0% packet loss
```