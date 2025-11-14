# GDB init file for ESP32-C6 Rust debugging
#
# Usage:
#   1. Start OpenOCD in one terminal:
#      openocd -f board/esp32c6.cfg
#
#   2. In another terminal, run GDB:
#      riscv32-esp-elf-gdb target/riscv32imc-unknown-none-elf/debug/main
#
# This .gdbinit will automatically connect to OpenOCD and prepare for debugging.

# Set architecture for ESP32-C6 (RISC-V 32-bit)
set arch riscv:rv32

# Connect to OpenOCD
target extended-remote :3333

# Load the program
load

# Enable pretty-printing for Rust types
set print pretty on
set print array on
set print array-indexes on

# Show demangled Rust function names
set print asm-demangle on
set demangle-style rust

# Set pagination off for scripting
set pagination off

# Set logging (optional, useful for AI analysis)
# set logging file gdb.log
# set logging on

# Useful aliases
define reset-target
    monitor reset halt
    load
end

define show-peripherals
    # I2C0 base: 0x60013000
    printf "\\n=== I2C0 Registers ===\\n"
    printf "  CUST_ADDR: 0x%08x\\n", *(unsigned int*)0x60013000
    printf "  STATUS:    0x%08x\\n", *(unsigned int*)0x60013004

    # GPIO base: 0x60004000
    printf "\\n=== GPIO Registers ===\\n"
    printf "  OUT:       0x%08x\\n", *(unsigned int*)0x60004004
    printf "  IN:        0x%08x\\n", *(unsigned int*)0x6000403C
end

define show-stack
    info frame
    info locals
    backtrace
end

# Print helpful startup message
echo \\n
echo ===================================\\n
echo ESP32-C6 GDB Debug Session (Rust)\\n
echo ===================================\\n
echo Connected to OpenOCD on :3333\\n
echo Program loaded. Type 'continue' to start.\\n
echo \\n
echo Useful commands:\\n
echo   reset-target     - Reset and reload firmware\\n
echo   show-peripherals - Display I2C/GPIO registers\\n
echo   show-stack       - Show frame, locals, backtrace\\n
echo ===================================\\n
echo \\n
