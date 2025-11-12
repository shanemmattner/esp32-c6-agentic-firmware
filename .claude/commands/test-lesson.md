# Test Lesson - Unified Hardware Testing Command

Execute comprehensive hardware tests for any ESP32-C6 lesson with automatic hardware detection and robust error recovery.

**Usage:**
```bash
/test-lesson <lesson_number> [mode]
```

**Arguments:**
- `lesson_number`: Lesson to test (e.g., "07", "08", "03")
- `mode` (optional): "quick" or "full" (defaults to "quick")

**Examples:**
```bash
/test-lesson 07           # Quick test of lesson 07
/test-lesson 08 full      # Full test of lesson 08
/test-lesson 03 quick     # Quick test of lesson 03
```

---

## Your Task

{{argument}} <!-- e.g., "07", "08 full", "03" -->

Execute hardware validation tests for the specified lesson according to its TEST.md specification.

---

## Understanding Serial Output Channels

ESP32-C6 firmware can output to multiple serial interfaces. Understanding these is critical for boot verification:

### 1. USB CDC (Built-in USB-JTAG)
- **Device:** `/dev/cu.usbmodem*` (macOS) or `/dev/ttyACM*` (Linux)
- **Used for:**
  - Debug logs (`info!()`, `warn!()`, `error!()` macros via `esp_println`)
  - Flashing firmware (with `espflash`)
  - Built-in JTAG debugging
- **Baud rate:** 115200 (default)
- **Available:** Only when ESP32-C6 connected via USB cable

### 2. UART (Dedicated GPIO Pins)
- **Device:** `/dev/cu.usbserial*` (macOS) or `/dev/ttyUSB*` (Linux)
- **Used for:**
  - Interactive terminal/CLI (if firmware implements UART interface)
  - Custom serial protocols
- **Configuration:** Varies by lesson (check `main.rs` for GPIO pins and baud rate)
- **Common pins:** GPIO15 (TX), GPIO23 (RX)
- **Available:** When external UART adapter connected to ESP32-C6 UART pins

### 3. RTT (Real-Time Transfer via JTAG)
- **Device:** N/A (uses JTAG debug interface)
- **Used for:**
  - High-speed debug output
  - No GPIO pins required
  - Zero-overhead logging
- **Available:** When JTAG probe connected
- **Access via:** `probe-rs run --chip esp32c6` or `probe-rs attach --rtt`

**Boot verification strategy:**
1. Auto-detect all available serial ports
2. Test each port for terminal activity (send `help`, check for response)
3. If USB CDC available, check for `esp_println` debug output
4. If JTAG available, check for RTT output

**Recommendation:** Lessons should document in TEST.md which serial interface(s) they use.

---

## Step 0: Setup - Parse Arguments and Detect Hardware

**CRITICAL:** This setup script now supports multiple hardware configurations:
- USB CDC only (most common)
- JTAG probe only (for debugging without USB)
- Both USB CDC + JTAG probe

Execute this comprehensive setup script:

```bash
cat > /tmp/test_setup.sh << 'SCRIPT'
#!/bin/bash
set -e

# Parse arguments
ARGS="{{argument}}"

# Extract lesson number (first argument)
LESSON_NUM=$(echo "$ARGS" | awk '{print $1}')
if [ -z "$LESSON_NUM" ]; then
    echo "✗ ERROR: No lesson number provided"
    echo "Usage: /test-lesson <lesson_number> [mode]"
    echo "Example: /test-lesson 07"
    exit 1
fi

# Normalize lesson number (07 -> 07, 7 -> 07)
LESSON_NUM=$(printf "%02d" "$LESSON_NUM" 2>/dev/null || echo "$LESSON_NUM")

# Extract mode (second argument, defaults to "quick")
MODE=$(echo "$ARGS" | awk '{print $2}')
MODE=${MODE:-quick}

# Find project root and lessons directory
PROJECT_ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
LESSONS_DIR="$PROJECT_ROOT/lessons"

if [ ! -d "$LESSONS_DIR" ]; then
    echo "✗ ERROR: Lessons directory not found at $LESSONS_DIR"
    echo "Are you running this from the project root?"
    exit 1
fi

# Find lesson directory
LESSON_DIR=$(find "$LESSONS_DIR" -maxdepth 1 -type d -name "${LESSON_NUM}-*" | head -1)

if [ -z "$LESSON_DIR" ]; then
    echo "✗ ERROR: Lesson $LESSON_NUM not found"
    echo "Available lessons:"
    ls -1 "$LESSONS_DIR" | grep "^[0-9]"
    exit 1
fi

LESSON_NAME=$(basename "$LESSON_DIR")

echo "=== Test Lesson $LESSON_NUM: $LESSON_NAME ==="
echo "Mode: $MODE"
echo "Directory: $LESSON_DIR"
echo ""

# Save lesson metadata to files (file-based state management)
echo "$LESSON_DIR" > /tmp/test_lesson_dir.txt
echo "$LESSON_NUM" > /tmp/test_lesson_num.txt
echo "$MODE" > /tmp/test_mode.txt
echo "$LESSON_NAME" > /tmp/test_lesson_name.txt

# Detect and save hardware configuration
echo "=== Detecting Hardware ==="

# Detect USB CDC port (for flashing with espflash)
USB_CDC_PORT=$(ls /dev/cu.usbmodem* 2>/dev/null | head -1)
if [ -n "$USB_CDC_PORT" ]; then
    echo "✓ USB CDC: $USB_CDC_PORT"
    echo "$USB_CDC_PORT" > /tmp/usb_cdc_port.txt
    echo "espflash" > /tmp/flash_method.txt
else
    echo "⚠ USB CDC: not detected"
    echo "" > /tmp/usb_cdc_port.txt
fi

# Detect ESP JTAG probe (if available)
ESP_PROBE=$(probe-rs list 2>&1 | grep -i "esp.*jtag" | grep -oE '[0-9a-fA-F]{4}:[0-9a-fA-F]{4}(:[0-9A-F:]+)?' | head -1)
if [ -n "$ESP_PROBE" ]; then
    echo "✓ ESP Probe: $ESP_PROBE"
    echo "$ESP_PROBE" > /tmp/esp_probe.txt
    echo "--probe $ESP_PROBE" > /tmp/probe_arg.txt

    # If no USB CDC, use probe-rs for flashing
    if [ -z "$USB_CDC_PORT" ]; then
        echo "  → Will use probe-rs for flashing (no USB CDC available)"
        echo "probe-rs" > /tmp/flash_method.txt
    fi
else
    echo "⚠ ESP Probe: not detected"
    echo "" > /tmp/esp_probe.txt
    echo "" > /tmp/probe_arg.txt
fi

# Check if we have any flashing method
FLASH_METHOD=$(cat /tmp/flash_method.txt 2>/dev/null)
if [ -z "$FLASH_METHOD" ]; then
    echo ""
    echo "✗ ERROR: No flashing method available"
    echo "  - USB CDC not found (no /dev/cu.usbmodem*)"
    echo "  - ESP JTAG probe not found"
    echo ""
    echo "Please connect ESP32-C6 via:"
    echo "  1. USB cable (for USB CDC + built-in JTAG), OR"
    echo "  2. External JTAG probe (ESP-Prog, J-Link, etc.)"
    exit 1
fi

# Detect UART adapters (for terminal interfaces)
UART_PORTS=$(ls /dev/cu.usbserial* 2>/dev/null | tr '\n' ' ')
if [ -n "$UART_PORTS" ]; then
    UART_COUNT=$(echo "$UART_PORTS" | wc -w | tr -d ' ')
    echo "✓ UART adapters: $UART_COUNT detected"
    echo "$UART_PORTS" > /tmp/uart_ports.txt
else
    echo "⚠ UART: not detected (optional)"
    echo "" > /tmp/uart_ports.txt
fi

# Detect binary name from Cargo.toml
cd "$LESSON_DIR"
BINARY_NAME=$(grep -A1 '\[\[bin\]\]' Cargo.toml 2>/dev/null | grep 'name' | cut -d'"' -f2 | head -1)
if [ -z "$BINARY_NAME" ]; then
    BINARY_NAME="main"  # Fallback to default
fi
echo "Binary name: $BINARY_NAME"
echo "$BINARY_NAME" > /tmp/binary_name.txt

# Define standard paths
TARGET_DIR="target/riscv32imac-unknown-none-elf/release"
echo "$TARGET_DIR" > /tmp/target_dir.txt

# Check for TEST.md
if [ ! -f "$LESSON_DIR/TEST.md" ]; then
    echo ""
    echo "⚠ WARNING: No TEST.md found for this lesson"
    echo "Will perform generic infrastructure tests"
    echo "generic" > /tmp/test_spec_type.txt
else
    echo ""
    echo "✓ Found TEST.md specification"
    echo "custom" > /tmp/test_spec_type.txt
fi

echo ""
echo "=== Hardware Summary ==="
echo "Flash method: $FLASH_METHOD"
if [ -n "$USB_CDC_PORT" ]; then
    echo "USB CDC: $USB_CDC_PORT"
fi
if [ -n "$ESP_PROBE" ]; then
    echo "JTAG Probe: $ESP_PROBE"
fi
if [ -n "$UART_PORTS" ]; then
    echo "UART ports: $UART_COUNT"
fi

SCRIPT

chmod +x /tmp/test_setup.sh
/tmp/test_setup.sh
```

