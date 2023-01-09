# Settings that might need to be adjusted for a different project
AVR_TARGET=atmega32
export AVR_CPU_FREQUENCY_HZ=8000000

# Other variables
CARGO_FILES=$(wildcard Cargo.*)
ELF_PATH=target/$(AVR_TARGET)/release/examples
HEX_PATH=target
HEX_FILES=$(subst .rs,.hex,$(subst examples/,$(HEX_PATH)/,$(wildcard examples/*.rs)))

# Target definitions
all: hex doc

$(ELF_PATH)/%.elf: examples/%.rs $(CARGO_FILES)
	@echo "Building example $< for the $(AVR_TARGET) architecture with cargo:"
	cargo build -Z build-std=core --target $(AVR_TARGET).json --release --example $(basename $(notdir $<))

hex: $(HEX_FILES)

$(HEX_PATH)/%.hex: $(ELF_PATH)/%.elf
	@echo "Rebuilding $@:"
	avr-objcopy -O ihex -R .eeprom $< $@
	@echo ""

%: $(HEX_PATH)/%.hex

doc:
	@echo "Building rust docs for the $(AVR_TARGET) architecture with cargo:"
	cargo doc -Z build-std=core --target $(AVR_TARGET).json --release

clean:
	@cargo clean

.PHONY: all elfs hex doc clean
