"""
ESP32-C6 GDB Helper Scripts for Rust Embedded Debugging

Simple, practical helpers for inspecting hardware state during debugging.
Load this in GDB with: source gdb_helpers.py

Example commands:
  (gdb) show-i2c        # Show I2C0 status with decoded flags
  (gdb) show-gpio       # Show GPIO input/output state
  (gdb) show-all        # Show all peripherals at once

Register addresses from ESP32-C6 Technical Reference Manual:
https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf

Alternative: Use esp32c6 PAC crate for typed register access in Rust code
"""

import gdb

# ESP32-C6 Peripheral Base Addresses (from TRM)
I2C0_BASE = 0x60013000      # Chapter 10: I2C Controller
GPIO_BASE = 0x60004000      # Chapter 7: GPIO & IO MUX
UART0_BASE = 0x60000000     # Chapter 25: UART Controller
UART1_BASE = 0x60010000
RMT_BASE = 0x60006000       # Chapter 16: Remote Control Peripheral

class ShowI2C(gdb.Command):
    """Show ESP32-C6 I2C0 peripheral status with decoded flags

    Usage: show-i2c

    Reads I2C0 registers and displays status in human-readable format.
    """

    def __init__(self):
        super(ShowI2C, self).__init__("show-i2c", gdb.COMMAND_USER)

    def invoke(self, arg, from_tty):
        try:
            # I2C0 STATUS register offset (TRM Section 10.5)
            STATUS_OFFSET = 0x04

            # Read status register
            status = int(gdb.parse_and_eval(f"*(unsigned int*){I2C0_BASE + STATUS_OFFSET:#x}"))

            print("\n=== I2C0 Status (0x60013004) ===")
            print(f"Raw value: 0x{status:08x}")
            print("\nFlags:")

            # Decode status bits (ESP32-C6 TRM)
            if status & (1 << 0):
                print("  [BUSY] Transaction in progress")
            else:
                print("  [IDLE] No transaction")

            if status & (1 << 5):
                print("  ⚠️  [TIMEOUT] I2C timeout occurred")

            if status & (1 << 10):
                print("  ⚠️  [ACK_ERROR] Slave did not acknowledge")

            if status & (1 << 3):
                print("  [TRANS_COMPLETE] Transaction completed")

            if status == 0:
                print("  ✓ All clear")

            print()

        except gdb.error as e:
            print(f"Error reading I2C registers: {e}")


class ShowGPIO(gdb.Command):
    """Show ESP32-C6 GPIO state for key pins

    Usage: show-gpio

    Displays GPIO input and output state for configured pins.
    """

    def __init__(self):
        super(ShowGPIO, self).__init__("show-gpio", gdb.COMMAND_USER)

    def invoke(self, arg, from_tty):
        try:
            # GPIO register offsets (TRM Section 7.5)
            OUT_OFFSET = 0x04   # GPIO_OUT_REG
            IN_OFFSET = 0x3C    # GPIO_IN_REG

            # Read registers
            gpio_out = int(gdb.parse_and_eval(f"*(unsigned int*){GPIO_BASE + OUT_OFFSET:#x}"))
            gpio_in = int(gdb.parse_and_eval(f"*(unsigned int*){GPIO_BASE + IN_OFFSET:#x}"))

            print("\n=== GPIO State ===")
            print(f"OUT register (0x60004004): 0x{gpio_out:08x}")
            print(f"IN register  (0x6000403C): 0x{gpio_in:08x}")
            print("\nKey Pins:")

            # GPIO8: NeoPixel
            gpio8_out = (gpio_out >> 8) & 1
            print(f"  GPIO8  (NeoPixel): OUT={'HIGH' if gpio8_out else 'LOW'}")

            # GPIO9: Button (active LOW)
            gpio9_in = (gpio_in >> 9) & 1
            button_state = "released" if gpio9_in else "PRESSED"
            print(f"  GPIO9  (Button):   IN={'HIGH' if gpio9_in else 'LOW'} ({button_state})")

            # GPIO2: I2C SDA
            gpio2_in = (gpio_in >> 2) & 1
            print(f"  GPIO2  (I2C SDA):  IN={'HIGH' if gpio2_in else 'LOW'}")

            # GPIO11: I2C SCL
            gpio11_in = (gpio_in >> 11) & 1
            print(f"  GPIO11 (I2C SCL):  IN={'HIGH' if gpio11_in else 'LOW'}")

            print()

        except gdb.error as e:
            print(f"Error reading GPIO registers: {e}")


