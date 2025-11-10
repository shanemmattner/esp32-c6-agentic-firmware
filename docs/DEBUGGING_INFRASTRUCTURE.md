# Debugging Infrastructure & Tooling
## Making Remote ESP32-C6 Development Easier

---

## 🔍 Current Pain Points

1. **Serial Monitoring**: `espflash monitor` has input reader issues on macOS
2. **Remote Access**: Flashing/monitoring through RPi adds complexity
3. **Log Collection**: Hard to capture and analyze output programmatically
4. **Manual Steps**: Build → Flash → Monitor requires multiple commands
5. **Data Analysis**: No easy way to parse/filter/search logs
6. **Testing**: No automated validation of firmware behavior

---

## 💡 Proposed Solutions

### 1. **Python Serial Monitor** (Quick Win ⭐)
Replace `espflash monitor` with a robust Python script:

```python
# scripts/monitor.py
import serial
import sys
import argparse
from datetime import datetime

class SerialMonitor:
    def __init__(self, port, baudrate=115200):
        self.serial = serial.Serial(port, baudrate, timeout=1)
        self.log_file = f"logs/serial_{datetime.now().isoformat()}.log"

    def run(self):
        """Read and display serial output with logging"""
        try:
            with open(self.log_file, 'w') as f:
                while True:
                    if self.serial.in_waiting:
                        line = self.serial.readline().decode('utf-8', errors='ignore')
                        print(line, end='')
                        f.write(line)
                        f.flush()
        except KeyboardInterrupt:
            print(f"\n✓ Log saved to {self.log_file}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('--port', default='/dev/cu.usbserial-110')
    parser.add_argument('--baud', type=int, default=115200)
    args = parser.parse_args()

    monitor = SerialMonitor(args.port, args.baud)
    monitor.run()
```

**Benefits:**
- ✅ Works on macOS without issues
- ✅ Automatic log file creation
- ✅ Easy to extend with filtering/parsing
- ✅ Can run in background

---

### 2. **Unified Build/Flash/Monitor Script** (Automation ⭐⭐)

```bash
#!/bin/bash
# scripts/build-flash-monitor.sh

LESSON=${1:-01-blinky}
PORT=${2:-/dev/cu.usbserial-110}
BAUD=${3:-115200}

echo "🔨 Building lesson: $LESSON"
cd lessons/$LESSON
cargo build --release || exit 1

echo "📝 Flashing to $PORT..."
espflash flash --monitor \
    target/riscv32imac-unknown-none-elf/release/$LESSON \
    --port $PORT || exit 1

echo "👀 Monitoring output..."
python3 ../../scripts/monitor.py --port $PORT --baud $BAUD
```

**Usage:**
```bash
./scripts/build-flash-monitor.sh 01-blinky /dev/cu.usbserial-110
```

**Benefits:**
- ✅ One command instead of three
- ✅ Fails fast if build fails
- ✅ Automatic log creation
- ✅ Easy to customize

---

### 3. **Log Parser & Analyzer** (Data Intelligence ⭐⭐)

```python
# scripts/analyze_logs.py
import re
import sys
from collections import defaultdict
from datetime import datetime

class LogAnalyzer:
    def __init__(self, log_file):
        self.log_file = log_file
        self.entries = self.parse_log()

    def parse_log(self):
        """Parse structured logs from device output"""
        entries = {
            'INFO': [],
            'WARN': [],
            'ERROR': [],
            'DEBUG': [],
        }

        with open(self.log_file) as f:
            for line in f:
                for level in entries.keys():
                    if f" {level} " in line:
                        entries[level].append(line.strip())

        return entries

    def print_summary(self):
        """Print log summary"""
        print("\n📊 Log Summary")
        print("=" * 60)
        for level, lines in self.entries.items():
            print(f"{level:6} : {len(lines):3} entries")

        if self.entries['ERROR']:
            print("\n⚠️  Errors Found:")
            for line in self.entries['ERROR'][:5]:
                print(f"  {line}")

        print("=" * 60)

    def export_csv(self, output_file):
        """Export logs to CSV for analysis"""
        import csv
        with open(output_file, 'w', newline='') as f:
            writer = csv.writer(f)
            writer.writerow(['Timestamp', 'Level', 'Message'])
            # Parse and write entries...
```

**Usage:**
```bash
python3 scripts/analyze_logs.py logs/serial_2025-11-10T02:19:09.log
```

**Output:**
```
📊 Log Summary
============================================================
INFO   :  12 entries
WARN   :   2 entries
ERROR  :   0 entries
DEBUG  :   8 entries
============================================================
```

---

### 4. **SSH Remote Flashing** (Remote Development 🌐)

