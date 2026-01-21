# ---- Minimal RPi3 kernel Makefile (bare metal Rust) ----
# Produces: kernel8.img (what the Pi firmware loads)

# 1) Target triple (AArch64, no OS)
TARGET     := aarch64-unknown-none-softfloat

# 2) Output filename expected by RPi firmware for 64-bit kernels
KERNEL_BIN := kernel8.img

# 3) Where your linker script lives + its name
LD_SCRIPT_PATH       := $(shell pwd)/
KERNEL_LINKER_SCRIPT := linker.ld

# Cargo’s default kernel ELF path (adjust "kernel" if your package/bin name differs)
KERNEL_ELF := target/$(TARGET)/release/kernel

# 4) Rustc/linker flags: CPU tuning + pass linker script to ld
RUSTFLAGS := -C target-cpu=cortex-a53 \
             -C link-arg=--library-path=$(LD_SCRIPT_PATH) \
             -C link-arg=--script=$(KERNEL_LINKER_SCRIPT)

# 5) Build command
CARGO_RUSTC := cargo rustc --target=$(TARGET) --release

# 6) Convert ELF -> raw binary
OBJCOPY := rust-objcopy --strip-all -O binary

.PHONY: all clean

# 7) Tie it together
all: $(KERNEL_BIN)

$(KERNEL_ELF):
	@RUSTFLAGS="$(RUSTFLAGS)" $(CARGO_RUSTC)

$(KERNEL_BIN): $(KERNEL_ELF)
	@$(OBJCOPY) $(KERNEL_ELF) $(KERNEL_BIN)
	@echo "Built $(KERNEL_BIN)"

clean:
	@rm -rf target $(KERNEL_BIN)
