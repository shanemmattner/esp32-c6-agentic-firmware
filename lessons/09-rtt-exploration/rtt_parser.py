#!/usr/bin/env python3
"""
RTT Parser for Lesson 09: Variable Streaming Infrastructure

Parses defmt-formatted RTT output from ESP32-C6 and extracts structured telemetry data.

Usage:
    cargo run --release 2>&1 | python3 rtt_parser.py

The parser extracts:
- I2C status: write/read counts, errors, last transaction
- Config register: what was written vs what was read back
- ADC results: raw values and converted voltage
- Data quality: min/max/range/stuck detection
- State machine: current state, transitions, timing
"""

import sys
import re
from dataclasses import dataclass
from typing import Optional, Dict, Any
from collections import defaultdict
from datetime import datetime


@dataclass
class I2CData:
    """I2C communication statistics"""
    write_attempts: int = 0
    write_success: int = 0
    read_attempts: int = 0
    read_success: int = 0
    error_count: int = 0
    last_addr: Optional[int] = None
    last_value: Optional[int] = None


@dataclass
class ConfigData:
    """Configuration register state"""
    written: Optional[int] = None
    readback: Optional[int] = None
    mux: Optional[int] = None
    pga: Optional[int] = None
    mode: Optional[int] = None
    dr: Optional[int] = None
    matches: bool = True


@dataclass
class ADCData:
    """ADC conversion results"""
    raw: Optional[int] = None
    mv: Optional[int] = None
    ready: bool = False
    busy: bool = False


@dataclass
class DataQualityData:
    """Data quality metrics"""
    min_val: Optional[int] = None
    max_val: Optional[int] = None
    range_val: int = 0
    stuck_count: int = 0


@dataclass
class StateData:
    """State machine tracking"""
    state: Optional[str] = None
    changes: int = 0
    time_ms: int = 0


@dataclass
class Telemetry:
    """Complete telemetry snapshot"""
    timestamp: str
    i2c: I2CData
    config: ConfigData
    adc: ADCData
    data_quality: DataQualityData
    state: StateData