For your RPi setup, add SSH forwarding:

```bash
#!/bin/bash
# scripts/remote-flash.sh

REMOTE_USER=${1:-pi}
REMOTE_HOST=${2:-raspberrypi.local}
LESSON=${3:-01-blinky}
REMOTE_PORT=${4:-/dev/ttyUSB0}

echo "📦 Building locally..."
cd lessons/$LESSON
cargo build --release

echo "🚀 Copying to remote..."
scp -r target/riscv32imac-unknown-none-elf/release/$LESSON \
    $REMOTE_USER@$REMOTE_HOST:/tmp/

echo "⚡ Flashing remotely..."
ssh $REMOTE_USER@$REMOTE_HOST \
    "espflash flash /tmp/$LESSON --port $REMOTE_PORT"

echo "👀 Monitoring remotely..."
ssh $REMOTE_USER@$REMOTE_HOST \
    "python3 /path/to/monitor.py --port $REMOTE_PORT" | tee logs/remote.log
```

**Benefits:**
- ✅ Build on fast laptop
- ✅ Flash through RPi
- ✅ Logs stream back to laptop
- ✅ No need to move hardware

---

### 5. **Automated Testing Framework** (CI/CD for Hardware 🧪)

```python
# scripts/test_firmware.py
import subprocess
import time
import re

class FirmwareTest:
    def __init__(self, port, timeout=30):
        self.port = port
        self.timeout = timeout

    def run_test(self, name, expected_patterns):
        """
        Run firmware and check for expected log patterns

        Args:
            name: Test name
            expected_patterns: List of regex patterns to find in logs

        Returns:
            bool: True if all patterns found
        """
        print(f"🧪 Running test: {name}")

        # Capture output
        output = self.capture_output()

        # Check patterns
        results = {}
        for pattern in expected_patterns:
            found = bool(re.search(pattern, output))
            results[pattern] = found
            status = "✓" if found else "✗"
            print(f"  {status} {pattern}")

        passed = all(results.values())
        print(f"{'✓' if passed else '✗'} Test {'PASSED' if passed else 'FAILED'}")
        return passed

    def capture_output(self):
        """Capture serial output"""
        # Use our Python monitor to capture
        return subprocess.check_output([
            'python3', 'scripts/monitor.py',
            '--port', self.port,
            '--timeout', str(self.timeout)
        ]).decode()

# Example test suite
if __name__ == "__main__":
    tester = FirmwareTest('/dev/cu.usbserial-110')

    # Test Lesson 01: Blinky
    tester.run_test("Blinky Initialization", [
        r"🚀 Starting Blinky",
        r"✓ HAL initialized",
        r"✓ GPIO13 configured",
        r"💡 Entering blink loop",
    ])

    # Test Lesson 02: Button Input
    tester.run_test("Button Debouncing", [
        r"Button pressed",
        r"Debounce timer",
        r"Button released",
    ])
```

**Usage:**
```bash
python3 scripts/test_firmware.py
```

**Output:**
```
🧪 Running test: Blinky Initialization
  ✓ 🚀 Starting Blinky
  ✓ ✓ HAL initialized
  ✓ ✓ GPIO13 configured
  ✓ 💡 Entering blink loop
✓ Test PASSED
```

---

### 6. **Log Collection & Cloud Storage** (Long-term Data 📊)

For tracking performance over time:

```python
# scripts/upload_logs.py
import os
import json
from datetime import datetime
import hashlib

class LogCollector:
    def __init__(self, logs_dir='logs'):
        self.logs_dir = logs_dir
        self.metadata_file = 'logs/metadata.json'

    def collect_and_catalog(self):
        """Collect all logs and create metadata"""
        metadata = {}

        for log_file in os.listdir(self.logs_dir):
            if log_file.endswith('.log'):
                path = os.path.join(self.logs_dir, log_file)

                # Calculate hash for deduplication
                with open(path, 'rb') as f:
                    file_hash = hashlib.sha256(f.read()).hexdigest()

                metadata[log_file] = {
                    'timestamp': os.path.getmtime(path),
                    'size': os.path.getsize(path),
                    'hash': file_hash,
                    'path': path,
                }

        # Save metadata
        with open(self.metadata_file, 'w') as f:
            json.dump(metadata, f, indent=2)

        return metadata

    def upload_to_s3(self, bucket_name):
        """Upload logs to AWS S3 for archival"""
        # Optional: requires boto3
        import boto3
        s3 = boto3.client('s3')

        for log_file in os.listdir(self.logs_dir):
            if log_file.endswith('.log'):
                s3.upload_file(
                    os.path.join(self.logs_dir, log_file),
                    bucket_name,
                    f"logs/{datetime.now().isoformat()}/{log_file}"
                )
```

