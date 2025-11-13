#!/usr/bin/env python3
"""
USB CDC Stream Parser for ESP32-C6

Parses structured logging output from USB serial port and displays
real-time statistics and events.

Usage:
    python3 stream_parser.py /dev/cu.usbmodem2101
    python3 stream_parser.py /dev/cu.usbmodem2101 --csv output.csv
    python3 stream_parser.py /dev/cu.usbmodem2101 --stats
"""

import serial
import sys
import time
import argparse
from dataclasses import dataclass, field
from typing import Dict, Optional
from datetime import datetime


@dataclass
class ParserStats:
    """Statistics for parsed messages"""

    boot_count: int = 0
    status_count: int = 0
    i2c_count: int = 0
    gpio_count: int = 0
    sensor_count: int = 0
    heartbeat_count: int = 0
    unknown_count: int = 0
    total_bytes: int = 0
    start_time: float = field(default_factory=time.time)

    def rate(self) -> float:
        """Messages per second"""
        elapsed = time.time() - self.start_time
        total = (
            self.boot_count
            + self.status_count
            + self.i2c_count
            + self.gpio_count
            + self.sensor_count
            + self.heartbeat_count
        )
        return total / elapsed if elapsed > 0 else 0

    def throughput_kbps(self) -> float:
        """Throughput in KB/s"""
        elapsed = time.time() - self.start_time
        return (self.total_bytes / elapsed / 1024) if elapsed > 0 else 0


