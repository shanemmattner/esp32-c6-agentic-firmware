#!/usr/bin/env python3
"""
UART Daemon - High-speed data streaming from ESP32-C6

Simple daemon with JSON stdin/stdout command interface.
Records binary data, exports to CSV on demand.

Commands (JSON on stdin):
  {"cmd": "start", "port": "/dev/cu.usbserial-111300", "baudrate": 115200, "record": "data.bin"}
  {"cmd": "stop"}
  {"cmd": "status"}
  {"cmd": "export", "format": "csv", "output": "data.csv"}
  {"cmd": "quit"}

Responses (JSON on stdout):
  {"status": "ok", "msg": "..."}
  {"status": "error", "msg": "..."}
  {"status": "data", "packets": 123, "bytes": 7896, "rate": 6400}
"""

import sys
import json
import serial
import time
import struct
import threading
from pathlib import Path
from dataclasses import dataclass
from typing import Optional, List

@dataclass
class TestPacket:
    """64-byte test packet from ESP32"""
    magic: int        # u32
    seq: int          # u32
    timestamp_ms: int # u64
    counter: int      # u32
    sensor_temp: int  # i32
    accel_x: int      # i16
    accel_y: int      # i16
    accel_z: int      # i16
    state: int        # u8
    checksum: int     # u32

    PACKET_SIZE = 64
    MAGIC = 0xDEADBEEF

    @classmethod
    def from_bytes(cls, data: bytes):
        """Parse 64-byte packet"""
        if len(data) != cls.PACKET_SIZE:
            raise ValueError(f"Expected {cls.PACKET_SIZE} bytes, got {len(data)}")

        # Unpack: magic, seq, timestamp, counter, temp, accel_xyz, state, padding, checksum
        # Format: I I Q I i h h h B 29x I
        fmt = '<IIQIihhh29xI'
        unpacked = struct.unpack(fmt, data)

        return cls(
            magic=unpacked[0],
            seq=unpacked[1],
            timestamp_ms=unpacked[2],
            counter=unpacked[3],
            sensor_temp=unpacked[4],
            accel_x=unpacked[5],
            accel_y=unpacked[6],
            accel_z=unpacked[7],
            state=unpacked[8],
            checksum=unpacked[9]
        )

    def validate(self) -> bool:
        """Check magic number"""
        return self.magic == self.MAGIC

    def to_dict(self):
        """Convert to dict for CSV export"""
        return {
            'seq': self.seq,
            'timestamp_ms': self.timestamp_ms,
            'counter': self.counter,
            'sensor_temp_cC': self.sensor_temp,
            'sensor_temp_C': self.sensor_temp / 100.0,
            'accel_x': self.accel_x,
            'accel_y': self.accel_y,
            'accel_z': self.accel_z,
            'state': self.state,
        }


