import subprocess
import time
import sys

cmd = [
    "qemu-system-x86_64",
    "-drive", "format=raw,file=/root/rust_os/rust_os/target/x86_64-blog_os/debug/bootimage-rust_os.bin",
    "-netdev", "user,id=net0",
    "-device", "rtl8139,netdev=net0",
    "-serial", "stdio",
    "-display", "none"
]

proc = subprocess.Popen(cmd, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)

time.sleep(1.0)

test_commands = [
    "help\n",
    "uname -a\n",
    "cpuinfo\n",
    "mem\n",
    "uptime\n",
    "boottime\n",
    "sched --all\n",
    "sched --fcfs\n",
    "sched --sjf\n",
    "sched --priority\n",
    "sched --rr 2\n",
    "sched --paper\n",
    "pci\n",
    "ifconfig\n",
    "ping 192.168.1.1\n",
    "ping 127.0.0.1\n",
    "wifi\n",
    "apt list\n",
    "apt install snake-arcade\n",
    "apt show snake-arcade\n",
    "snap list\n",
    "snapstore\n",
    "bench\n"
]

for c in test_commands:
    proc.stdin.write(c)
    proc.stdin.flush()
    time.sleep(0.3)

time.sleep(2.0)
proc.terminate()
try:
    stdout, stderr = proc.communicate(timeout=3)
except:
    proc.kill()
    stdout, stderr = proc.communicate()

print(stdout)