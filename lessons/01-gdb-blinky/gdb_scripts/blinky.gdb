# GDB Automated LED Blinky for ESP32-C6
# Lesson 01: Control GPIO12 LED using only GDB commands
#
# Usage:
#   1. Flash the blank firmware
#   2. Start debug server: probe-rs attach --chip esp32c6 target/.../main
#   3. In another terminal: riscv32-esp-elf-gdb target/.../main
#   4. (gdb) source gdb_scripts/blinky.gdb
#   5. (gdb) continue

# Connect to debug server
target remote :3333

printf "\n"
printf "========================================\n"
printf "  Lesson 01: GDB Automated Blinky\n"
printf "========================================\n"
printf "\n"

# ESP32-C6 GPIO Registers (from PAC crate)
set $GPIO_BASE      = 0x60091000
set $GPIO_ENABLE_W1TS = 0x60091024  # Enable output
set $GPIO_OUT_W1TS  = 0x60091008    # Set high (ON)
set $GPIO_OUT_W1TC  = 0x6009100C    # Clear low (OFF)
set $GPIO12_MASK    = 0x1000        # Bit 12

# Enable GPIO12 as output
set *(uint32_t*)$GPIO_ENABLE_W1TS = $GPIO12_MASK
printf "✓ GPIO12 configured as output\n"

# Define LED toggle function
define toggle_led
    if $led_state == 0
        # Turn ON
        set *(uint32_t*)$GPIO_OUT_W1TS = $GPIO12_MASK
        printf "💡 LED ON\n"
        set $led_state = 1
    else
        # Turn OFF
        set *(uint32_t*)$GPIO_OUT_W1TC = $GPIO12_MASK
        printf "   LED OFF\n"
        set $led_state = 0
    end
end

document toggle_led
Toggle GPIO12 LED state.
Usage: (gdb) toggle_led
end

# Define manual ON/OFF commands
define led_on
    set *(uint32_t*)$GPIO_OUT_W1TS = $GPIO12_MASK
    printf "💡 LED forced ON\n"
    set $led_state = 1
end

document led_on
Turn LED on manually.
Usage: (gdb) led_on
end

define led_off
    set *(uint32_t*)$GPIO_OUT_W1TC = $GPIO12_MASK
    printf "   LED forced OFF\n"
    set $led_state = 0
end

document led_off
Turn LED off manually.
Usage: (gdb) led_off
end

# Set up automated blinking
printf "\n"
printf "Setting up automated blinking...\n"

set $led_state = 0

# Set breakpoint on delay loop in main
break main.rs:54
commands
    silent
    toggle_led
    continue
end

printf "✓ Breakpoint set on delay loop\n"
printf "\n"
printf "========================================\n"
printf "  Ready!\n"
printf "========================================\n"
printf "\n"
printf "Commands available:\n"
printf "  continue     - Start automated blinking (500ms)\n"
printf "  toggle_led   - Manually toggle LED\n"
printf "  led_on       - Force LED on\n"
printf "  led_off      - Force LED off\n"
printf "  Ctrl-C       - Pause blinking\n"
printf "\n"
printf "Type 'continue' to start blinking:\n"
