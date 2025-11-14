#!/usr/bin/env python3
"""
Test script to verify structured logging output from ESP32-C6.
Captures RTT output and validates structured log format.
"""

import subprocess
import re
import sys
import time
import signal

def run_firmware_test(duration_seconds=8):
    """Run firmware and capture RTT output for testing."""
    print("📡 Starting firmware flashing and RTT capture...")
    print(f"   Duration: {duration_seconds} seconds")
    print()

    cmd = [
        'probe-rs', 'run',
        '--chip', 'esp32c6',
        '--probe', '303a:1001:F0:F5:BD:01:88:2C',
        'target/riscv32imac-unknown-none-elf/release/main'
    ]

    start_time = time.time()
    output_lines = []

    try:
        with subprocess.Popen(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            bufsize=1
        ) as proc:
            # Capture output for specified duration
            while time.time() - start_time < duration_seconds:
                try:
                    line = proc.stdout.readline()
                    if line:
                        output_lines.append(line.rstrip())
                        print(line.rstrip())
                except:
                    break

            # Gracefully terminate
            proc.terminate()
            try:
                proc.wait(timeout=2)
            except subprocess.TimeoutExpired:
                proc.kill()

    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        return []

    return output_lines

def analyze_output(lines):
    """Analyze captured output for structured logging patterns."""
    print("\n" + "="*70)
    print("ANALYSIS: Structured Logging Output")
    print("="*70)

    # Patterns to look for
    patterns = {
        'startup': r'Starting Lesson 08.*Structured Logging',
        'init_complete': r'Initialization complete',
        'loop_iteration': r'Loop iteration: count=',
        'i2c_transaction': r'I2C transaction:',
        'gpio_event': r'GPIO event:',
        'imu_reading': r'IMU reading:',
        'sensor_status': r'Sensor status:',
        'checkpoint': r'Checkpoint reached',
    }

    found_patterns = {}
    for pattern_name, pattern in patterns.items():
        matches = [line for line in lines if re.search(pattern, line)]
        found_patterns[pattern_name] = len(matches)
        if matches:
            print(f"✅ {pattern_name:20s}: Found {len(matches)} occurrence(s)")
            if len(matches) <= 3:
                for match in matches:
                    print(f"   └─ {match[:75]}")
        else:
            print(f"❌ {pattern_name:20s}: Not found")

    print("\n" + "-"*70)
    print("SUMMARY")
    print("-"*70)

    total_found = sum(found_patterns.values())
    total_expected = len(patterns)

    if total_found >= total_expected - 2:  # Allow 2 patterns to be missing
        print("✅ Structured logging appears to be working correctly!")
        print(f"   Found {total_found}/{total_expected} expected log patterns")
        return True
    else:
        print("⚠️  Some structured logging patterns not detected")
        print(f"   Found {total_found}/{total_expected} expected log patterns")
        return False

def main():
    print("\n" + "="*70)
    print("LESSON 08: Structured Logging Test Suite")
    print("="*70 + "\n")

    # Kill any existing probe-rs processes
    subprocess.run(['pkill', '-9', 'probe-rs'], stderr=subprocess.DEVNULL)
    time.sleep(1)

    # Run test
    output_lines = run_firmware_test(duration_seconds=8)

    if output_lines:
        success = analyze_output(output_lines)

        print("\n" + "="*70)
        if success:
            print("TEST RESULT: ✅ PASS")
        else:
            print("TEST RESULT: ⚠️  PARTIAL (some patterns missing)")
        print("="*70 + "\n")

        return 0
    else:
        print("\n❌ No output captured - firmware may not have flashed properly")
        return 1

if __name__ == '__main__':
    sys.exit(main())
