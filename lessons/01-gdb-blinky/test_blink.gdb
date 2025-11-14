# Quick LED blink test for manual verification
# This script blinks GPIO8 LED 3 times
#
# Usage:
#   1. Start probe-rs: probe-rs attach --chip esp32c6 --protocol jtag target/.../main
#   2. In another terminal: riscv32-esp-elf-gdb target/.../main -x test_blink.gdb

target remote :3333

# GPIO register addresses
set $GPIO_ENABLE_W1TS = 0x60091024
set $GPIO_OUT_W1TS    = 0x60091008
set $GPIO_OUT_W1TC    = 0x6009100C
set $GPIO12_MASK      = 0x1000

printf "\n"
printf "========================================\n"
printf "  LED Blink Test (GPIO12)\n"
printf "========================================\n"
printf "\n"

# Enable GPIO12 as output
printf "Enabling GPIO12 as output...\n"
set *(uint32_t*)$GPIO_ENABLE_W1TS = $GPIO12_MASK
printf "✓ GPIO12 enabled\n\n"

# Blink 3 times
printf "Blinking LED 3 times...\n\n"

# Blink 1
printf "1. LED ON\n"
set *(uint32_t*)$GPIO_OUT_W1TS = $GPIO12_MASK
shell sleep 1
printf "1. LED OFF\n\n"
set *(uint32_t*)$GPIO_OUT_W1TC = $GPIO12_MASK
shell sleep 1

# Blink 2
printf "2. LED ON\n"
set *(uint32_t*)$GPIO_OUT_W1TS = $GPIO12_MASK
shell sleep 1
printf "2. LED OFF\n\n"
set *(uint32_t*)$GPIO_OUT_W1TC = $GPIO12_MASK
shell sleep 1

# Blink 3
printf "3. LED ON\n"
set *(uint32_t*)$GPIO_OUT_W1TS = $GPIO12_MASK
shell sleep 1
printf "3. LED OFF\n\n"
set *(uint32_t*)$GPIO_OUT_W1TC = $GPIO12_MASK

printf "========================================\n"
printf "  Test Complete!\n"
printf "========================================\n"
printf "\n"
printf "Did you see GPIO12 LED blink 3 times?\n"
printf "\n"

quit
