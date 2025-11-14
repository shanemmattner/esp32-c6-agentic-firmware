# ESP32-C6 GPIO Register Map

**Source:** esp32c6 PAC crate v0.22.0

## Base Address
```
GPIO Base: 0x60091000
```

## Key Registers for LED Blinking

| Register | Offset | Address | Purpose |
|----------|--------|---------|---------|
| OUT | 0x04 | 0x60091004 | GPIO output value (read/write all bits) |
| OUT_W1TS | 0x08 | 0x60091008 | Write 1 to set (turn ON specific GPIO) |
| OUT_W1TC | 0x0C | 0x6009100C | Write 1 to clear (turn OFF specific GPIO) |
| ENABLE | 0x20 | 0x60091020 | GPIO output enable (read/write all bits) |
| ENABLE_W1TS | 0x24 | 0x60091024 | Write 1 to enable output |
| ENABLE_W1TC | 0x28 | 0x60091028 | Write 1 to disable output |
| IN | 0x3C | 0x6009103C | GPIO input value (read-only) |

## GPIO8 (Onboard LED/NeoPixel)

To control GPIO8, use bit 8 (value: `1 << 8` = `0x100`)

### GDB Commands for GPIO8

```gdb
# Enable GPIO8 as output
(gdb) set *(uint32_t*)0x60091024 = 0x100

# Turn LED ON (set GPIO8 high)
(gdb) set *(uint32_t*)0x60091008 = 0x100

# Turn LED OFF (set GPIO8 low)
(gdb) set *(uint32_t*)0x6009100C = 0x100

# Toggle (read current state, flip bit, write back)
(gdb) set $gpio_out = *(uint32_t*)0x60091004
(gdb) set *(uint32_t*)0x60091004 = $gpio_out ^ 0x100
```

## GPIO9 (BOOT Button)

To read button state (active LOW), use bit 9:

```gdb
# Read GPIO input
(gdb) x/1xw 0x6009103C

# Check if button pressed (bit 9 = 0 means pressed)
(gdb) print (*(uint32_t*)0x6009103C >> 9) & 1
# Returns: 0 = pressed, 1 = not pressed
```

## Register Bit Fields

Each register controls GPIOs 0-31 via individual bits:
- Bit 0 = GPIO0
- Bit 8 = GPIO8 (LED)
- Bit 9 = GPIO9 (Button)
- ...
- Bit 31 = GPIO31

**W1TS (Write 1 to Set):** Writing 1 to a bit sets that GPIO high
**W1TC (Write 1 to Clear):** Writing 1 to a bit clears that GPIO low

This allows atomic bit manipulation without read-modify-write sequences.

## Example: Automated Blinking with GDB

```gdb
# Setup
(gdb) set $led_state = 0
(gdb) set *(uint32_t*)0x60091024 = 0x100  # Enable GPIO8 output

# Create toggle function
define toggle_led
    if $led_state == 0
        set *(uint32_t*)0x60091008 = 0x100
        set $led_state = 1
        printf "LED ON\n"
    else
        set *(uint32_t*)0x6009100C = 0x100
        set $led_state = 0
        printf "LED OFF\n"
    end
end

# Manual toggle
(gdb) toggle_led

# Automated blinking (breakpoint on delay loop)
(gdb) break main.rs:12
(gdb) commands
  > silent
  > toggle_led
  > continue
  > end
(gdb) continue
```

## IO MUX Registers (Advanced)

For GPIO8 to work as a simple output, IO_MUX should already be configured correctly by bootloader.

If GPIO doesn't respond, check IO_MUX configuration:
- IO_MUX base: 0x60090000
- GPIO8 func sel: 0x60090020 + (8 * 4) = 0x60090040

Default function (GPIO) should be 0x01.

## References

- PAC Crate: `esp32c6` v0.22.0
- Datasheet: ESP32-C6 Technical Reference Manual Chapter 5 (GPIO)
