/**
 * ROM_ORIGIN and ROM_LENGTH define the memory region given to the kernel.
 *
 * Teensy 3.5/3.6 boards run the HalfKay bootloader on an external Cortex-M0, so
 * there is no need to allocate space for a bootloader unless there is a reason
 * to specifically use tockloader.
 *
 * [Teensy 3.6 Schematic]
 */
/**
 * Program flash is 1M total.
 *
 * [Kinetis K66 Sub-Family Reference Manual Section 2.3]
 * [Teensy 3.6 Schematic]
 */
/**
 * SRAM is 256K total, split into an upper 192K chunk and a lower 64K chunk.
 *
 * [Kinetis K66 Sub-Family Reference Manual Section 4.10]
 */
RAM_ORIGIN  = 0x20000000;
/* Memory Spaces Definitions, 448K flash, 64K ram */
/* Use bootloader starting at 0x0000 */
MEMORY
{
  rom (rx)  : ORIGIN = 0x00000000, LENGTH = 0x00020000
  prog (rx) : ORIGIN = 0x00020000, LENGTH = 0x000E0000
  ram (rwx) : ORIGIN = 0x20000000, LENGTH = 192K
}

/**
 * Minimum memory protection unit alignment size.
 * [Kinetis K66 Sub-Family Reference Manual Section 22.3.2]
 *
 * TODO: verify this number
 */
MPU_MIN_ALIGN = 8K;
