# Settings that might need to be adjusted for a different project
AVR_TARGET=atmega32
export AVR_CPU_FREQUENCY_HZ=8000000

# Other variables
CARGO_OPTS=--release

ELF_PATH=target/$(AVR_TARGET)/release/examples
ELF_FILES=$(subst .rs,.elf,$(subst examples/,$(ELF_PATH)/,$(wildcard examples/*.rs)))
HEX_PATH=target
HEX_FILES=$(subst .rs,.hex,$(subst examples/,$(HEX_PATH)/,$(wildcard examples/*.rs)))

ROBOTLOADER_PATH=robotloader

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
	@echo ""
	@avr-size --format=avr --mcu=$(AVR_TARGET) $<
	@echo "Rebuilding $@:"
	avr-objcopy -O ihex -R .eeprom $< $@
	@echo ""

%: $(HEX_PATH)/%.hex

doc:
	@echo "Building rust docs for the $(AVR_TARGET) architecture with cargo:"
	cargo doc $(CARGO_OPTS) --all-features --document-private-items --no-deps
	@echo "Updating ./docs rust docs for the $(AVR_TARGET) architecture with cargo:"
	@rm -rf docs/
	@cp -r target/atmega32/doc/ docs/
	@echo "<meta http-equiv=\"refresh\" content=\"0; url=rp6\">" > docs/index.html


clean:
	@cargo clean

robotloader: hex
	cd $(ROBOTLOADER_PATH)/ && sudo ./robotloader_linux_x64.sh

.PHONY: all elfs hex doc clean robotloader
