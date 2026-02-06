## Minimal RPi3 build + run helpers (no Docker).

BSP                := rpi3
TARGET             := aarch64-unknown-none-softfloat
KERNEL_BIN         := kernel8.img
QEMU_BINARY        := qemu-system-aarch64
QEMU_MACHINE_TYPE  := raspi3b
LD_SCRIPT_PATH     := .
QEMU_RELEASE_ARGS := -serial stdio -display none
# QEMU_RELEASE_ARGS  := -d in_asm
RUSTC_MISC_ARGS    := -C target-cpu=cortex-a53

export LD_SCRIPT_PATH

KERNEL_MANIFEST = Cargo.toml
KERNEL_ELF      = target/$(TARGET)/release/kernel

RUSTFLAGS = $(RUSTC_MISC_ARGS)                   \
    -C link-arg=--library-path=$(LD_SCRIPT_PATH) \
    -C link-arg=--script=kernel.ld

FEATURES      = --features bsp_$(BSP)
COMPILER_ARGS = --target=$(TARGET) \
    $(FEATURES)                    \
    --release

RUSTC_CMD   = cargo rustc $(COMPILER_ARGS)
OBJCOPY_CMD = rust-objcopy --strip-all -O binary

.PHONY: all qemu clean

all: $(KERNEL_BIN)

$(KERNEL_ELF): $(KERNEL_MANIFEST)
	@RUSTFLAGS="$(RUSTFLAGS)" $(RUSTC_CMD)

$(KERNEL_BIN): $(KERNEL_ELF)
	@$(OBJCOPY_CMD) $(KERNEL_ELF) $(KERNEL_BIN)

qemu: $(KERNEL_BIN)
	@$(QEMU_BINARY) -M $(QEMU_MACHINE_TYPE) $(QEMU_RELEASE_ARGS) -kernel $(KERNEL_BIN) 

clean:
	rm -rf target $(KERNEL_BIN)