class StreamParser:
    """Parse structured logs from ESP32-C6 USB CDC stream"""

    def __init__(
        self,
        port: str,
        baudrate: int = 115200,
        csv_file: Optional[str] = None,
        show_stats: bool = False,
    ):
        self.ser = serial.Serial(port, baudrate, timeout=1)
        self.stats = ParserStats()
        self.csv_file = csv_file
        self.show_stats = show_stats
        self.csv_handle = None

        if self.csv_file:
            self.csv_handle = open(self.csv_file, "w")
            self.csv_handle.write("timestamp,type,data\n")

    def parse_fields(self, parts: list) -> Dict[str, str]:
        """Parse pipe-delimited fields into dict"""
        fields = {}
        for part in parts[1:]:
            if "=" in part:
                key, value = part.split("=", 1)
                fields[key] = value
        return fields

    def handle_boot(self, fields: Dict[str, str]):
        """Handle BOOT message"""
        self.stats.boot_count += 1
        print(f"🚀 BOOT: {fields.get('chip', 'unknown')} v{fields.get('version', '?')}")

    def handle_status(self, fields: Dict[str, str]):
        """Handle STATUS message"""
        self.stats.status_count += 1
        msg = fields.get("msg", "")
        ready = fields.get("ready", "false")
        print(f"✓ STATUS: {msg} (ready={ready})")

    def handle_i2c(self, fields: Dict[str, str]):
        """Handle I2C transaction"""
        self.stats.i2c_count += 1
        if not self.show_stats:
            print(
                f"I2C: addr={fields.get('addr', '?')} "
                f"op={fields.get('op', '?')} "
                f"bytes={fields.get('bytes', '?')} "
                f"status={fields.get('status', '?')}"
            )

    def handle_gpio(self, fields: Dict[str, str]):
        """Handle GPIO event"""
        self.stats.gpio_count += 1
        if not self.show_stats:
            pin = fields.get("pin", "?")
            state = fields.get("state", "?")
            emoji = "🔴" if state == "High" else "⚪"
            print(f"GPIO: pin={pin} {emoji} {state}")

    def handle_sensor(self, fields: Dict[str, str]):
        """Handle sensor reading"""
        self.stats.sensor_count += 1
        if not self.show_stats:
            sensor_id = fields.get("id", "?")
            value = fields.get("value", "?")
            unit = fields.get("unit", "?")
            print(f"📊 SENSOR {sensor_id}: {value} {unit}")

    def handle_heartbeat(self, fields: Dict[str, str]):
        """Handle heartbeat"""
        self.stats.heartbeat_count += 1
        count = fields.get("count", "?")
        if self.show_stats:
            # Print statistics on heartbeat
            print(f"\n📈 Statistics (heartbeat #{count}):")
            print(f"  I2C: {self.stats.i2c_count}")
            print(f"  GPIO: {self.stats.gpio_count}")
            print(f"  Sensor: {self.stats.sensor_count}")
            print(f"  Rate: {self.stats.rate():.1f} msg/s")
            print(f"  Throughput: {self.stats.throughput_kbps():.2f} KB/s")
        else:
            print(f"💓 Heartbeat #{count}")

    def parse_line(self, line: str):
        """Parse a single structured log line"""
        if not line:
            return

        self.stats.total_bytes += len(line) + 1  # +1 for newline

        # Write to CSV if enabled
        if self.csv_handle:
            timestamp = datetime.now().isoformat()
            self.csv_handle.write(f'"{timestamp}","{line}"\n')

        parts = line.split("|")
        if len(parts) < 2:
            print(f"Raw: {line}")
            self.stats.unknown_count += 1
            return

        msg_type = parts[0]
        fields = self.parse_fields(parts)

        # Dispatch based on message type
        handlers = {
            "BOOT": self.handle_boot,
            "STATUS": self.handle_status,
            "I2C": self.handle_i2c,
            "GPIO": self.handle_gpio,
            "SENSOR": self.handle_sensor,
            "HEARTBEAT": self.handle_heartbeat,
        }

        handler = handlers.get(msg_type)
        if handler:
            handler(fields)
        else:
            print(f"Unknown: {msg_type}: {fields}")
            self.stats.unknown_count += 1

    def run(self):
        """Main parser loop"""
        print(f"📡 Listening on {self.ser.port} @ {self.ser.baudrate} baud")
        if self.csv_file:
            print(f"📝 Logging to {self.csv_file}")
        if self.show_stats:
            print(f"📊 Statistics mode enabled")
        print("Press Ctrl+C to stop\n")

        try:
            while True:
                if self.ser.in_waiting > 0:
                    line = self.ser.readline().decode("utf-8", errors="replace").strip()
                    self.parse_line(line)
        except KeyboardInterrupt:
            print("\n\n✓ Stream parser stopped")
            self.print_final_stats()
        finally:
            if self.csv_handle:
                self.csv_handle.close()
            self.ser.close()

    def print_final_stats(self):
        """Print final statistics"""
        elapsed = time.time() - self.stats.start_time
        print("\n" + "=" * 60)
        print("Final Statistics:")
        print("=" * 60)
        print(f"  Runtime: {elapsed:.1f} seconds")
        print(f"  BOOT messages: {self.stats.boot_count}")
        print(f"  STATUS messages: {self.stats.status_count}")
        print(f"  I2C transactions: {self.stats.i2c_count}")
        print(f"  GPIO events: {self.stats.gpio_count}")
        print(f"  Sensor readings: {self.stats.sensor_count}")
        print(f"  Heartbeats: {self.stats.heartbeat_count}")
        print(f"  Unknown messages: {self.stats.unknown_count}")
        print(f"  Total bytes: {self.stats.total_bytes:,}")
        print(f"  Average rate: {self.stats.rate():.1f} messages/second")
        print(f"  Throughput: {self.stats.throughput_kbps():.2f} KB/s")
        print("=" * 60)


def main():
    parser = argparse.ArgumentParser(
        description="Parse structured logs from ESP32-C6 USB CDC stream"
    )
    parser.add_argument("port", help="Serial port (e.g., /dev/cu.usbmodem2101)")
    parser.add_argument(
        "--baudrate", type=int, default=115200, help="Baud rate (default: 115200)"
    )
    parser.add_argument("--csv", help="Save raw logs to CSV file")
    parser.add_argument(
        "--stats", action="store_true", help="Show statistics mode (less verbose)"
    )

    args = parser.parse_args()

    stream = StreamParser(args.port, args.baudrate, args.csv, args.stats)
    stream.run()


if __name__ == "__main__":
    main()
