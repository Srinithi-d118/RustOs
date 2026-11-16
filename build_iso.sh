#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$DIR"

export PATH="/root/.cargo/bin:/usr/local/bin:/usr/bin:/bin:$PATH"

echo "Building RustOS boot image..."
cargo bootimage

BOOTIMAGE="$DIR/target/x86_64-blog_os/debug/bootimage-rust_os.bin"
ISO_DIR="$DIR/target/rustos_iso"
ISO_OUT="$DIR/target/rust_os.iso"

test -f "$BOOTIMAGE" || { echo "Boot image not found"; exit 1; }
test -f /usr/lib/syslinux/memdisk || { echo "MEMDISK not found"; exit 1; }

echo "Preparing GRUB and MEMDISK files..."
rm -rf "$ISO_DIR"
mkdir -p "$ISO_DIR/boot/grub"

cp "$BOOTIMAGE" "$ISO_DIR/boot/rustos.img"
cp /usr/lib/syslinux/memdisk "$ISO_DIR/boot/memdisk"

cat > "$ISO_DIR/boot/grub/grub.cfg" <<'GRUB'
set timeout=5
set default=0

menuentry "RustOS - Start Operating System" {
    linux16 /boot/memdisk harddisk
    initrd16 /boot/rustos.img
}
GRUB

echo "Creating bootable ISO..."
grub-mkrescue -o "$ISO_OUT" "$ISO_DIR"

echo "Checking ISO..."
ls -lh "$ISO_OUT"

echo "RustOS ISO generation completed."
