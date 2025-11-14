#!/usr/bin/env python3
"""
ESP32-C6 Register Discovery Tool

Searches the esp32c6 PAC crate for peripheral base addresses and register offsets.
This teaches students how to find hardware registers without reading datasheets.

Usage:
    python3 find-registers.py GPIO
    python3 find-registers.py UART
    python3 find-registers.py I2C
    python3 find-registers.py --all
"""

import argparse
import glob
import re
import os
from pathlib import Path


def find_pac_crate():
    """Find the esp32c6 PAC crate in cargo registry"""
    registry_path = Path.home() / ".cargo/registry/src"

    if not registry_path.exists():
        print("Error: Cargo registry not found at ~/.cargo/registry/src")
        print("Have you built any ESP32-C6 projects yet?")
        return None

    # Find all esp32c6 crate versions
    crates = sorted(
        registry_path.glob("*/esp32c6-*"),
        key=lambda p: p.name,
        reverse=True  # Latest version first
    )

    if not crates:
        print("Error: esp32c6 PAC crate not found in cargo registry")
        print("Try: cargo add esp32c6")
        return None

    return crates[0]  # Return latest version


def find_peripheral_base(pac_path, peripheral):
    """Find base address for a peripheral"""
    lib_path = pac_path / "src/lib.rs"

    if not lib_path.exists():
        print(f"Error: {lib_path} not found")
        return None

    # Search for peripheral definition like:
    # pub type GPIO = crate::Periph<gpio::RegisterBlock, 0x6009_1000>;
    pattern = rf"pub type {peripheral}\s*=.*?0x([0-9a-fA-F_]+)"

    with open(lib_path, 'r') as f:
        content = f.read()
        match = re.search(pattern, content, re.MULTILINE)

        if match:
            addr_str = match.group(1).replace('_', '')
            return int(addr_str, 16)

    return None


def find_register_offsets(pac_path, peripheral):
    """Find register offsets for a peripheral"""
    # Peripheral module file (e.g., gpio.rs, uart.rs)
    module_file = pac_path / "src" / f"{peripheral.lower()}.rs"

    if not module_file.exists():
        print(f"Warning: {module_file} not found")
        return []

    registers = []

    with open(module_file, 'r') as f:
        content = f.read()

        # Find register definitions like:
        # #[doc = "0x04 - GPIO output register for GPIO0-31"]
        # pub const fn out(&self) -> &OUT {
        pattern = r'#\[doc = "0x([0-9a-fA-F]+) - ([^"]+)"\]\s+#\[inline.*?\]\s+pub const fn (\w+)\('

        for match in re.finditer(pattern, content, re.MULTILINE | re.DOTALL):
            offset_str = match.group(1)
            description = match.group(2)
            name = match.group(3).upper()

            offset = int(offset_str, 16)
            registers.append({
                'name': name,
                'offset': offset,
                'description': description
            })

    # Sort by offset
    registers.sort(key=lambda r: r['offset'])

    return registers


def print_peripheral_info(peripheral, base_addr, registers):
    """Pretty-print peripheral information"""
    print(f"\n{'='*80}")
    print(f"  {peripheral} Peripheral")
    print(f"{'='*80}")
    print(f"\nBase Address: 0x{base_addr:08X}")
    print(f"\nRegisters found: {len(registers)}")
    print(f"\n{'Register':<20} {'Offset':<10} {'Address':<12} {'Description'}")
    print('-' * 80)

    for reg in registers:
        addr = base_addr + reg['offset']
        print(f"{reg['name']:<20} 0x{reg['offset']:04X}     0x{addr:08X}   {reg['description'][:40]}")

    print(f"\n{'='*80}\n")


def generate_gdb_commands(peripheral, base_addr, registers):
    """Generate GDB commands for register access"""
    print(f"\n## GDB Commands for {peripheral}\n")
    print("```gdb")

    for reg in registers[:10]:  # Show first 10 registers
        addr = base_addr + reg['offset']
        reg_name = reg['name'].replace('_', '')
        print(f"# {reg['description'][:50]}")
        print(f"(gdb) x/1xw 0x{addr:08X}  # {peripheral}_{reg_name}")
        print()

    print("```\n")


def main():
    parser = argparse.ArgumentParser(
        description='Find ESP32-C6 peripheral registers from PAC crate',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python3 find-registers.py GPIO
  python3 find-registers.py UART
  python3 find-registers.py I2C0
  python3 find-registers.py --all
        """
    )

    parser.add_argument(
        'peripheral',
        nargs='?',
        help='Peripheral name (GPIO, UART, I2C, etc.)'
    )

    parser.add_argument(
        '--all',
        action='store_true',
        help='List all available peripherals'
    )

    parser.add_argument(
        '--gdb',
        action='store_true',
        help='Generate GDB commands'
    )

    args = parser.parse_args()

    # Find PAC crate
    pac_path = find_pac_crate()
    if not pac_path:
        return 1

    print(f"Using PAC crate: {pac_path.name}")

    # List all peripherals
    if args.all:
        lib_path = pac_path / "src/lib.rs"
        with open(lib_path, 'r') as f:
            content = f.read()
            # Find all peripheral definitions
            pattern = r"pub type (\w+) = crate::Periph<.*?, 0x([0-9a-fA-F_]+)>"

            peripherals = []
            for match in re.finditer(pattern, content):
                name = match.group(1)
                addr_str = match.group(2).replace('_', '')
                addr = int(addr_str, 16)
                peripherals.append((name, addr))

            peripherals.sort()

            print(f"\nAvailable Peripherals ({len(peripherals)}):\n")
            print(f"{'Name':<20} {'Base Address'}")
            print('-' * 40)
            for name, addr in peripherals:
                print(f"{name:<20} 0x{addr:08X}")
            print()

        return 0

    # Find specific peripheral
    if not args.peripheral:
        parser.print_help()
        return 1

    peripheral = args.peripheral.upper()

    # Find base address
    base_addr = find_peripheral_base(pac_path, peripheral)
    if base_addr is None:
        print(f"Error: Peripheral '{peripheral}' not found")
        print("Try: python3 find-registers.py --all")
        return 1

    # Find registers
    registers = find_register_offsets(pac_path, peripheral)

    # Print results
    print_peripheral_info(peripheral, base_addr, registers)

    # Generate GDB commands if requested
    if args.gdb:
        generate_gdb_commands(peripheral, base_addr, registers)

    return 0


if __name__ == '__main__':
    exit(main())
