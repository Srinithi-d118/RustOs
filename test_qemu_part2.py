import subprocess
import time

cmd = [
    "qemu-system-x86_64",
    "-drive", "format=raw,file=/root/rust_os/rust_os/target/x86_64-blog_os/debug/bootimage-rust_os.bin",
    "-netdev", "user,id=net0",
    "-device", "rtl8139,netdev=net0",
    "-serial", "stdio",
    "-display", "none"
]

proc = subprocess.Popen(cmd, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
time.sleep(0.8)

test_commands = [
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
    time.sleep(0.4)

time.sleep(2.5)
proc.terminate()
try:
    stdout, stderr = proc.communicate(timeout=3)
except:
    proc.kill()
    stdout, stderr = proc.communicate()

print(stdout)