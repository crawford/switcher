CC = arm-none-eabi-gcc
LD = arm-none-eabi-gcc

LIB_DIR = ../../src

CFLAGS = \
    -mcpu=cortex-m0 \
    -mthumb \
    -Wall \
    -std=c99 \
    -I$(LIB_DIR)/ \

LDFLAGS = \
    -mcpu=cortex-m0 \
    -mthumb \
    --specs=nosys.specs \

.PHONY: all
.DEFAULT: all

all: simple

simple: main.o $(LIB_DIR)/crc.o $(LIB_DIR)/switcher.o
	@echo "    LD   " $<
	@$(LD) $(LDFLAGS) $^ --output $@

%.o: %.c
	@echo "    CC   " $<
	@$(CC) $(CFLAGS) -c $< --output $@