---

## Step 1: Read TEST.md (if available)

**If TEST.md exists (test_spec_type.txt contains "custom"):**

Use the Read tool to read the TEST.md file:
```
Read: $(cat /tmp/test_lesson_dir.txt)/TEST.md
```

**If TEST.md doesn't exist (test_spec_type.txt contains "generic"):**

Skip to Step 2 and use the generic test specification below.

---

## Step 2: Execute Tests

### Cleanup Previous Debug Sessions

```bash
echo "=== Cleanup ==="
pkill -f "probe-rs" 2>/dev/null || true
pkill -f "openocd" 2>/dev/null || true
sleep 1
echo "✓ Cleanup complete"
```

### Build Firmware (with subsecond timing)

```bash
cat > /tmp/test_build.sh << 'SCRIPT'
#!/bin/bash
set -e

LESSON_DIR=$(cat /tmp/test_lesson_dir.txt)
cd "$LESSON_DIR"

echo "=== Build Firmware ==="
echo "Building in: $LESSON_DIR"

# Use millisecond precision for accurate build timing
START_TIME=$(python3 -c "import time; print(int(time.time() * 1000))")
cargo build --release 2>&1 | tail -20
BUILD_EXIT=$?
END_TIME=$(python3 -c "import time; print(int(time.time() * 1000))")
BUILD_TIME_MS=$((END_TIME - START_TIME))
BUILD_TIME_SEC=$(python3 -c "print(f'{$BUILD_TIME_MS / 1000:.2f}')")

if [ $BUILD_EXIT -eq 0 ]; then
    BINARY_NAME=$(cat /tmp/binary_name.txt)
    TARGET_DIR=$(cat /tmp/target_dir.txt)
    BINARY_PATH="$TARGET_DIR/$BINARY_NAME"

    if [ -f "$BINARY_PATH" ]; then
        BINARY_SIZE=$(ls -lh "$BINARY_PATH" | awk '{print $5}')
        BINARY_SIZE_BYTES=$(ls -l "$BINARY_PATH" | awk '{print $5}')
        echo ""
        echo "✓ Build successful"
        echo "✓ Binary: $BINARY_NAME"
        echo "✓ Size: $BINARY_SIZE ($BINARY_SIZE_BYTES bytes)"
        echo "✓ Build time: ${BUILD_TIME_SEC}s"

        # Save for report
        echo "$BINARY_SIZE_BYTES" > /tmp/binary_size_bytes.txt
        echo "$BUILD_TIME_SEC" > /tmp/build_time.txt
        echo "pass" > /tmp/test_build_result.txt
    else
        echo "✗ Binary not found at $BINARY_PATH"
        echo "fail" > /tmp/test_build_result.txt
        exit 1
    fi
else
    echo "✗ Build failed (exit code: $BUILD_EXIT)"
    echo "fail" > /tmp/test_build_result.txt
    exit $BUILD_EXIT
fi

SCRIPT

chmod +x /tmp/test_build.sh
/tmp/test_build.sh
```

### Flash Firmware (with automatic hardware re-detection)