class ShowAll(gdb.Command):
    """Show all ESP32-C6 peripheral states at once

    Usage: show-all

    Convenience command to display I2C and GPIO status together.
    """

    def __init__(self):
        super(ShowAll, self).__init__("show-all", gdb.COMMAND_USER)

    def invoke(self, arg, from_tty):
        print("\n" + "="*50)
        print("ESP32-C6 Peripheral Status")
        print("="*50)

        # Call other commands
        gdb.execute("show-i2c")
        gdb.execute("show-gpio")


class InspectVar(gdb.Command):
    """Smart variable inspection with type-aware formatting

    Usage: inspect <variable>

    Inspects a variable and formats it based on its type.
    """

    def __init__(self):
        super(InspectVar, self).__init__("inspect", gdb.COMMAND_USER)

    def invoke(self, arg, from_tty):
        if not arg:
            print("Usage: inspect <variable>")
            return

        try:
            val = gdb.parse_and_eval(arg)
            val_type = val.type

            print(f"\n=== Inspecting: {arg} ===")
            print(f"Type: {val_type}")
            print(f"Value: {val}")

            # Try to provide extra context based on type
            type_name = str(val_type)

            if "bool" in type_name.lower():
                print(f"  → {arg} is {'TRUE' if val else 'FALSE'}")

            elif "u8" in type_name or "u16" in type_name or "u32" in type_name:
                print(f"  → Decimal: {int(val)}")
                print(f"  → Hex: 0x{int(val):x}")
                print(f"  → Binary: 0b{int(val):b}")

            print()

        except gdb.error as e:
            print(f"Error: {e}")


# Register all commands
ShowI2C()
ShowGPIO()
ShowAll()
InspectVar()

print("\n✓ ESP32-C6 GDB helpers loaded!")
print("\nAvailable commands:")
print("  show-i2c    - Display I2C0 status with decoded flags")
print("  show-gpio   - Display GPIO state for key pins")
print("  show-all    - Display all peripherals")
print("  inspect VAR - Smart variable inspection\n")

"""
=== How to Find More Registers ===

Official Sources:

1. ESP32-C6 Technical Reference Manual (PDF)
   https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf

   Key chapters:
   - Chapter 7:  GPIO & IO MUX (base 0x60004000)
   - Chapter 10: I2C Controller (base 0x60013000)
   - Chapter 16: RMT (Remote Control) (base 0x60006000)
   - Chapter 25: UART Controller (base 0x60000000, 0x60010000)
   - Chapter 29: USB Serial/JTAG Controller (base 0x60043000)

2. esp32c6 PAC Crate (Rust)
   https://docs.rs/esp32c6/latest/esp32c6/

   View register definitions:
   $ cd ~/.cargo/registry/src/*/esp32c6-*
   $ ls src/          # All peripherals as modules
   $ cat src/i2c0.rs  # I2C0 register definitions

3. SVD Files (Machine-Readable XML)
   https://github.com/espressif/svd

   Used to generate the PAC crate.

Example Usage in Rust Code:
   use esp32c6::{I2C0, GPIO};

   let i2c0 = unsafe { &*I2C0::ptr() };
   let status = i2c0.status.read();
   if status.bus_busy().bit_is_set() { /* ... */ }

Example Usage in GDB:
   (gdb) x/1xw 0x60013004  # Read I2C0 STATUS register
   (gdb) show-i2c          # Use helper script
"""
