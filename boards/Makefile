# Generic makefile for building the tock kernel on teensy

TARGET = thumbv7em-none-eabi
TEENSY_LOADER=teensy_loader_cli

# include the base Tock kernel makefile
include ../../tock/boards/Makefile.common

.PHONY: program
program: $(TOCK_BOARD)/target/$(TARGET)/release/$(PLATFORM).hex
	$(TEENSY_LOADER) -mmcu=$(MCU) -v -w $<