---

### 7. **Interactive Dashboard** (Live Monitoring 📈)

Using Streamlit for real-time log visualization:

```python
# scripts/dashboard.py
import streamlit as st
import pandas as pd
import glob
from datetime import datetime

st.set_page_config(page_title="ESP32-C6 Monitor", layout="wide")

st.title("📊 ESP32-C6 Firmware Dashboard")

# Sidebar: Select log file
log_files = sorted(glob.glob('logs/*.log'), reverse=True)
selected_log = st.sidebar.selectbox("Select Log File", log_files)

if selected_log:
    with open(selected_log) as f:
        content = f.read()

    # Display raw log
    st.subheader("Raw Output")
    st.text_area("Serial Output", content, height=400)

    # Parse and display stats
    st.subheader("Statistics")
    info_count = content.count("[INFO]")
    warn_count = content.count("[WARN]")
    error_count = content.count("[ERROR]")

    col1, col2, col3 = st.columns(3)
    col1.metric("Info Messages", info_count)
    col2.metric("Warnings", warn_count)
    col3.metric("Errors", error_count)

    # Download button
    st.download_button(
        label="Download Log",
        data=content,
        file_name=os.path.basename(selected_log)
    )
```

**Run with:**
```bash
streamlit run scripts/dashboard.py
```

Opens interactive web UI at `http://localhost:8501`

---

## 🛠️ Implementation Roadmap

### Phase 1: Core Tooling (This Week)
- [ ] Create `scripts/monitor.py` (Python serial monitor)
- [ ] Create `scripts/build-flash-monitor.sh` (unified script)
- [ ] Create `scripts/analyze_logs.py` (log parser)
- [ ] Add `logs/` directory to `.gitignore`

### Phase 2: Testing & Automation (Next Week)
- [ ] Create `scripts/test_firmware.py` (test framework)
- [ ] Add GitHub Actions workflow for CI/CD
- [ ] Create `tests/` directory for test cases

### Phase 3: Remote Development (Later)
- [ ] SSH remote flashing script
- [ ] Log aggregation system
- [ ] Cloud storage integration

### Phase 4: Visualization (Nice to Have)
- [ ] Streamlit dashboard
- [ ] Performance graphs
- [ ] Historical trend analysis

---

## 📁 Recommended Directory Structure

```
esp32-c6-agentic-firmware/
├── scripts/
│   ├── monitor.py                 # Python serial monitor
│   ├── build-flash-monitor.sh     # Unified build/flash/monitor
│   ├── analyze_logs.py            # Log analyzer
│   ├── test_firmware.py           # Automated testing
│   ├── remote-flash.sh            # SSH remote flashing
│   ├── upload_logs.py             # Log collection
│   └── dashboard.py               # Streamlit dashboard
├── tests/
│   ├── test_lesson_01.py          # Lesson 01 tests
│   ├── test_lesson_02.py          # Lesson 02 tests
│   └── conftest.py                # Test fixtures
├── logs/                          # Serial output logs (gitignored)
│   └── .gitkeep
├── .gitignore                     # Ignore logs/
└── ...existing files...
```

---

## 🚀 Quick Start

```bash
# 1. Create scripts directory
mkdir -p scripts tests logs

# 2. Install Python dependencies
pip install pyserial streamlit

# 3. Copy the scripts from this document
# (Or we can create them together)

# 4. Make executable
chmod +x scripts/build-flash-monitor.sh
chmod +x scripts/monitor.py

# 5. Use it!
./scripts/build-flash-monitor.sh 01-blinky
```

---

## 🎯 Benefits Summary

| Tool | Benefit | Priority |
|------|---------|----------|
| Python Monitor | Reliable serial I/O on macOS | ⭐⭐⭐ |
| Build/Flash Script | Single command workflow | ⭐⭐⭐ |
| Log Analyzer | Programmatic log parsing | ⭐⭐ |
| Test Framework | Automated hardware testing | ⭐⭐ |
| Remote Flashing | Easy RPi integration | ⭐⭐ |
| Dashboard | Visual monitoring | ⭐ |

---

## 💭 Questions for You

1. **RPi Integration**: Should we prioritize remote SSH flashing?
2. **Cloud Storage**: Do you want logs stored in S3 or just locally?
3. **CI/CD**: Should we set up GitHub Actions for automated builds?
4. **Testing**: What patterns should firmware tests check for?
5. **Dashboard**: Would a web UI be useful or overkill?

---

**This infrastructure will pay dividends as you create more lessons! 🚀**