class UartDaemon:
    def __init__(self):
        self.ser: Optional[serial.Serial] = None
        self.recording = False
        self.record_file: Optional[str] = None
        self.packets: List[TestPacket] = []
        self.bytes_received = 0
        self.packets_received = 0
        self.errors = 0
        self.running = False
        self.thread: Optional[threading.Thread] = None

    def send_response(self, status: str, **kwargs):
        """Send JSON response on stdout"""
        response = {"status": status, **kwargs}
        print(json.dumps(response), flush=True)

    def start_streaming(self, port: str, baudrate: int, record: Optional[str] = None):
        """Start UART streaming"""
        if self.running:
            self.send_response("error", msg="Already running")
            return

        try:
            self.ser = serial.Serial(port, baudrate, timeout=0.1)
            self.recording = record is not None
            self.record_file = record
            self.packets = []
            self.bytes_received = 0
            self.packets_received = 0
            self.errors = 0
            self.running = True

            # Start reader thread
            self.thread = threading.Thread(target=self._read_loop, daemon=True)
            self.thread.start()

            self.send_response("ok", msg=f"Started streaming on {port} @ {baudrate} baud")

        except Exception as e:
            self.send_response("error", msg=str(e))

    def stop_streaming(self):
        """Stop UART streaming"""
        if not self.running:
            self.send_response("error", msg="Not running")
            return

        self.running = False
        if self.thread:
            self.thread.join(timeout=2.0)

        if self.ser:
            self.ser.close()
            self.ser = None

        # Save recorded data if requested
        if self.recording and self.record_file and self.packets:
            self._save_binary(self.record_file)

        self.send_response("ok", msg="Stopped streaming",
                          packets=self.packets_received,
                          bytes=self.bytes_received,
                          errors=self.errors)

    def get_status(self):
        """Get current status"""
        self.send_response("data",
                          running=self.running,
                          packets=self.packets_received,
                          bytes=self.bytes_received,
                          errors=self.errors,
                          rate=self.bytes_received if self.running else 0)

    def export_csv(self, output: str):
        """Export recorded packets to CSV"""
        if not self.packets:
            self.send_response("error", msg="No data to export")
            return

        try:
            import csv
            with open(output, 'w', newline='') as f:
                if self.packets:
                    writer = csv.DictWriter(f, fieldnames=self.packets[0].to_dict().keys())
                    writer.writeheader()
                    for pkt in self.packets:
                        writer.writerow(pkt.to_dict())

            self.send_response("ok", msg=f"Exported {len(self.packets)} packets to {output}")

        except Exception as e:
            self.send_response("error", msg=str(e))

    def _save_binary(self, filename: str):
        """Save raw binary data"""
        # For now, just save packet data
        # Could be extended to save raw bytes if needed
        pass

    def _read_loop(self):
        """Background thread to read UART data"""
        buffer = bytearray()
        last_stats = time.time()

        while self.running:
            try:
                if self.ser and self.ser.in_waiting:
                    chunk = self.ser.read(self.ser.in_waiting)
                    buffer.extend(chunk)
                    self.bytes_received += len(chunk)

                    # Try to parse packets
                    while len(buffer) >= TestPacket.PACKET_SIZE:
                        # Look for magic number
                        if len(buffer) >= 4:
                            # Check if first 4 bytes are magic
                            magic = struct.unpack('<I', buffer[:4])[0]

                            if magic == TestPacket.MAGIC:
                                # Try to parse packet
                                if len(buffer) >= TestPacket.PACKET_SIZE:
                                    try:
                                        pkt_bytes = bytes(buffer[:TestPacket.PACKET_SIZE])
                                        pkt = TestPacket.from_bytes(pkt_bytes)

                                        if pkt.validate():
                                            self.packets.append(pkt)
                                            self.packets_received += 1
                                            buffer = buffer[TestPacket.PACKET_SIZE:]
                                        else:
                                            # Invalid packet
                                            self.errors += 1
                                            buffer = buffer[1:]  # Skip one byte
                                    except Exception:
                                        self.errors += 1
                                        buffer = buffer[1:]
                                else:
                                    break  # Wait for more data
                            else:
                                # Not magic, skip one byte
                                buffer = buffer[1:]
                        else:
                            break  # Need more data

                # Periodic stats (every second)
                now = time.time()
                if now - last_stats >= 1.0:
                    self.get_status()
                    last_stats = now

            except Exception as e:
                self.errors += 1
                time.sleep(0.01)

    def run(self):
        """Main command loop - read JSON commands from stdin"""
        self.send_response("ok", msg="UART daemon ready")

        try:
            for line in sys.stdin:
                line = line.strip()
                if not line:
                    continue

                try:
                    cmd = json.loads(line)
                    cmd_type = cmd.get("cmd")

                    if cmd_type == "start":
                        port = cmd.get("port", "/dev/cu.usbserial-111300")
                        baudrate = cmd.get("baudrate", 115200)
                        record = cmd.get("record")
                        self.start_streaming(port, baudrate, record)

                    elif cmd_type == "stop":
                        self.stop_streaming()

                    elif cmd_type == "status":
                        self.get_status()

                    elif cmd_type == "export":
                        fmt = cmd.get("format", "csv")
                        output = cmd.get("output", "data.csv")
                        if fmt == "csv":
                            self.export_csv(output)
                        else:
                            self.send_response("error", msg=f"Unknown format: {fmt}")

                    elif cmd_type == "quit":
                        if self.running:
                            self.stop_streaming()
                        self.send_response("ok", msg="Goodbye")
                        break

                    else:
                        self.send_response("error", msg=f"Unknown command: {cmd_type}")

                except json.JSONDecodeError as e:
                    self.send_response("error", msg=f"Invalid JSON: {e}")
                except Exception as e:
                    self.send_response("error", msg=str(e))

        except KeyboardInterrupt:
            if self.running:
                self.stop_streaming()

        finally:
            if self.running:
                self.stop_streaming()


if __name__ == "__main__":
    daemon = UartDaemon()
    daemon.run()
