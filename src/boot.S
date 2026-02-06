.section .text._start
.global _start
.type _start, %function

/* -------------------------------------------------------
   Constants
------------------------------------------------------- */

.equ CPUINFO_SIZE,        32
.equ CPUINFO_READY_OFF,    0
.equ CPUINFO_GOTO_OFF,     8
.equ CPUINFO_SP_OFF,      16

/* -------------------------------------------------------
   Entry
------------------------------------------------------- */

_start:

    // Get core id from MPIDR_EL1
    mrs     x0, MPIDR_EL1
    and     x0, x0, #0xFF          // Aff0 = core id
    mov     x2, x0                 // save core_id

    // If not boot core, go to mailbox wait
    cbnz    x0, secondary_wait

/* -------------------------------------------------------
   Boot core path
------------------------------------------------------- */

boot_core:

    // Zero BSS
    adrp    x0, __bss_start
    add     x0, x0, :lo12:__bss_start

    adrp    x1, __bss_end_exclusive
    add     x1, x1, :lo12:__bss_end_exclusive

zero_bss:
    cmp     x0, x1
    b.hs    init_rust

    stp     xzr, xzr, [x0], #16
    b       zero_bss


init_rust:

    // Setup boot stack
    adrp    x0, __boot_core_stack_end_exclusive
    add     x0, x0, :lo12:__boot_core_stack_end_exclusive
    mov     sp, x0

    bl      system_main

halt:
    wfe
    b       halt


/* -------------------------------------------------------
   Secondary core mailbox wait
------------------------------------------------------- */

secondary_wait:

.global wait_loop
wait_loop:
	mrs x2, MPIDR_EL1
	and x2, x2, #0xFF          // Aff0 = core id
    // Load base of mailbox array
    adrp    x1, core_mailbox
    add     x1, x1, :lo12:core_mailbox

    // Compute &core_mailbox[core_id]
    mov     x3, #CPUINFO_SIZE
    madd    x3, x2, x3, x1

    // Read ready flag
    ldr     x4, [x3, #CPUINFO_READY_OFF]
    cbnz    x4, secondary_go

    wfe
    b       wait_loop

secondary_go:

    // Ensure visibility of sp + goto_address
    dmb     ish

    // Load stack pointer
    ldr     x6, [x3, #CPUINFO_SP_OFF]
    mov     sp, x6

    // Load entry address
    ldr     x5, [x3, #CPUINFO_GOTO_OFF]

    // Jump (not call)
    br      x5


.size _start, . - _start
