# GPIO Register Snapshot Tool
# Use this to compare register states between working and non-working firmware
#
# Usage:
#   (gdb) source gdb_scripts/snapshot_registers.gdb

target remote :3333

printf "\n"
printf "========================================\n"
printf "  GPIO Register Snapshot\n"
printf "========================================\n"
printf "\n"

printf "GPIO Peripheral Registers (Base: 0x60091000)\n"
printf "----------------------------------------------\n"
printf "OUT (0x60091004):         "
x/1xw 0x60091004
printf "OUT_W1TS (0x60091008):    "
x/1xw 0x60091008
printf "OUT_W1TC (0x6009100C):    "
x/1xw 0x6009100C
printf "ENABLE (0x60091020):      "
x/1xw 0x60091020
printf "ENABLE_W1TS (0x60091024): "
x/1xw 0x60091024
printf "\n"

printf "IO_MUX Registers (Base: 0x60090000)\n"
printf "----------------------------------------------\n"
printf "GPIO12 IO_MUX (0x60090034): "
x/1xw 0x60090034
printf "\n"

printf "GPIO PIN Registers (Base: 0x60091000 + offset)\n"
printf "----------------------------------------------\n"
printf "GPIO12 PIN_CTRL (0x600910B4): "
x/1xw 0x600910B4
printf "\n"

printf "Full GPIO Register Dump:\n"
printf "----------------------------------------------\n"
x/64xw 0x60091000

printf "\n"
printf "Full IO_MUX Register Dump:\n"
printf "----------------------------------------------\n"
x/32xw 0x60090000

printf "\n"
printf "========================================\n"
printf "  Snapshot Complete\n"
printf "========================================\n"
printf "\n"
printf "Compare this output between:\n"
printf "  1. Blank firmware (no GPIO config)\n"
printf "  2. Rust HAL firmware (LED working)\n"
printf "\n"
printf "Look for differences to find missing config!\n"
printf "\n"