class RTTParser:
    """Parse defmt RTT output and extract structured telemetry"""

    # Regex patterns for defmt logs
    PATTERNS = {
        'i2c': re.compile(r'i2c: wr=(\d+)/(\d+) rd=(\d+)/(\d+) err=(\d+) last_addr=0x([0-9a-f]{2}) last_val=0x([0-9a-f]{4})'),
        'cfg_wr': re.compile(r'cfg_wr: wrote=0x([0-9a-f]{4}) mux=(\d+) pga=(\d+) mode=(\d+) dr=(\d+)'),
        'cfg_rb': re.compile(r'cfg_rb: read=0x([0-9a-f]{4}) mux=(\d+) pga=(\d+) mode=(\d+) match=(\d+)'),
        'adc': re.compile(r'adc: raw=0x([0-9a-f]{4}) mv=(-?\d+) busy=(\d+) ready=(\d+)'),
        'dat': re.compile(r'dat: min=0x([0-9a-f]{4}) max=0x([0-9a-f]{4}) range=(\d+) stuck=(\d+)'),
        'fsm': re.compile(r'fsm: state=(\w+) changes=(\d+) time_ms=(\d+)'),
        'sys': re.compile(r'sys: i2c_ok=(\d+) cfg_ok=(\d+) adc_mv=(-?\d+) state=(\w+)'),
    }

    def __init__(self):
        self.telemetries: list[Telemetry] = []
        self.current = self._new_telemetry()
        self.log_count = 0

    def _new_telemetry(self) -> Telemetry:
        """Create a new telemetry snapshot"""
        return Telemetry(
            timestamp=datetime.now().isoformat(),
            i2c=I2CData(),
            config=ConfigData(),
            adc=ADCData(),
            data_quality=DataQualityData(),
            state=StateData(),
        )

    def parse_line(self, line: str) -> None:
        """Parse a single RTT output line"""
        line = line.strip()
        if not line:
            return

        # Try to match each pattern
        for key, pattern in self.PATTERNS.items():
            match = pattern.search(line)
            if not match:
                continue

            if key == 'i2c':
                groups = match.groups()
                self.current.i2c = I2CData(
                    write_success=int(groups[0]),
                    write_attempts=int(groups[1]),
                    read_success=int(groups[2]),
                    read_attempts=int(groups[3]),
                    error_count=int(groups[4]),
                    last_addr=int(groups[5], 16),
                    last_value=int(groups[6], 16),
                )

            elif key == 'cfg_wr':
                groups = match.groups()
                self.current.config.written = int(groups[0], 16)
                self.current.config.mux = int(groups[1])
                self.current.config.pga = int(groups[2])
                self.current.config.mode = int(groups[3])
                self.current.config.dr = int(groups[4])

            elif key == 'cfg_rb':
                groups = match.groups()
                self.current.config.readback = int(groups[0], 16)
                self.current.config.mux = int(groups[1])
                self.current.config.pga = int(groups[2])
                self.current.config.mode = int(groups[3])
                self.current.config.matches = bool(int(groups[4]))

            elif key == 'adc':
                groups = match.groups()
                self.current.adc = ADCData(
                    raw=int(groups[0], 16),
                    mv=int(groups[1]),
                    busy=bool(int(groups[2])),
                    ready=bool(int(groups[3])),
                )

            elif key == 'dat':
                groups = match.groups()
                self.current.data_quality = DataQualityData(
                    min_val=int(groups[0], 16),
                    max_val=int(groups[1], 16),
                    range_val=int(groups[2]),
                    stuck_count=int(groups[3]),
                )

            elif key == 'fsm':
                groups = match.groups()
                self.current.state = StateData(
                    state=groups[0],
                    changes=int(groups[1]),
                    time_ms=int(groups[2]),
                )

            elif key == 'sys':
                # Critical state log - this indicates a complete telemetry cycle
                groups = match.groups()
                # Log this as a complete snapshot
                self.telemetries.append(self.current)
                self.current = self._new_telemetry()
                self.log_count += 1
                # Print formatted output
                self._print_telemetry(self.telemetries[-1])

    def _print_telemetry(self, t: Telemetry) -> None:
        """Pretty-print a telemetry snapshot"""
        ts = datetime.fromisoformat(t.timestamp).strftime("%H:%M:%S")

        print(f"\n[{ts}] Telemetry #{self.log_count}")
        print("=" * 60)

        # I2C status
        if t.i2c.write_attempts > 0:
            wr_rate = (t.i2c.write_success * 100) // t.i2c.write_attempts
            print(f"I2C Writes:  {t.i2c.write_success}/{t.i2c.write_attempts} ({wr_rate}%)")
        if t.i2c.read_attempts > 0:
            rd_rate = (t.i2c.read_success * 100) // t.i2c.read_attempts
            print(f"I2C Reads:   {t.i2c.read_success}/{t.i2c.read_attempts} ({rd_rate}%)")
        if t.i2c.error_count > 0:
            print(f"I2C Errors:  {t.i2c.error_count} ⚠️")
        if t.i2c.last_addr is not None:
            print(f"Last I2C TX: addr=0x{t.i2c.last_addr:02x} val=0x{t.i2c.last_value:04x}")

        # Config register
        if t.config.written is not None:
            match_str = "✓" if t.config.matches else "✗ MISMATCH"
            print(f"\nConfig Register:")
            print(f"  Written:  0x{t.config.written:04x} (mux={t.config.mux} pga={t.config.pga} mode={t.config.mode} dr={t.config.dr})")
            if t.config.readback is not None:
                print(f"  Readback: 0x{t.config.readback:04x} {match_str}")

        # ADC data
        if t.adc.raw is not None:
            print(f"\nADC Results:")
            print(f"  Raw:     0x{t.adc.raw:04x} ({t.adc.raw})")
            print(f"  Voltage: {t.adc.mv} mV")
            status = "ready" if t.adc.ready else "busy" if t.adc.busy else "idle"
            print(f"  Status:  {status}")

        # Data quality
        if t.data_quality.min_val is not None:
            print(f"\nData Quality:")
            print(f"  Range:  0x{t.data_quality.min_val:04x}..0x{t.data_quality.max_val:04x} (delta={t.data_quality.range_val})")
            if t.data_quality.stuck_count > 0:
                print(f"  Stuck:  {t.data_quality.stuck_count} samples ⚠️")

        # State machine
        if t.state.state:
            print(f"\nState Machine:")
            print(f"  State:   {t.state.state}")
            print(f"  Changes: {t.state.changes}")
            print(f"  Time:    {t.state.time_ms} ms")

    def print_summary(self) -> None:
        """Print summary statistics"""
        if not self.telemetries:
            return

        print("\n" + "=" * 60)
        print(f"RTT Parse Summary: {len(self.telemetries)} complete telemetry cycles")
        print("=" * 60)

        # Calculate statistics
        total_i2c_errors = sum(t.i2c.error_count for t in self.telemetries)
        config_mismatches = sum(1 for t in self.telemetries if not t.config.matches)
        total_stuck = sum(t.data_quality.stuck_count for t in self.telemetries)

        print(f"\nI2C Health:")
        print(f"  Total errors: {total_i2c_errors}")

        print(f"\nConfig Register:")
        print(f"  Mismatches: {config_mismatches}")

        print(f"\nADC Data Quality:")
        print(f"  Total stuck samples: {total_stuck}")

        if self.telemetries:
            last = self.telemetries[-1]
            if last.adc.raw:
                print(f"\nLast ADC Reading:")
                print(f"  Raw: 0x{last.adc.raw:04x}")
                print(f"  mV:  {last.adc.mv}")


def main():
    """Main entry point"""
    parser = RTTParser()

    try:
        for line in sys.stdin:
            parser.parse_line(line)
    except KeyboardInterrupt:
        pass
    finally:
        parser.print_summary()


if __name__ == '__main__':
    main()
