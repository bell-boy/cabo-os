/* SPDX-License-Identifier: MIT OR Apache-2.0
 *
 * Copyright (c) 2018-2022 Andre Richter <andre.o.richter@gmail.com>
 */

/* The physical address at which the the kernel binary will be loaded by the Raspberry's firmware */
__rpi_phys_binary_load_addr = 0x80000;


ENTRY(__rpi_phys_binary_load_addr)

/* Flags:
 *     4 == R
 *     5 == RX
 *     6 == RW
 */
PHDRS
{
    segment_code PT_LOAD FLAGS(5);
    ro_data PT_LOAD FLAGS(4);
    data PT_LOAD FLAGS(6);

}

SECTIONS
{
    . =  __rpi_phys_binary_load_addr;

    /***********************************************************************************************
    * Code
    ***********************************************************************************************/
    .text :
    {
        KEEP(*(.text._start))
        *(.text*)
    } :segment_code

    /***********************************************************************************************
    * Read-only data
    ***********************************************************************************************/
    .rodata :
    {
        *(.rodata*)
    } :ro_data

    /***********************************************************************************************
    * Data
    ***********************************************************************************************/
    .data :
    {
        *(.data*)
    } :data

    /***********************************************************************************************
    * BSS + stack (no load)
    ***********************************************************************************************/
    .bss (NOLOAD) :
    {
        __bss_start = .;
        *(.bss*)

        . = ALIGN(16);
        __boot_core_stack_start = .;
        . = . + 0x80000; // 512 KB for boot core's stack
        __boot_core_stack_end_exclusive = .;
        //TODO: do we need to include small stack
        __bss_end_exclusive = .;
    }: data

}
