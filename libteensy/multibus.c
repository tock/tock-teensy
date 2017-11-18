#include "tock.h"
#include "spi.h"
#include "multibus.h"

int select_spi_bus(int spi_num) {
    return command(DRIVER_NUM_SPI, 0, spi_num, 0);
}
