# GDB Manual LED Control for ESP32-C6
# Lesson 01: Step-by-step LED control discovery
#
# Usage:
#   1. Flash firmware and start debug server
#   2. riscv32-esp-elf-gdb target/.../main
#   3. (gdb) source gdb_scripts/manual_control.gdb
#   4. Follow the prompts

target remote :3333

printf "\n"
printf "========================================\n"
printf "  Lesson 01: Manual LED Control\n"
printf "  Discover GPIO registers step-by-step\n"
printf "========================================\n"
printf "\n"

# GPIO register addresses
set $GPIO_ENABLE_W1TS = 0x60091024
set $GPIO_OUT_W1TS    = 0x60091008
set $GPIO_OUT_W1TC    = 0x6009100C
set $GPIO_OUT         = 0x60091004
set $GPIO8_MASK       = 0x100

printf "GPIO8 Register Addresses:\n"
printf "  ENABLE_W1TS: 0x%08X (enable output)\n", $GPIO_ENABLE_W1TS
printf "  OUT_W1TS:    0x%08X (set high)\n", $GPIO_OUT_W1TS
printf "  OUT_W1TC:    0x%08X (clear low)\n", $GPIO_OUT_W1TC
printf "  OUT:         0x%08X (read/write value)\n", $GPIO_OUT
printf "  GPIO8 mask:  0x%X (bit 8)\n", $GPIO8_MASK
printf "\n"

# Helper commands
define step1
    printf "\nStep 1: Enable GPIO8 as output\n"
    printf "Command: set *(uint32_t*)0x60091024 = 0x100\n"
    set *(uint32_t*)$GPIO_ENABLE_W1TS = $GPIO8_MASK
    printf "✓ GPIO8 enabled as output\n\n"
    printf "Next: Type 'step2' to turn LED ON\n"
end

define step2
    printf "\nStep 2: Turn LED ON\n"
    printf "Command: set *(uint32_t*)0x60091008 = 0x100\n"
    set *(uint32_t*)$GPIO_OUT_W1TS = $GPIO8_MASK
    printf "✓ LED should be ON now!\n\n"
    printf "Check the LED. Is it on? (yes/no)\n"
    printf "Next: Type 'step3' to turn LED OFF\n"
end

define step3
    printf "\nStep 3: Turn LED OFF\n"
    printf "Command: set *(uint32_t*)0x6009100C = 0x100\n"
    set *(uint32_t*)$GPIO_OUT_W1TC = $GPIO8_MASK
    printf "✓ LED should be OFF now\n\n"
    printf "Next: Type 'step4' to read GPIO state\n"
end

define step4
    printf "\nStep 4: Read GPIO output register\n"
    printf "Command: x/1xw 0x60091004\n"
    x/1xw $GPIO_OUT
    printf "\n"
    printf "Bit 8 (0x100) should be 0 (LED off)\n"
    printf "\nNow try turning it back on: step2\n"
end

# Document commands
document step1
Enable GPIO8 as output.
end

document step2
Turn LED ON.
end

document step3
Turn LED OFF.
end

document step4
Read GPIO output register to verify state.
end

printf "========================================\n"
printf "  Interactive Learning Mode\n"
printf "========================================\n"
printf "\n"
printf "Type these commands in order:\n"
printf "  step1  - Enable GPIO8 output\n"
printf "  step2  - Turn LED ON\n"
printf "  step3  - Turn LED OFF\n"
printf "  step4  - Read GPIO state\n"
printf "\n"
printf "Start with: step1\n"
