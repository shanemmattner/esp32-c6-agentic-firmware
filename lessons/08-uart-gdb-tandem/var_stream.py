#!/usr/bin/env python3
"""
Arbitrary Runtime Variable Streaming Tool

This tool provides a user-friendly interface to stream arbitrary variables from
ESP32-C6 firmware at runtime WITHOUT compile-time registration.

Architecture:
1. Parse ELF file to extract variable addresses and types
2. Send commands to ESP32 firmware over USB CDC
3. Receive hex data and convert back to meaningful values
4. Display real-time variable values

Usage:
    python var_stream.py firmware.elf
    >>> watch sensor_temp 100        # Stream sensor_temp at 100 Hz
    >>> watch gpio_state 1000        # Stream gpio_state at 1000 Hz
    >>> list                         # List all available variables
    >>> stop sensor_temp             # Stop streaming sensor_temp
    >>> quit
"""

import serial
import time
import struct
import sys
from typing import Dict, Tuple, Optional
from dataclasses import dataclass
import subprocess
import re


@dataclass
class Variable:
    """Variable metadata from ELF file"""
    name: str
    address: int
    size: int
    type_name: str


class ElfParser:
    """Parse ELF file to extract variable addresses and types"""

    def __init__(self, elf_path: str):
        self.elf_path = elf_path
        self.variables: Dict[str, Variable] = {}
        self._parse()

    def _parse(self):
        """Extract symbols from ELF using nm command"""
        try:
            # Use nm to get symbol table
            result = subprocess.run(
                ['nm', '-S', '--size-sort', self.elf_path],
                capture_output=True,
                text=True,
                check=True
            )

            # Parse nm output format:
            # address size type name
            for line in result.stdout.splitlines():
                parts = line.split()
                if len(parts) >= 4:
                    try:
                        addr = int(parts[0], 16)
                        size = int(parts[1], 16)
                        sym_type = parts[2]
                        name = parts[3]

                        # Only include data symbols (B, D, d, b)
                        if sym_type in ['B', 'D', 'd', 'b']:
                            # Filter out internal symbols
                            if not name.startswith('_') and not name.startswith('.'):
                                self.variables[name] = Variable(
                                    name=name,
                                    address=addr,
                                    size=size,
                                    type_name=f"u{size*8}" if size <= 8 else "array"
                                )
                    except (ValueError, IndexError):
                        continue

            print(f"✓ Loaded {len(self.variables)} variables from {self.elf_path}")

        except subprocess.CalledProcessError as e:
            print(f"❌ Failed to parse ELF file: {e}")
            sys.exit(1)

    def find(self, name: str) -> Optional[Variable]:
        """Find variable by name (supports partial matching)"""
        # Exact match
        if name in self.variables:
            return self.variables[name]

        # Partial match
        matches = [v for n, v in self.variables.items() if name.lower() in n.lower()]
        if len(matches) == 1:
            return matches[0]
        elif len(matches) > 1:
            print(f"⚠ Multiple matches for '{name}':")
            for v in matches[:5]:
                print(f"  - {v.name}")
            return None

        return None

    def list_variables(self, limit: int = 50):
        """List all available variables"""
        vars_sorted = sorted(self.variables.values(), key=lambda v: v.address)
        print(f"\n{'Name':<40} {'Address':<12} {'Size':<6} {'Type':<10}")
        print("-" * 70)
        for i, var in enumerate(vars_sorted[:limit]):
            print(f"{var.name:<40} 0x{var.address:08x}   {var.size:<6} {var.type_name:<10}")

        if len(vars_sorted) > limit:
            print(f"\n... and {len(vars_sorted) - limit} more")


