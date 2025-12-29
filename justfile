mod async 'async'
mod board 'board/rp-pico'

# Remove build artifacts and disk image
@clean: async::clean board::clean
    rm -f disk_image.img

# Check formatting and linting
@check: async::check board::check

# Sort dependencies
@sort: async::sort board::sort

# Update dependencies
@update: async::update board::update

# Check for outdated dependencies
@outdated: async::outdated board::outdated

# Release build all components
@release: async::release board::release

# Debug build all components
@debug: async::debug board::debug

# Flash cyw43xx firmware to the board
@flash-wifi:
    echo "{{BLUE}}Flashing cyw43 firmware...{{NORMAL}}"
    probe-rs download ./assets/43439A0.bin --binary-format bin --chip RP2040 --base-address 0x10100000
    probe-rs download ./assets/43439A0_clm.bin --binary-format bin --chip RP2040 --base-address 0x10140000