```bash
cat > /tmp/test_flash.sh << 'SCRIPT'
#!/bin/bash
set -e

LESSON_DIR=$(cat /tmp/test_lesson_dir.txt)
BINARY_NAME=$(cat /tmp/binary_name.txt)
TARGET_DIR=$(cat /tmp/target_dir.txt)
FLASH_METHOD=$(cat /tmp/flash_method.txt)

cd "$LESSON_DIR"
BINARY_PATH="$TARGET_DIR/$BINARY_NAME"

if [ ! -f "$BINARY_PATH" ]; then
    echo "✗ ERROR: Binary not found at $BINARY_PATH"
    exit 1
fi

echo "=== Flash Firmware ==="
echo "Method: $FLASH_METHOD"
echo "Binary: $BINARY_NAME"

# Function to re-detect hardware if flash fails
redetect_hardware() {
    echo ""
    echo "⚠ Flash failed - attempting hardware re-detection..."
    echo ""

    # Re-detect USB CDC
    NEW_USB_CDC=$(ls /dev/cu.usbmodem* 2>/dev/null | head -1)
    OLD_USB_CDC=$(cat /tmp/usb_cdc_port.txt 2>/dev/null)

    if [ "$NEW_USB_CDC" != "$OLD_USB_CDC" ]; then
        if [ -n "$NEW_USB_CDC" ]; then
            echo "✓ USB CDC changed: $OLD_USB_CDC → $NEW_USB_CDC"
            echo "$NEW_USB_CDC" > /tmp/usb_cdc_port.txt
        else
            echo "✗ USB CDC port no longer available (was: $OLD_USB_CDC)"
            echo "" > /tmp/usb_cdc_port.txt
        fi
    fi

    # Re-detect probe
    NEW_PROBE=$(probe-rs list 2>&1 | grep -i "esp.*jtag" | grep -oE '[0-9a-fA-F]{4}:[0-9a-fA-F]{4}(:[0-9A-F:]+)?' | head -1)
    OLD_PROBE=$(cat /tmp/esp_probe.txt 2>/dev/null)

    if [ "$NEW_PROBE" != "$OLD_PROBE" ]; then
        if [ -n "$NEW_PROBE" ]; then
            echo "✓ JTAG probe changed: $OLD_PROBE → $NEW_PROBE"
            echo "$NEW_PROBE" > /tmp/esp_probe.txt
        else
            echo "✗ JTAG probe no longer available (was: $OLD_PROBE)"
            echo "" > /tmp/esp_probe.txt
        fi
    fi

    echo ""
    echo "Re-detection complete. Please reconnect hardware and retry."
    return 1
}

# Try flash with current hardware config
FLASH_EXIT=1

if [ "$FLASH_METHOD" = "espflash" ]; then
    USB_CDC_PORT=$(cat /tmp/usb_cdc_port.txt)
    echo "Port: $USB_CDC_PORT"

    espflash flash --port "$USB_CDC_PORT" "$BINARY_PATH" 2>&1 | tee /tmp/flash_output.txt | tail -15
    FLASH_EXIT=$?

    if [ $FLASH_EXIT -ne 0 ]; then
        # Check if error is due to missing port
        if grep -qE "(No such file|cannot open|not found)" /tmp/flash_output.txt; then
            redetect_hardware
        fi
    fi

elif [ "$FLASH_METHOD" = "probe-rs" ]; then
    ESP_PROBE=$(cat /tmp/esp_probe.txt)
    echo "Probe: $ESP_PROBE"

    # Use probe-rs download instead of run (just flash, don't start)
    probe-rs download --chip esp32c6 --probe "$ESP_PROBE" "$BINARY_PATH" 2>&1 | tee /tmp/flash_output.txt | tail -15
    FLASH_EXIT=$?

    if [ $FLASH_EXIT -ne 0 ]; then
        if grep -q "No connected probes" /tmp/flash_output.txt; then
            redetect_hardware
        fi
    fi
else
    echo "✗ ERROR: Unknown flash method: $FLASH_METHOD"
    exit 1
fi

if [ $FLASH_EXIT -eq 0 ]; then
    echo ""
    echo "✓ Flash complete"
    echo "pass" > /tmp/test_flash_result.txt
else
    echo "✗ Flash failed"
    echo "fail" > /tmp/test_flash_result.txt
    exit 1
fi

SCRIPT

chmod +x /tmp/test_flash.sh
/tmp/test_flash.sh
```

### Detect Active Terminal Port (Auto-discovery)

**CRITICAL:** Before boot verification, automatically find which serial port has the active terminal:

```bash
cat > /tmp/detect_terminal_port.sh << 'SCRIPT'
#!/bin/bash

echo "=== Detecting Active Terminal Port ==="
echo ""

# Collect all available serial ports
SERIAL_PORTS=""

# Add USB CDC if available
USB_CDC=$(cat /tmp/usb_cdc_port.txt 2>/dev/null)
if [ -n "$USB_CDC" ] && [ -e "$USB_CDC" ]; then
    SERIAL_PORTS="$USB_CDC"
fi

# Add UART adapters if available
UART_PORTS=$(cat /tmp/uart_ports.txt 2>/dev/null)
if [ -n "$UART_PORTS" ]; then
    SERIAL_PORTS="$SERIAL_PORTS $UART_PORTS"
fi

# Remove duplicates and extra spaces
SERIAL_PORTS=$(echo "$SERIAL_PORTS" | tr ' ' '\n' | sort -u | tr '\n' ' ' | xargs)

if [ -z "$SERIAL_PORTS" ]; then
    echo "✗ No serial ports available"
    echo "" > /tmp/terminal_port.txt
    echo "none" > /tmp/terminal_port_status.txt
    exit 0
fi

echo "Scanning ports for terminal activity:"
for PORT in $SERIAL_PORTS; do
    echo "  - $PORT"
done
echo ""

# Test each port to find which one has terminal output
python3 << 'PYCODE'
import serial
import time
import sys
import os

ports_str = """$SERIAL_PORTS""".strip()
ports = [p.strip() for p in ports_str.split() if p.strip()]

def test_port(port):
    """Test if a port has an active terminal."""
    try:
        print(f"Testing {port}...", flush=True)

        # Try to open the port
        ser = serial.Serial(port, 115200, timeout=0.3)

        # Clear any existing data
        ser.reset_input_buffer()

        # Send newline to wake up terminal
        ser.write(b'\r\n')
        time.sleep(0.2)

        # Try help command
        ser.write(b'help\r\n')
        time.sleep(0.4)

        # Check for response
        output = []
        for _ in range(20):
            if ser.in_waiting > 0:
                try:
                    line = ser.readline().decode('utf-8', errors='replace').strip()
                    if line:
                        output.append(line)
                except:
                    pass

        ser.close()

        if len(output) > 0:
            # Check if it looks like a terminal
            output_str = ' '.join(output).lower()

            # Look for terminal indicators
            is_terminal = (
                '>' in output_str or
                'help' in output_str or
                'command' in output_str or
                'available' in output_str or
                'status' in output_str
            )

            if is_terminal:
                print(f"  ✓ Active terminal detected")
                print(f"  Response: {output[0][:60]}...")
                return True, output
            else:
                print(f"  ✗ Data received but not a terminal")
                print(f"  Response: {output[0][:60]}...")
        else:
            print(f"  ✗ No response")

        return False, []

    except serial.SerialException as e:
        print(f"  ✗ Cannot open: {e}")
        return False, []
    except Exception as e:
        print(f"  ✗ Error: {e}")
        return False, []

# Test each port
found_terminal = None
terminal_output = []

for port in ports:
    is_terminal, output = test_port(port)
    if is_terminal:
        found_terminal = port
        terminal_output = output
        break

print()

if found_terminal:
    print(f"✓ Found active terminal on {found_terminal}")

    # Save results
    with open('/tmp/terminal_port.txt', 'w') as f:
        f.write(found_terminal)
    with open('/tmp/terminal_port_status.txt', 'w') as f:
        f.write('found')
    with open('/tmp/terminal_sample_output.txt', 'w') as f:
        f.write('\n'.join(terminal_output[:5]))

    sys.exit(0)
else:
    print("✗ No active terminal found on any port")
    print()
    print("Possible reasons:")
    print("  - Firmware not running (check flash step)")
    print("  - Terminal on RTT (use probe-rs run --rtt)")
    print("  - Hardware not connected (check wiring)")
    print("  - Wrong baud rate (check firmware config)")

    with open('/tmp/terminal_port.txt', 'w') as f:
        f.write('')
    with open('/tmp/terminal_port_status.txt', 'w') as f:
        f.write('not_found')

    sys.exit(0)

PYCODE

SCRIPT

chmod +x /tmp/detect_terminal_port.sh
/tmp/detect_terminal_port.sh
```

