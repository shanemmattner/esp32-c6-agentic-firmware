# Lesson 08: USB CDC High-Speed Data Streaming

Stream structured sensor data at high speed using USB CDC (virtual serial port) for real-time monitoring and analysis.

## Learning Objectives

- High-speed USB CDC streaming (up to 1.5 MB/s)
- Structured logging with machine-parseable format
- Real-time data visualization with Python
- Performance analysis and bandwidth optimization

## Hardware Requirements

- ESP32-C6 development board
- USB-C cable (for power and data)

**No external components needed** - uses built-in USB CDC

## What You'll Learn

This lesson demonstrates:
- Structured logging using pipe-delimited format
- Custom Display trait implementations for data types
- USB CDC streaming performance characteristics
- Python parsing and real-time visualization
- Bandwidth budgeting and throughput analysis

## Build & Flash

```bash
cd lessons/08-usb-cdc-streaming

# Build
cargo build --release

# Flash and monitor
cargo run --release
```

## Expected Output

When you flash and run this lesson, you should see:

```
BOOT|version=1.0.0|chip=ESP32-C6
STATUS|msg=Initialization complete|ready=true
I2C|addr=0x68|op=Read|bytes=6|status=Success|ts=10
GPIO|pin=8|state=Low|ts=250
SENSOR|id=1|value=2530|unit=centi-C|ts=500
HEARTBEAT|count=1|ts=1000
I2C|addr=0x68|op=Read|bytes=6|status=Success|ts=1010
...
```

## Python Parser

Parse and visualize streaming data with the included Python tools.

### Installation

```bash
pip install pyserial matplotlib
```

### Basic Usage

```bash
# Real-time parsing
python3 stream_parser.py /dev/cu.usbmodem2101

# Statistics mode
python3 stream_parser.py /dev/cu.usbmodem2101 --stats

# Log to CSV
python3 stream_parser.py /dev/cu.usbmodem2101 --csv output.csv

# Real-time plotting
python3 plot_sensor_data.py /dev/cu.usbmodem2101
```

## Performance

### Bandwidth Budget

- **USB CDC capacity:** 1.5 MB/s (USB Full Speed)
- **Typical message size:** 60 bytes
- **Sustainable rate:** 1000+ messages/second
- **Throughput at 100 Hz:** ~6 KB/s (0.4% of capacity)

### Tested Performance

| Loop Rate | Messages/sec | Throughput | CPU Load |
|-----------|--------------|------------|----------|
| 100 Hz | ~20 | 1-3 KB/s | Low |
| 1000 Hz | ~200 | 10-30 KB/s | Moderate |

**Plenty of headroom for additional sensors and higher sample rates.**

## Code Structure

### Firmware (`src/`)

- `lib.rs` - Structured data types with Display traits
  - `I2cTransaction` - I2C bus events
  - `GpioEvent` - GPIO state changes
  - `SensorReading` - Generic sensor data
  - `BootInfo` - System startup info

- `bin/main.rs` - Main firmware
  - USB CDC initialization (automatic with esp-println)
  - Event simulation loop
  - Structured logging output

### Python Tools

- `stream_parser.py` - Parse and display streaming data
- `plot_sensor_data.py` - Real-time plotting

## Key Concepts

### Structured Logging Format

All messages follow this format:
```
TYPE|field1=val1|field2=val2|...
```

**Benefits:**
- Machine-parseable (easy for Python/scripts)
- Human-readable (debugging friendly)
- Self-documenting (field names included)
- Extensible (add fields without breaking parsers)

### Custom Display Traits

```rust
impl fmt::Display for I2cTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "I2C|addr=0x{:02x}|op={:?}|...", ...)
    }
}
```

**Advantage:** `println!("{}", event)` automatically formats correctly

### USB CDC vs RTT

| Feature | USB CDC | RTT |
|---------|---------|-----|
| Speed | 1.5 MB/s | 1-10 MB/s |
| Setup | USB cable only | JTAG probe required |
| Compatibility | Universal | Requires probe-rs |
| Blocking | Minimal | Non-blocking |
| **Status** | ✅ Works now | ❌ macOS issues |

## Troubleshooting

| Issue | Solution |
|-------|----------|
| No serial output | Check USB cable supports data (not just power) |
| Parser can't find port | Run `ls /dev/cu.usbmodem*` to find correct port |
| Garbled output | Ensure baudrate matches (115200) |
| Parser crashes | Install pyserial: `pip install pyserial` |

## Extending This Lesson

**Add real sensors:**
- Replace simulated data with actual I2C/SPI sensors
- Use real GPIO interrupts
- Add ADC readings

**Increase complexity:**
- Add more data types (SPI, UART, ADC)
- Implement ring buffer for burst data
- Add data compression
- Create multi-channel streams

**Analysis tools:**
- Export to time-series database
- Create Grafana dashboard
- Add statistical analysis
- Implement anomaly detection

## Next Steps

- **Lesson 09:** (TBD - depends on RTT status)
- Experiment: Add your own sensors and data types
- Challenge: Achieve 1000 Hz sustained streaming with multiple sensors

## References

- [ESP32-C6 USB Serial JTAG](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c6/api-guides/usb-serial-jtag-console.html)
- [esp-println Documentation](https://docs.esp-rs.org/esp-println/)
- [USB CDC Class Specification](https://www.usb.org/document-library/class-definitions-communication-devices-12)
