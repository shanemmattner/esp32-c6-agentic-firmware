#!/usr/bin/env python3
import serial
import time
import sys

port = '/dev/cu.usbserial-110'
baudrate = 115200
duration = 20  # Read for 20 seconds

try:
    ser = serial.Serial(port, baudrate, timeout=0.5)
    print(f"Reading from {port} at {baudrate} baud for {duration} seconds...")
    print(f"Port is open: {ser.is_open}")
    print()

    start_time = time.time()
    bytes_read = 0
    while time.time() - start_time < duration:
        if ser.in_waiting > 0:
            data = ser.read(ser.in_waiting)
            bytes_read += len(data)
            try:
                text = data.decode('utf-8', errors='replace')
                print(text, end='', flush=True)
            except:
                print(f"[{len(data)} bytes]", flush=True)

    ser.close()
    print(f"\n\nTotal bytes read: {bytes_read}")
    print("Done reading.")
except Exception as e:
    print(f"Error: {e}")
    sys.exit(1)