### Execute Infrastructure Tests

**CRITICAL:** ALL infrastructure tests MUST use temp scripts to avoid parse errors in eval context.

**Test: Debug Symbols Verification**
```bash
cat > /tmp/test_debug_symbols.sh << 'SCRIPT'
#!/bin/bash
LESSON_DIR=$(cat /tmp/test_lesson_dir.txt)
BINARY_NAME=$(cat /tmp/binary_name.txt)
TARGET_DIR=$(cat /tmp/target_dir.txt)

cd "$LESSON_DIR"
echo "=== Test: Debug Symbols ==="

file "$TARGET_DIR/$BINARY_NAME" | grep "not stripped"
RESULT=$?

if [ $RESULT -eq 0 ]; then
    echo "✓ Debug symbols present"
    echo "pass" > /tmp/test_debug_result.txt
else
    echo "✗ Debug symbols missing"
    echo "fail" > /tmp/test_debug_result.txt
    exit 1
fi
SCRIPT

chmod +x /tmp/test_debug_symbols.sh
/tmp/test_debug_symbols.sh
```

**Test: Source Code Structure**
```bash
cat > /tmp/test_source_structure.sh << 'SCRIPT'
#!/bin/bash
LESSON_DIR=$(cat /tmp/test_lesson_dir.txt)
cd "$LESSON_DIR"

echo "=== Test: Source Code Structure ==="
echo "Checking required files..."

PASS=0
TOTAL=0

if [ -f src/bin/main.rs ]; then
    LINES=$(wc -l < src/bin/main.rs | tr -d ' ')
    echo "✓ src/bin/main.rs exists ($LINES lines)"
    PASS=$((PASS + 1))
else
    echo "✗ src/bin/main.rs missing"
fi
TOTAL=$((TOTAL + 1))

if [ -f src/lib.rs ]; then
    LINES=$(wc -l < src/lib.rs | tr -d ' ')
    echo "✓ src/lib.rs exists ($LINES lines)"
    PASS=$((PASS + 1))
else
    echo "⚠ src/lib.rs not present (optional)"
fi
TOTAL=$((TOTAL + 1))

if [ -f Cargo.toml ]; then
    echo "✓ Cargo.toml exists"
    PASS=$((PASS + 1))
else
    echo "✗ Cargo.toml missing"
fi
TOTAL=$((TOTAL + 1))

if [ -f .cargo/config.toml ]; then
    echo "✓ .cargo/config.toml exists"
    PASS=$((PASS + 1))
else
    echo "⚠ .cargo/config.toml missing"
fi
TOTAL=$((TOTAL + 1))

echo "$PASS/$TOTAL" > /tmp/test_source_result.txt

if [ $PASS -ge 3 ]; then
    echo "✓ Source structure acceptable ($PASS/$TOTAL files)"
else
    echo "✗ Source structure incomplete ($PASS/$TOTAL files)"
    exit 1
fi
SCRIPT

chmod +x /tmp/test_source_structure.sh
/tmp/test_source_structure.sh
```

**Test: Cargo.toml Configuration**
```bash
cat > /tmp/test_cargo_config.sh << 'SCRIPT'
#!/bin/bash
LESSON_DIR=$(cat /tmp/test_lesson_dir.txt)
cd "$LESSON_DIR"

echo "=== Test: Cargo.toml Configuration ==="
echo "Binary configuration:"
grep -A2 '\[\[bin\]\]' Cargo.toml

echo ""
echo "Debug configuration:"
HAS_DEBUG=$(grep -A2 '\[profile.release\]' Cargo.toml | grep "debug")

if [ -n "$HAS_DEBUG" ]; then
    echo "$HAS_DEBUG"
    echo "✓ Debug settings found"
    echo "pass" > /tmp/test_cargo_result.txt
else
    echo "⚠ No debug setting found in release profile"
    echo "warn" > /tmp/test_cargo_result.txt
fi
SCRIPT

chmod +x /tmp/test_cargo_config.sh
/tmp/test_cargo_config.sh
```

