mod pico 'pico-rs'

# Remove build artifacts and disk image
@clean: pico::clean

# Check formatting and linting
@check: pico::check

# Sort dependencies
@sort: pico::sort

# Update dependencies
@update: pico::update

# Check for outdated dependencies
@outdated: pico::outdated

# Release build all components
@release: pico::release

# Debug build all components
@debug: pico::debug

# Flash cyw43xx firmware to the board
@flash-wifi:
    echo "{{BLUE}}Flashing cyw43 firmware...{{NORMAL}}"
    probe-rs download ./assets/43439A0.bin --binary-format bin --chip RP2040 --base-address 0x10100000
    probe-rs download ./assets/43439A0_clm.bin --binary-format bin --chip RP2040 --base-address 0x10140000