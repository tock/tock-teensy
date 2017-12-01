#pragma once

#ifdef __cplusplus
extern "C" {
#endif

#define DATA_PIN 11
#define CLOCK_PIN 27
#define CHIP_SELECT 10

/**
 * Initialize the SPI hardware.
 */
void spi_half_duplex_init(void);

/**
 * Write a byte.
 */
void spi_half_duplex_write_byte(uint8_t byte);

/**
 * Read a byte.
 */
uint8_t spi_half_duplex_read_byte(void);

/**
 * Assert the chip select.
 */
void spi_half_duplex_begin_transaction(void);

/**
 * De-assert the chip select.
 */
void spi_half_duplex_end_transaction(void);

#ifdef __cplusplus
}
#endif