**Test: Boot Verification (if terminal detected)**
```bash
cat > /tmp/test_boot.sh << 'SCRIPT'
#!/bin/bash

TERMINAL_STATUS=$(cat /tmp/terminal_port_status.txt 2>/dev/null)
TERMINAL_PORT=$(cat /tmp/terminal_port.txt 2>/dev/null)

if [ "$TERMINAL_STATUS" != "found" ] || [ -z "$TERMINAL_PORT" ]; then
    echo "=== Test: Boot Verification ==="
    echo "⚠ SKIPPED - No terminal port detected"
    echo "  Firmware may be using RTT for output"
    echo "  Try: probe-rs run --chip esp32c6 target/.../binary"
    echo "skipped" > /tmp/test_boot_result.txt
    exit 0
fi

echo "=== Test: Boot Verification ==="
echo "Port: $TERMINAL_PORT"
echo "Status: Terminal detected and responsive"

# We already verified terminal is working during port detection
# Just read sample output again to confirm it's still alive

python3 << PYCODE
import serial
import time

try:
    port = "$TERMINAL_PORT"
    ser = serial.Serial(port, 115200, timeout=0.3)

    # Send status command
    ser.write(b'status\r\n')
    time.sleep(0.3)

    output = []
    for _ in range(10):
        if ser.in_waiting > 0:
            line = ser.readline().decode('utf-8', errors='replace').strip()
            if line:
                output.append(line)

    ser.close()

    if len(output) > 0:
        print("\nTerminal output sample:")
        for line in output[:5]:
            print(f"  {line}")

        # Save result
        with open('/tmp/test_boot_result.txt', 'w') as f:
            f.write('pass')

        print("\n✓ Boot verification passed")
    else:
        print("⚠ Terminal responsive but no output to status command")
        with open('/tmp/test_boot_result.txt', 'w') as f:
            f.write('partial')

except Exception as e:
    print(f"✗ Error: {e}")
    with open('/tmp/test_boot_result.txt', 'w') as f:
        f.write('fail')

PYCODE

SCRIPT

chmod +x /tmp/test_boot.sh
/tmp/test_boot.sh
```

### Execute Lesson-Specific Tests

**If TEST.md exists:** Parse and execute the automated tests specified in TEST.md.

#### TEST.md Standard Format

Most TEST.md files follow this structure:
```markdown
## Automated Tests

### Test N: [Name]
**Command:** `command to run`
**Expected:** Expected behavior
**Success Criteria:** How to verify success
```

#### Parsing Strategy

1. **Read the TEST.md file** (already done in Step 1)

2. **Identify automated tests:**
   - Look for "## Automated Tests" section
   - Each test has: name, command, expected output, success criteria

3. **For each automated test, create a temp script:**

**Example: If TEST.md specifies a GDB configuration test:**

```bash
cat > /tmp/test_lesson_specific.sh << 'SCRIPT'
#!/bin/bash
LESSON_DIR=$(cat /tmp/test_lesson_dir.txt)
cd "$LESSON_DIR"

echo "=== Test: GDB Configuration Files ==="
echo "Checking configuration files..."

PASS=0
TOTAL=0

if [ -f .gdbinit ]; then
    LINES=$(wc -l < .gdbinit | tr -d ' ')
    echo "✓ .gdbinit exists ($LINES lines)"
    PASS=$((PASS + 1))
else
    echo "✗ .gdbinit missing"
fi
TOTAL=$((TOTAL + 1))

if [ -f gdb_helpers.py ]; then
    LINES=$(wc -l < gdb_helpers.py | tr -d ' ')
    echo "✓ gdb_helpers.py exists ($LINES lines)"
    PASS=$((PASS + 1))
else
    echo "✗ gdb_helpers.py missing"
fi
TOTAL=$((TOTAL + 1))

echo "$PASS/$TOTAL" > /tmp/test_lesson_result.txt

if [ $PASS -eq $TOTAL ]; then
    echo "✓ All lesson-specific files present"
else
    echo "⚠ Missing $((TOTAL - PASS)) file(s)"
fi
SCRIPT

chmod +x /tmp/test_lesson_specific.sh
/tmp/test_lesson_specific.sh
```

4. **Save test results to files:**
   - Use `/tmp/testN_result.txt` for each test (pass/fail/partial/warn/skipped)
   - Reference these files in report generation

5. **Skip interactive/manual tests in quick mode:**
   - Tests marked "Interactive" require user input
   - Note these in the report as "Manual tests - see TEST.md"

**If TEST.md doesn't exist:** The generic infrastructure tests above are sufficient.

---

## Step 3: Generate Test Report

**Simplified report generation (no nested heredoc issues):**

