// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2021-2023 Andre Richter <andre.o.richter@gmail.com>

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------
// - https://sourceware.org/binutils/docs-2.36/as/AArch64_002dRelocations.html
.macro ADR_REL register, symbol
	adrp	\register, \symbol
	add	\register, \register, #:lo12:\symbol
.endm

.section .text._start

//------------------------------------------------------------------------------
// fn _start()
//------------------------------------------------------------------------------
_start:
	mrs	x0, MPIDR_EL1
	//extract cpu/core number, half if it's not zero
	and	x0, x0, #0xFF
	cmp	xzr, x0
	b.ne _halt

	// If execution reaches here, it is the boot core.

	// Initialize DRAM.
	ADR_REL	x0, __bss_start
	ADR_REL x1, __bss_end_exclusive

// initialize bss to zero
_init_bss:
    cmp x0, x1
    b.eq _init_rust
    stp xzr, xzr, [x0], #16 
    b _init_bss

_init_rust:
    // set the stack pointer to stack initialized in linker.ld, 0x80000 bytes
    ADR_REL x0, __boot_core_stack_end_exclusive
    mov sp, x0
    bl kmain

_halt:
	wfe
	b _halt



.size	_start, . - _start
.type	_start, function
.global	_start
