# Continuous GPIO12 blink test
# This will blink indefinitely so you can check LED polarity
#
# Usage:
#   1. Start probe-rs: probe-rs attach --chip esp32c6 target/.../main
#   2. In another terminal: riscv32-esp-elf-gdb target/.../main -x test_continuous_blink.gdb

target remote :3333

# GPIO register addresses
set $GPIO_ENABLE_W1TS = 0x60091024
set $GPIO_OUT_W1TS    = 0x60091008
set $GPIO_OUT_W1TC    = 0x6009100C
set $GPIO12_MASK      = 0x1000

printf "\n"
printf "========================================\n"
printf "  Continuous GPIO12 Blink Test\n"
printf "  Press Ctrl-C to stop\n"
printf "========================================\n"
printf "\n"

# Enable GPIO12 as output
printf "Enabling GPIO12 as output...\n"
set *(uint32_t*)$GPIO_ENABLE_W1TS = $GPIO12_MASK
printf "✓ GPIO12 enabled\n\n"

printf "Starting continuous blink (1 Hz)...\n"
printf "If LED doesn't blink, try reversing polarity\n\n"

# Continuous blink loop
set $count = 0
while 1
    # LED ON
    printf "%d. LED ON (GPIO12 HIGH)\n", $count
    set *(uint32_t*)$GPIO_OUT_W1TS = $GPIO12_MASK
    shell sleep 1

    # LED OFF
    printf "%d. LED OFF (GPIO12 LOW)\n\n", $count
    set *(uint32_t*)$GPIO_OUT_W1TC = $GPIO12_MASK
    shell sleep 1

    set $count = $count + 1
end