```bash
echo "=== Generating Test Report ==="

LESSON_NUM=$(cat /tmp/test_lesson_num.txt)
LESSON_NAME=$(cat /tmp/test_lesson_name.txt)
MODE=$(cat /tmp/test_mode.txt)
CURRENT_DATE=$(date '+%Y-%m-%d %H:%M')
USB_CDC_PORT=$(cat /tmp/usb_cdc_port.txt)
ESP_PROBE=$(cat /tmp/esp_probe.txt)
FLASH_METHOD=$(cat /tmp/flash_method.txt)
TERMINAL_PORT=$(cat /tmp/terminal_port.txt)
TERMINAL_STATUS=$(cat /tmp/terminal_port_status.txt)

# Count test results
TEST_RESULTS=""
TEST_PASS=0
TEST_FAIL=0
TEST_WARN=0
TEST_SKIP=0

# Count infrastructure tests
for TEST_FILE in /tmp/test_*_result.txt; do
    if [ -f "$TEST_FILE" ]; then
        RESULT=$(cat "$TEST_FILE")
        case "$RESULT" in
            pass) TEST_PASS=$((TEST_PASS + 1)) ;;
            fail) TEST_FAIL=$((TEST_FAIL + 1)) ;;
            warn) TEST_WARN=$((TEST_WARN + 1)) ;;
            partial) TEST_WARN=$((TEST_WARN + 1)) ;;
            skipped) TEST_SKIP=$((TEST_SKIP + 1)) ;;
        esac
    fi
done

TEST_TOTAL=$((TEST_PASS + TEST_FAIL + TEST_WARN + TEST_SKIP))
SUCCESS_RATE=$(python3 -c "print(f'{$TEST_PASS / $TEST_TOTAL * 100:.1f}' if $TEST_TOTAL > 0 else '0.0')")

cat > /tmp/test_report.md << 'REPORT_EOF'
# Lesson LESSON_NUM_PLACEHOLDER Test Report

**Date:** DATE_PLACEHOLDER
**Lesson:** LESSON_NUM_PLACEHOLDER - LESSON_NAME_PLACEHOLDER
**Mode:** MODE_PLACEHOLDER
**Duration:** ~DURATION_PLACEHOLDER minutes

## Summary
- Total Automated Tests: TOTAL_PLACEHOLDER
- Passed: PASS_PLACEHOLDER
- Failed: FAIL_PLACEHOLDER
- Warnings: WARN_PLACEHOLDER
- Skipped: SKIP_PLACEHOLDER
- Success Rate: RATE_PLACEHOLDER%

## Environment
- ESP32-C6: Connected ✓
- Flash Method: FLASH_METHOD_PLACEHOLDER
- USB CDC Port: USB_CDC_PLACEHOLDER USB_CDC_STATUS_PLACEHOLDER
- JTAG Probe: PROBE_PLACEHOLDER PROBE_STATUS_PLACEHOLDER
- Terminal Port: TERMINAL_PLACEHOLDER TERMINAL_STATUS_PLACEHOLDER
- Firmware: Built ✓

## Automated Test Results

### Test 1: Build Verification
- **Command:** `cargo build --release`
- **Expected:** Successful build with debug symbols
- **Actual:** BUILD_ACTUAL_PLACEHOLDER
- **Status:** BUILD_STATUS_PLACEHOLDER

### Test 2: Flash Firmware
- **Command:** `FLASH_CMD_PLACEHOLDER`
- **Expected:** Firmware flashes successfully
- **Actual:** FLASH_ACTUAL_PLACEHOLDER
- **Status:** FLASH_STATUS_PLACEHOLDER

### Test 3: Debug Symbols
- **Command:** `file ... | grep "not stripped"`
- **Expected:** Binary contains debug symbols
- **Actual:** DEBUG_ACTUAL_PLACEHOLDER
- **Status:** DEBUG_STATUS_PLACEHOLDER

### Test 4: Source Code Structure
- **Command:** Check for required files
- **Expected:** All required files present
- **Actual:** SOURCE_ACTUAL_PLACEHOLDER
- **Status:** SOURCE_STATUS_PLACEHOLDER

### Test 5: Cargo.toml Configuration
- **Command:** Verify binary and debug configuration
- **Expected:** Proper binary definition and debug settings
- **Actual:** CARGO_ACTUAL_PLACEHOLDER
- **Status:** CARGO_STATUS_PLACEHOLDER

### Test 6: Boot Verification
- **Command:** Detect and test serial terminal
- **Expected:** Terminal responds to commands
- **Actual:** BOOT_ACTUAL_PLACEHOLDER
- **Status:** BOOT_STATUS_PLACEHOLDER

LESSON_SPECIFIC_TESTS_PLACEHOLDER

## Manual Test Instructions

TEST_SPEC_PLACEHOLDER

## Issues Found

ISSUES_PLACEHOLDER

## Recommendations

RECOMMENDATIONS_PLACEHOLDER

## Conclusion

**Test Status:** CONCLUSION_STATUS_PLACEHOLDER

CONCLUSION_SUMMARY_PLACEHOLDER

**Hardware Status:**
- HARDWARE_STATUS_PLACEHOLDER

**Next Steps:**
NEXT_STEPS_PLACEHOLDER

REPORT_EOF

# Now do all the substitutions (this avoids nested heredoc issues)
BUILD_STATUS=$(cat /tmp/test_build_result.txt 2>/dev/null || echo "unknown")
BUILD_TIME=$(cat /tmp/build_time.txt 2>/dev/null || echo "?")
BINARY_SIZE=$(cat /tmp/binary_size_bytes.txt 2>/dev/null || echo "?")

sed -i '' "s/LESSON_NUM_PLACEHOLDER/$LESSON_NUM/g" /tmp/test_report.md
sed -i '' "s/DATE_PLACEHOLDER/$CURRENT_DATE/g" /tmp/test_report.md
sed -i '' "s/LESSON_NAME_PLACEHOLDER/$LESSON_NAME/g" /tmp/test_report.md
sed -i '' "s/MODE_PLACEHOLDER/$MODE/g" /tmp/test_report.md
sed -i '' "s/TOTAL_PLACEHOLDER/$TEST_TOTAL/g" /tmp/test_report.md
sed -i '' "s/PASS_PLACEHOLDER/$TEST_PASS/g" /tmp/test_report.md
sed -i '' "s/FAIL_PLACEHOLDER/$TEST_FAIL/g" /tmp/test_report.md
sed -i '' "s/WARN_PLACEHOLDER/$TEST_WARN/g" /tmp/test_report.md
sed -i '' "s/SKIP_PLACEHOLDER/$TEST_SKIP/g" /tmp/test_report.md
sed -i '' "s/RATE_PLACEHOLDER/$SUCCESS_RATE/g" /tmp/test_report.md
sed -i '' "s/FLASH_METHOD_PLACEHOLDER/$FLASH_METHOD/g" /tmp/test_report.md

# Build human-readable status indicators
if [ -n "$USB_CDC_PORT" ]; then
    sed -i '' "s|USB_CDC_PLACEHOLDER|$USB_CDC_PORT|g" /tmp/test_report.md
    sed -i '' "s/USB_CDC_STATUS_PLACEHOLDER/✓/g" /tmp/test_report.md
else
    sed -i '' "s/USB_CDC_PLACEHOLDER/Not detected/g" /tmp/test_report.md
    sed -i '' "s/USB_CDC_STATUS_PLACEHOLDER/✗/g" /tmp/test_report.md
fi

if [ -n "$ESP_PROBE" ]; then
    sed -i '' "s/PROBE_PLACEHOLDER/$ESP_PROBE/g" /tmp/test_report.md
    sed -i '' "s/PROBE_STATUS_PLACEHOLDER/✓/g" /tmp/test_report.md
else
    sed -i '' "s/PROBE_PLACEHOLDER/Not detected/g" /tmp/test_report.md
    sed -i '' "s/PROBE_STATUS_PLACEHOLDER/✗/g" /tmp/test_report.md
fi

if [ "$TERMINAL_STATUS" = "found" ] && [ -n "$TERMINAL_PORT" ]; then
    sed -i '' "s/TERMINAL_PLACEHOLDER/$TERMINAL_PORT/g" /tmp/test_report.md
    sed -i '' "s/TERMINAL_STATUS_PLACEHOLDER/✓/g" /tmp/test_report.md
else
    sed -i '' "s/TERMINAL_PLACEHOLDER/Not detected/g" /tmp/test_report.md
    sed -i '' "s/TERMINAL_STATUS_PLACEHOLDER/⚠/g" /tmp/test_report.md
fi

# Fill in test results
BUILD_ACTUAL="Build time: ${BUILD_TIME}s, Size: ${BINARY_SIZE} bytes"
sed -i '' "s/BUILD_ACTUAL_PLACEHOLDER/$BUILD_ACTUAL/g" /tmp/test_report.md

if [ "$BUILD_STATUS" = "pass" ]; then
    sed -i '' "s/BUILD_STATUS_PLACEHOLDER/✓ PASS/g" /tmp/test_report.md
else
    sed -i '' "s/BUILD_STATUS_PLACEHOLDER/✗ FAIL/g" /tmp/test_report.md
fi

# Add flash command based on method
if [ "$FLASH_METHOD" = "espflash" ]; then
    sed -i '' "s/FLASH_CMD_PLACEHOLDER/espflash flash --port $USB_CDC_PORT .../g" /tmp/test_report.md
else
    sed -i '' "s/FLASH_CMD_PLACEHOLDER/probe-rs download --chip esp32c6 --probe .../g" /tmp/test_report.md
fi

FLASH_STATUS=$(cat /tmp/test_flash_result.txt 2>/dev/null || echo "unknown")
if [ "$FLASH_STATUS" = "pass" ]; then
    sed -i '' "s/FLASH_STATUS_PLACEHOLDER/✓ PASS/g" /tmp/test_report.md
    sed -i '' "s/FLASH_ACTUAL_PLACEHOLDER/Flashed successfully via $FLASH_METHOD/g" /tmp/test_report.md
else
    sed -i '' "s/FLASH_STATUS_PLACEHOLDER/✗ FAIL/g" /tmp/test_report.md
    sed -i '' "s/FLASH_ACTUAL_PLACEHOLDER/Flash failed - see output above/g" /tmp/test_report.md
fi

DEBUG_STATUS=$(cat /tmp/test_debug_result.txt 2>/dev/null || echo "unknown")
if [ "$DEBUG_STATUS" = "pass" ]; then
    sed -i '' "s/DEBUG_STATUS_PLACEHOLDER/✓ PASS/g" /tmp/test_report.md
    sed -i '' "s/DEBUG_ACTUAL_PLACEHOLDER/Binary contains debug symbols (not stripped)/g" /tmp/test_report.md
else
    sed -i '' "s/DEBUG_STATUS_PLACEHOLDER/✗ FAIL/g" /tmp/test_report.md
    sed -i '' "s/DEBUG_ACTUAL_PLACEHOLDER/Debug symbols not found/g" /tmp/test_report.md
fi

SOURCE_RESULT=$(cat /tmp/test_source_result.txt 2>/dev/null || echo "?/?")
sed -i '' "s/SOURCE_ACTUAL_PLACEHOLDER/Files present: $SOURCE_RESULT/g" /tmp/test_report.md
if echo "$SOURCE_RESULT" | grep -q "3/\|4/4"; then
    sed -i '' "s/SOURCE_STATUS_PLACEHOLDER/✓ PASS/g" /tmp/test_report.md
else
    sed -i '' "s/SOURCE_STATUS_PLACEHOLDER/⚠ PARTIAL/g" /tmp/test_report.md
fi

CARGO_STATUS=$(cat /tmp/test_cargo_result.txt 2>/dev/null || echo "unknown")
if [ "$CARGO_STATUS" = "pass" ]; then
    sed -i '' "s/CARGO_STATUS_PLACEHOLDER/✓ PASS/g" /tmp/test_report.md
    sed -i '' "s/CARGO_ACTUAL_PLACEHOLDER/Binary and debug configuration found/g" /tmp/test_report.md
else
    sed -i '' "s/CARGO_STATUS_PLACEHOLDER/⚠ WARN/g" /tmp/test_report.md
    sed -i '' "s/CARGO_ACTUAL_PLACEHOLDER/Debug configuration missing or incomplete/g" /tmp/test_report.md
fi

BOOT_STATUS=$(cat /tmp/test_boot_result.txt 2>/dev/null || echo "unknown")
if [ "$BOOT_STATUS" = "pass" ]; then
    sed -i '' "s/BOOT_STATUS_PLACEHOLDER/✓ PASS/g" /tmp/test_report.md
    sed -i '' "s/BOOT_ACTUAL_PLACEHOLDER/Terminal detected on $TERMINAL_PORT and responsive/g" /tmp/test_report.md
elif [ "$BOOT_STATUS" = "skipped" ]; then
    sed -i '' "s/BOOT_STATUS_PLACEHOLDER/⊘ SKIPPED/g" /tmp/test_report.md
    sed -i '' "s/BOOT_ACTUAL_PLACEHOLDER/No terminal port detected - firmware may use RTT/g" /tmp/test_report.md
else
    sed -i '' "s/BOOT_STATUS_PLACEHOLDER/✗ FAIL/g" /tmp/test_report.md
    sed -i '' "s/BOOT_ACTUAL_PLACEHOLDER/Terminal not responding/g" /tmp/test_report.md
fi

# Add lesson-specific tests section if any
if [ -f /tmp/test_lesson_result.txt ]; then
    sed -i '' "s/LESSON_SPECIFIC_TESTS_PLACEHOLDER/### Additional Lesson Tests\\nSee above for lesson-specific test results./g" /tmp/test_report.md
else
    sed -i '' "s/LESSON_SPECIFIC_TESTS_PLACEHOLDER//g" /tmp/test_report.md
fi

# Add test spec note
TEST_SPEC_TYPE=$(cat /tmp/test_spec_type.txt)
if [ "$TEST_SPEC_TYPE" = "custom" ]; then
    sed -i '' "s/TEST_SPEC_PLACEHOLDER/⚠️ **See TEST.md for manual test procedures**/g" /tmp/test_report.md
else
    sed -i '' "s/TEST_SPEC_PLACEHOLDER/⚠️ **No TEST.md found - manual testing not specified**/g" /tmp/test_report.md
fi

# Generic conclusion based on success rate
if [ "$TEST_FAIL" -eq 0 ] && [ "$TEST_TOTAL" -gt 0 ]; then
    sed -i '' "s/CONCLUSION_STATUS_PLACEHOLDER/✓ PASS/g" /tmp/test_report.md
    sed -i '' "s/CONCLUSION_SUMMARY_PLACEHOLDER/All automated tests passed successfully. Firmware is ready for use./g" /tmp/test_report.md
    sed -i '' "s/HARDWARE_STATUS_PLACEHOLDER/✓ ESP32-C6 detected and functional\\n- ✓ Firmware builds and flashes successfully/g" /tmp/test_report.md
    sed -i '' "s/NEXT_STEPS_PLACEHOLDER/1. Test manually if TEST.md specifies interactive tests\\n2. Deploy to hardware/g" /tmp/test_report.md
    sed -i '' "s/ISSUES_PLACEHOLDER/None - all tests passed./g" /tmp/test_report.md
    sed -i '' "s/RECOMMENDATIONS_PLACEHOLDER/No issues found./g" /tmp/test_report.md
elif [ "$TEST_FAIL" -gt 0 ]; then
    sed -i '' "s/CONCLUSION_STATUS_PLACEHOLDER/✗ FAIL/g" /tmp/test_report.md
    sed -i '' "s/CONCLUSION_SUMMARY_PLACEHOLDER/$TEST_FAIL test(s) failed. See test results above for details./g" /tmp/test_report.md
    sed -i '' "s/HARDWARE_STATUS_PLACEHOLDER/⚠ Some tests failed - review test output above/g" /tmp/test_report.md
    sed -i '' "s/NEXT_STEPS_PLACEHOLDER/1. Fix failing tests\\n2. Re-run: \/test-lesson $LESSON_NUM/g" /tmp/test_report.md
    sed -i '' "s/ISSUES_PLACEHOLDER/See failed test results above./g" /tmp/test_report.md
    sed -i '' "s/RECOMMENDATIONS_PLACEHOLDER/Fix failing tests before deployment./g" /tmp/test_report.md
else
    sed -i '' "s/CONCLUSION_STATUS_PLACEHOLDER/⚠ PARTIAL/g" /tmp/test_report.md
    sed -i '' "s/CONCLUSION_SUMMARY_PLACEHOLDER/$TEST_WARN warnings, $TEST_SKIP skipped. Review test results./g" /tmp/test_report.md
    sed -i '' "s/HARDWARE_STATUS_PLACEHOLDER/⚠ Tests completed with warnings/g" /tmp/test_report.md
    sed -i '' "s/NEXT_STEPS_PLACEHOLDER/1. Review warnings\\n2. Consider addressing before deployment/g" /tmp/test_report.md
    sed -i '' "s/ISSUES_PLACEHOLDER/See warnings above./g" /tmp/test_report.md
    sed -i '' "s/RECOMMENDATIONS_PLACEHOLDER/Address warnings if possible./g" /tmp/test_report.md
fi

# Add duration estimate
DURATION="?"
if [ "$TEST_TOTAL" -le 5 ]; then
    DURATION="2"
elif [ "$TEST_TOTAL" -le 8 ]; then
    DURATION="3-5"
else
    DURATION="5-10"
fi
sed -i '' "s/DURATION_PLACEHOLDER/$DURATION/g" /tmp/test_report.md

echo "✓ Report generated"
echo ""
cat /tmp/test_report.md
```