class VariableStreamer:
    """Manages bidirectional USB CDC communication with ESP32"""

    def __init__(self, port: str, baudrate: int = 115200):
        self.port = port
        self.baudrate = baudrate
        self.ser: Optional[serial.Serial] = None
        self.streams: Dict[str, Variable] = {}  # Active streams

    def connect(self):
        """Open serial connection"""
        try:
            self.ser = serial.Serial(self.port, self.baudrate, timeout=0.1)
            print(f"✓ Connected to {self.port}")
            time.sleep(1)  # Wait for device to settle

            # Ping device
            self.ser.write(b"PING\n")
            response = self.ser.readline().decode('utf-8', errors='replace').strip()
            if response == "PONG":
                print("✓ Device responded to ping")
            else:
                print(f"⚠ Expected PONG, got: {response}")

        except serial.SerialException as e:
            print(f"❌ Failed to connect: {e}")
            sys.exit(1)

    def watch(self, var: Variable, rate_hz: int):
        """Start streaming a variable"""
        if not self.ser:
            print("❌ Not connected")
            return

        # Send STREAM command
        cmd = f"STREAM 0x{var.address:08x} {var.size} {rate_hz}\n"
        self.ser.write(cmd.encode())
        self.streams[var.name] = var
        print(f"✓ Watching {var.name} @ {rate_hz} Hz")

    def stop(self, var_name: str):
        """Stop streaming a variable"""
        if var_name not in self.streams:
            print(f"⚠ {var_name} is not being watched")
            return

        var = self.streams[var_name]
        cmd = f"STOP 0x{var.address:08x}\n"
        self.ser.write(cmd.encode())
        del self.streams[var_name]
        print(f"✓ Stopped watching {var_name}")

    def read_and_display(self, timeout: float = 1.0):
        """Read incoming data and display"""
        if not self.ser:
            return

        start = time.time()
        while time.time() - start < timeout:
            if self.ser.in_waiting > 0:
                line = self.ser.readline().decode('utf-8', errors='replace').strip()
                if line:
                    self._parse_line(line)

    def _parse_line(self, line: str):
        """Parse incoming data line"""
        if line.startswith("DATA|"):
            # Format: DATA|addr=0xXXXXXXXX|hex=AABBCCDD
            parts = {}
            for part in line.split('|')[1:]:
                if '=' in part:
                    key, val = part.split('=', 1)
                    parts[key] = val

            if 'addr' in parts and 'hex' in parts:
                addr_str = parts['addr']
                hex_data = parts['hex']

                # Find which variable this belongs to
                addr = int(addr_str, 16)
                var_name = None
                for name, var in self.streams.items():
                    if var.address == addr:
                        var_name = name
                        break

                if var_name:
                    # Convert hex to value
                    value = self._hex_to_value(hex_data, self.streams[var_name].size)
                    print(f"{var_name}: {value}")

        elif line.startswith("STATUS|") or line.startswith("HEARTBEAT|"):
            print(f"  {line}")
        elif line.startswith("ERROR|"):
            print(f"❌ {line}")

    def _hex_to_value(self, hex_str: str, size: int) -> str:
        """Convert hex string to human-readable value"""
        try:
            bytes_data = bytes.fromhex(hex_str)

            if size == 1:
                return str(struct.unpack('B', bytes_data)[0])
            elif size == 2:
                return str(struct.unpack('<H', bytes_data)[0])
            elif size == 4:
                # Try as both int and float
                int_val = struct.unpack('<I', bytes_data)[0]
                float_val = struct.unpack('<f', bytes_data)[0]
                return f"{int_val} (or {float_val:.3f})"
            elif size == 8:
                return str(struct.unpack('<Q', bytes_data)[0])
            else:
                return hex_str  # Just show hex for large values
        except:
            return hex_str

    def close(self):
        """Close serial connection"""
        if self.ser:
            self.ser.close()
            print("✓ Disconnected")


def interactive_mode(elf_path: str, port: str):
    """Interactive REPL for variable streaming"""
    parser = ElfParser(elf_path)
    streamer = VariableStreamer(port)
    streamer.connect()

    print("\nVariable Streaming Tool - Interactive Mode")
    print("Commands:")
    print("  watch <var_name> <rate_hz>  - Start streaming variable")
    print("  stop <var_name>             - Stop streaming variable")
    print("  list                        - List all variables")
    print("  quit                        - Exit")
    print()

    try:
        while True:
            try:
                # Read and display any incoming data
                streamer.read_and_display(timeout=0.1)

                # Get user command (non-blocking would be better, but this works)
                cmd = input(">>> ").strip()

                if not cmd:
                    continue

                parts = cmd.split()
                command = parts[0].lower()

                if command == "quit" or command == "exit":
                    break

                elif command == "list":
                    parser.list_variables()

                elif command == "watch" and len(parts) >= 3:
                    var_name = parts[1]
                    rate_hz = int(parts[2])

                    var = parser.find(var_name)
                    if var:
                        streamer.watch(var, rate_hz)
                    else:
                        print(f"❌ Variable '{var_name}' not found")

                elif command == "stop" and len(parts) >= 2:
                    var_name = parts[1]
                    streamer.stop(var_name)

                else:
                    print("❌ Unknown command or invalid syntax")

            except KeyboardInterrupt:
                print("\n(Press Ctrl+C again or type 'quit' to exit)")
                continue

    finally:
        streamer.close()


def main():
    if len(sys.argv) < 2:
        print("Usage: python var_stream.py <firmware.elf> [port]")
        print("Example: python var_stream.py target/.../memory_streamer /dev/cu.usbmodem2101")
        sys.exit(1)

    elf_path = sys.argv[1]
    port = sys.argv[2] if len(sys.argv) >= 3 else "/dev/cu.usbmodem2101"

    interactive_mode(elf_path, port)


if __name__ == "__main__":
    main()
