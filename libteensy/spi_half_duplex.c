#include "spi_half_duplex.h"
#include <gpio.h>
#include <timer.h>

#define BIT_TIME 4

/**
 * There is no microsecond-resolution delay function, so we're making do with a
 * busy loop that approximately waits for BIT_TIME/2 microseconds.
 *
 * Assumes the CPU frequency is 120 MHz.
 */
static void delay_bit_time(void) {
    int i = 0;
    for (; i < 120*(BIT_TIME/2); i++) {
        asm volatile("nop");
    }
}

/**
 * Initialize the SPI hardware.
 */
void spi_half_duplex_init(void) {
    gpio_enable_output(CLOCK_PIN);
    gpio_enable_output(CHIP_SELECT);

    // Deselect the device.
    gpio_set(CHIP_SELECT);

    // CPOL = 1.
    gpio_set(CLOCK_PIN);
}

/**
 * Write a byte.
 */
void spi_half_duplex_write_byte(uint8_t byte) {
    gpio_enable_output(DATA_PIN);

    int i = 0;
    for (; i < 8; i++) {
        if (byte & 0x80) {
           gpio_set(DATA_PIN);
        } else {
           gpio_clear(DATA_PIN);
        }

        // Data is sampled on the rising edge.
        gpio_clear(CLOCK_PIN);
        delay_bit_time();
        gpio_set(CLOCK_PIN);
        delay_bit_time();

        // Shift the sent bit off.
        byte <<= 1;
    }
}

/**
 * Read a byte.
 */
uint8_t spi_half_duplex_read_byte(void) {
    gpio_enable_input(DATA_PIN, PullNone);

    uint8_t byte = 0;

    int i = 0;
    for (; i < 8; i++) {
        byte <<= 1;

        gpio_clear(CLOCK_PIN);
        delay_bit_time();
        gpio_set(CLOCK_PIN);

        // Sample on the rising edge.
        if (gpio_read(DATA_PIN)) {
            byte |= 1;
        }
        delay_bit_time();
    }

    return byte;
}

/**
 * Assert the chip select.
 */
void spi_half_duplex_begin_transaction(void) {
    gpio_clear(CHIP_SELECT);
}

/**
 * De-assert the chip select.
 */
void spi_half_duplex_end_transaction(void) {
    gpio_set(CHIP_SELECT);
}