---

## Step 4: Cleanup

After testing completes, clean up temp files:

```bash
echo ""
echo "=== Cleanup Temp Files ==="
rm -f /tmp/test_*.txt /tmp/test_*.sh /tmp/test_*.md 2>/dev/null || true
rm -f /tmp/usb_*.txt /tmp/esp_*.txt /tmp/uart_*.txt 2>/dev/null || true
rm -f /tmp/binary_*.txt /tmp/target_*.txt /tmp/probe_*.txt 2>/dev/null || true
rm -f /tmp/boot_*.txt /tmp/flash_*.txt /tmp/terminal_*.txt 2>/dev/null || true
echo "✓ Cleanup complete"
```

---

## Best Practices for Test Execution

### 1. Shell Syntax Guidelines

**⚠️ CRITICAL: ALWAYS use temp scripts for ANY test with:**
- Command substitution: `$(command)`
- Conditionals: `if/then/fi`
- Loops: `for/while`
- Pipes with complex commands: `cmd | grep ...`

**❌ NEVER do this (causes parse errors in eval context):**
```bash
# This WILL fail with "parse error near '('"
LESSON_DIR=$(cat /tmp/test_lesson_dir.txt)
file "$TARGET_DIR/$BINARY_NAME" | grep "not stripped"
```

