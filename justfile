mod async 'async'
mod board 'board/rp-pico'
mod classic 'classic'

# Remove build artifacts and disk image
@clean: async::clean board::clean classic::clean
    rm -f disk_image.img

# Check formatting and linting
@check: async::check board::check classic::check

# Sort dependencies
@sort: async::sort board::sort classic::sort

# Update dependencies
@update: async::update board::update classic::update

# Check for outdated dependencies
@outdated: async::outdated board::outdated classic::outdated

# Release build all components
@release: async::release board::release classic::release

# Debug build all components
@debug: async::debug board::debug classic::debug
