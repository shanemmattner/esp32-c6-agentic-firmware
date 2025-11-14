#!/usr/bin/env python3
"""
Real-time plotting of sensor data from USB CDC stream

Usage:
    python3 plot_sensor_data.py /dev/cu.usbmodem2101
"""

import serial
import sys
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
from collections import deque
import argparse


class SensorPlotter:
    def __init__(self, port: str, baudrate: int = 115200, max_points: int = 100):
        self.ser = serial.Serial(port, baudrate, timeout=1)
        self.max_points = max_points

        # Data buffers
        self.timestamps = deque(maxlen=max_points)
        self.sensor_values = deque(maxlen=max_points)

        # Setup plot
        self.fig, self.ax = plt.subplots()
        (self.line,) = self.ax.plot([], [], "b-", linewidth=2)
        self.ax.set_xlabel("Time (s)")
        self.ax.set_ylabel("Sensor Value")
        self.ax.set_title("ESP32-C6 Real-Time Sensor Data")
        self.ax.grid(True)

    def parse_line(self, line: str):
        """Parse sensor data from line"""
        if not line.startswith("SENSOR|"):
            return None, None

        parts = line.split("|")
        fields = {}
        for part in parts[1:]:
            if "=" in part:
                key, value = part.split("=", 1)
                fields[key] = value

        try:
            timestamp = int(fields.get("ts", 0)) / 1000.0  # Convert ms to seconds
            value = int(fields.get("value", 0))
            return timestamp, value
        except (ValueError, KeyError):
            return None, None

    def update_plot(self, frame):
        """Update plot with new data"""
        if self.ser.in_waiting > 0:
            line = self.ser.readline().decode("utf-8", errors="replace").strip()
            timestamp, value = self.parse_line(line)

            if timestamp is not None:
                self.timestamps.append(timestamp)
                self.sensor_values.append(value)

                if len(self.timestamps) > 1:
                    # Normalize timestamps to start at 0
                    t0 = self.timestamps[0]
                    times = [t - t0 for t in self.timestamps]

                    self.line.set_data(times, list(self.sensor_values))
                    self.ax.relim()
                    self.ax.autoscale_view()

        return (self.line,)

    def run(self):
        """Start real-time plotting"""
        print(f"📊 Starting real-time plot from {self.ser.port}")
        print("Close plot window to stop")

        ani = FuncAnimation(self.fig, self.update_plot, interval=50, blit=True)
        plt.show()

        self.ser.close()


def main():
    parser = argparse.ArgumentParser(description="Real-time sensor plotting")
    parser.add_argument("port", help="Serial port")
    parser.add_argument("--baudrate", type=int, default=115200)
    parser.add_argument("--points", type=int, default=100, help="Max data points")

    args = parser.parse_args()

    try:
        plotter = SensorPlotter(args.port, args.baudrate, args.points)
        plotter.run()
    except KeyboardInterrupt:
        print("\n✓ Plotting stopped")


if __name__ == "__main__":
    main()