**✅ ALWAYS do this instead:**
```bash
cat > /tmp/test_step.sh << 'SCRIPT'
#!/bin/bash
LESSON_DIR=$(cat /tmp/test_lesson_dir.txt)
BINARY_NAME=$(cat /tmp/binary_name.txt)
TARGET_DIR=$(cat /tmp/target_dir.txt)

cd "$LESSON_DIR"
file "$TARGET_DIR/$BINARY_NAME" | grep "not stripped"

if [ $? -eq 0 ]; then
    echo "✓ Test passed"
    echo "pass" > /tmp/test_result.txt
else
    echo "✗ Test failed"
    echo "fail" > /tmp/test_result.txt
fi
SCRIPT

chmod +x /tmp/test_step.sh
/tmp/test_step.sh
```

### 2. Variable Management

**File-based state is reliable (KEEP USING THIS):**
```bash
# Save values to files
echo "$VALUE" > /tmp/my_value.txt

# Read back later (even in different bash invocation)
VALUE=$(cat /tmp/my_value.txt)
```

**Don't rely on export/source across tool calls** - variables don't persist between separate bash tool invocations.

### 3. Binary Path Construction

Always detect binary name from Cargo.toml:
```bash
BINARY_NAME=$(grep -A1 '\[\[bin\]\]' Cargo.toml | grep 'name' | cut -d'"' -f2 | head -1)
```

### 4. Serial Communication

For reading serial output, use Python (embedded in bash script):
```bash
cat > /tmp/test_serial.sh << 'SCRIPT'
#!/bin/bash
python3 << 'PYCODE'
import serial
import time

port = "/dev/cu.usbmodem1101"
ser = serial.Serial(port, 115200, timeout=1)

# Send command
ser.write(b'help\r\n')
time.sleep(0.3)

# Read response
output = []
while ser.in_waiting > 0:
    line = ser.readline().decode('utf-8', errors='replace').strip()
    if line:
        output.append(line)

ser.close()

print('\n'.join(output))
PYCODE
SCRIPT

chmod +x /tmp/test_serial.sh
/tmp/test_serial.sh
```

### 5. Hardware Re-detection Pattern

If a test fails with "device not found" error, automatically re-detect hardware:
```bash
# In your test script, after a command fails:
if grep -q "No such file" /tmp/error_output.txt; then
    # Re-detect hardware
    NEW_PORT=$(ls /dev/cu.usbmodem* 2>/dev/null | head -1)
    if [ -n "$NEW_PORT" ]; then
        echo "✓ Hardware reconnected: $NEW_PORT"
        echo "$NEW_PORT" > /tmp/usb_cdc_port.txt
    fi
fi
```

### 6. Success Criteria

**Quick mode passes if:**
- Firmware builds successfully
- Firmware flashes successfully
- At least 70% of automated tests pass (≥5/7 typical tests)
- No critical configuration issues

**Full mode passes if:**
- All automated tests pass (100%)
- Manual test instructions are clear
- No unresolved issues

### 7. Test Result Tracking

Always save test results to files for accurate reporting:
```bash
# In each test script:
if [ test_passed ]; then
    echo "pass" > /tmp/testN_result.txt
elif [ test_has_warnings ]; then
    echo "warn" > /tmp/testN_result.txt
elif [ test_skipped ]; then
    echo "skipped" > /tmp/testN_result.txt
else
    echo "fail" > /tmp/testN_result.txt
fi
```

---

**After testing, present the final test report to the user.**
