# Makefile for user applications

# Specify this directory relative to the current application.
TOCK_USERLAND_BASE_DIR = ../../tock/userland

# Which files to compile.
C_SRCS := $(wildcard *.c)

# Include userland master makefile. Contains rules and flags for actually
# building the application.
include $(TOCK_USERLAND_BASE_DIR)/AppMakefile.mk

APP=$(shell basename $(CURDIR))
