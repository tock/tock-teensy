# Teensy 3.6
CHIP = mk66
MCU=mk66fx1m0

PLATFORM=teensy

TARGET = thumbv7em-none-eabi
TEENSY_LOADER=teensy_loader_cli

APP ?= blink

# include the base Tock kernel makefile
include ../../tock/boards/Makefile.common

.PHONY: program
program: target/$(TARGET)/release/$(PLATFORM).hex
	$(TEENSY_LOADER) -mmcu=$(MCU) -v -w $<


.PHONY: app
app: target/$(TARGET)/release/$(PLATFORM)-$(APP).hex
	$(TEENSY_LOADER) -mmcu=$(MCU) -v -w $<


.PHONY: ../../apps/$(APP)/build/$(TOCK_ARCH)/app
../../apps/$(APP)/build/$(TOCK_ARCH)/app:
	@make -C ../../apps/$(APP) TOCK_ARCH=$(TOCK_ARCH)


target/$(TARGET)/release/$(PLATFORM)-$(APP): target/$(TARGET)/release/$(PLATFORM) ../../apps/$(APP)/build/$(TOCK_ARCH)/app 
	$(Q)$(OBJCOPY) --update-section .apps=../../apps/$(APP)/build/cortex-m4/cortex-m4.bin \
		--set-section-flags .apps=alloc,code \
		target/$(TARGET)/release/$(PLATFORM) $@

target/$(TARGET)/release/$(PLATFORM)-$(APP).elf: target/$(TARGET)/release/$(PLATFORM)-$(APP)
	$(Q)cp $^ $@

target/$(TARGET)/release/$(PLATFORM)-$(APP).hex: target/$(TARGET)/release/$(PLATFORM)-$(APP).elf
	$(Q)$(OBJCOPY) -Oihex $^ $@

target/$(TARGET)/release/$(PLATFORM).hex: target/$(TARGET)/release/$(PLATFORM).elf
	$(Q)$(OBJCOPY) -Oihex $^ $@
