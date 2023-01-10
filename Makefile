# Settings that might need to be adjusted for a different project
AVR_TARGET=atmega32
export AVR_CPU_FREQUENCY_HZ=8000000

CARGO_OPTS=-Z build-std=core --target $(AVR_TARGET).json --release

# Other variables
ELF_PATH=target/$(AVR_TARGET)/release/examples
ELF_FILES=$(subst .rs,.elf,$(subst examples/,$(ELF_PATH)/,$(wildcard examples/*.rs)))
HEX_PATH=target
HEX_FILES=$(subst .rs,.hex,$(subst examples/,$(HEX_PATH)/,$(wildcard examples/*.rs)))

# Target definitions
all: hex doc

elfs: $(ELF_FILES)

$(ELF_PATH)/%.elf: examples/%.rs
	@echo "Building example $< for the $(AVR_TARGET) architecture with cargo:"
	cargo build $(CARGO_OPTS) --example $(basename $(notdir $<))

examples:
	cargo build $(CARGO_OPTS) --examples

hex: examples $(HEX_FILES)

$(HEX_PATH)/%.hex: $(ELF_PATH)/%.elf
	@echo "Rebuilding $@:"
	avr-objcopy -O ihex -R .eeprom $< $@
	@echo ""

%: $(HEX_PATH)/%.hex

doc:
	@echo "Building rust docs for the $(AVR_TARGET) architecture with cargo:"
	cargo doc $(CARGO_OPTS)

clean:
	@cargo clean

.PHONY: all elfs hex doc clean
