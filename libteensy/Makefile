TOCK_USERLAND_BASE_DIR ?= ../tock/userland

LIBNAME := libteensy
$(LIBNAME)_SRCS := $(wildcard *.c)
$(LIBNAME)_DIR := .

override CPPFLAGS += -I$(TOCK_USERLAND_BASE_DIR)/libtock

include $(TOCK_USERLAND_BASE_DIR)/TockLibrary.mk
