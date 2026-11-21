# RustOS – College Project Demonstration Guide

## 1. Introduction

RustOS is an educational operating system kernel project developed using the Rust programming language.

The project explores important operating system concepts, including kernel architecture, memory management, CPU scheduling, interrupts, networking, and command-line interfaces.

An interactive browser-based demonstration is also provided to showcase sample RustOS commands and outputs.

## 2. Browser-Based Demonstration

**Demo Link:** https://huggingface.co/spaces/srinithikannan/RustOS

### Steps to Demonstrate

1. Open the RustOS demonstration link in a web browser.
2. Wait for the terminal interface to load.
3. Enter a command in the terminal input field.
4. Click the RUN button or press Enter.
5. Observe the displayed output.

### 3. Important Commands to Demonstrate

| Command | Explanation |
|---|---|
| `help` | Displays the available commands. |
| `about` | Displays information about RustOS. |
| `version` | Shows the project version. |
| `memory` | Displays sample memory information. |
| `cpuinfo` | Displays processor information. |
| `sched` | Demonstrates CPU scheduling information. |
| `sched --all` | Shows scheduling algorithm comparisons. |
| `boottime` | Displays sample boot-time information. |
| `wifi` | Displays network interface information. |
| `ifconfig` | Shows network configuration information. |
| `pci` | Displays PCI device information. |
| `apt list` | Displays sample package information. |
| `game` | Displays the game menu. |
| `bench` | Displays sample benchmark information. |
| `ls` | Displays a sample file listing. |
| `date` | Displays the date. |
| `clear` | Clears the terminal screen. |

**Note:** The browser-based demonstration displays simulated command responses. It does not boot or execute the actual RustOS kernel.

## 4. Operating System Concepts Demonstrated

The project explores the following concepts:

1. Kernel architecture and system initialization.
2. Memory management and allocation.
3. CPU scheduling algorithms.
4. Interrupt handling.
5. Networking and PCI device detection.
6. Package management concepts.
7. Command-line interface design.
8. Boot-time analysis.

## 5. CPU Scheduling Algorithms

The project includes educational implementations of four major CPU scheduling algorithms:

- First Come First Served (FCFS)
- Shortest Job First (SJF)
- Priority Scheduling
- Round Robin (RR)

These algorithms are studied using waiting time and turnaround time.

## 6. Conclusion

RustOS demonstrates the fundamentals of operating system development using Rust.

The project combines kernel source code, operating system concepts, technical documentation, and an interactive browser-based demonstration.

It provides practical learning about how operating systems manage memory, handle processes, and organize system operations.
