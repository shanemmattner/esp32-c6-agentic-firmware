#!/usr/bin/env python3
"""
Safe UART reader for Claude Code interaction.
Reads from UART with timeout and outputs to stdout.
"""

import sys
import serial
import time

def main():
    if len(sys.argv) < 2:
        print("Usage: read_uart.py <port> [duration_seconds] [baud_rate]")
        print("Example: read_uart.py /dev/cu.usbserial-FT58PFX4 5 921600")
        sys.exit(1)

    port = sys.argv[1]
    duration = int(sys.argv[2]) if len(sys.argv) > 2 else 3
    baud = int(sys.argv[3]) if len(sys.argv) > 3 else 115200

    try:
        ser = serial.Serial(port, baud, timeout=0.1)
        print(f"Reading from {port} at {baud} baud for {duration} seconds...")
        print("-" * 60)

        start_time = time.time()
        lines_read = 0

        while (time.time() - start_time) < duration:
            if ser.in_waiting > 0:
                try:
                    line = ser.readline().decode('utf-8', errors='ignore')
                    if line:
                        print(line, end='')
                        lines_read += 1
                except Exception as e:
                    print(f"[Decode error: {e}]", file=sys.stderr)
            time.sleep(0.01)

        print("-" * 60)
        print(f"Read {lines_read} lines in {duration} seconds")
        ser.close()

    except serial.SerialException as e:
        print(f"Error opening {port}: {e}", file=sys.stderr)
        sys.exit(1)
    except KeyboardInterrupt:
        print("\nInterrupted by user")
        sys.exit(0)

if __name__ == "__main__":
    main()
