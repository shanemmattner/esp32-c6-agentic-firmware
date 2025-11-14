# Find ESP32-C6 Peripheral Registers

Search the ESP32-C6 PAC crate for peripheral base addresses and register offsets.

This teaches how to discover hardware registers from Rust source code instead of reading datasheets.

## Usage

```
/find-registers <peripheral>
/find-registers --all
```

## What This Command Does

1. Locates the `esp32c6` PAC crate in your cargo registry
2. Searches `src/lib.rs` for peripheral base addresses
3. Parses peripheral module files (e.g., `gpio.rs`) for register offsets
4. Calculates absolute addresses (base + offset)
5. Displays register map with descriptions
6. Optionally generates GDB commands

## Examples

```bash
/find-registers GPIO
/find-registers UART
/find-registers I2C0
/find-registers --all
```

## When You're Asked

When the user asks to find registers or needs hardware addresses:

1. **Run the tool:**
   ```bash
   python3 scripts/find-registers.py <PERIPHERAL>
   ```

2. **Show the output:**
   - Base address
   - Register list with offsets
   - Absolute addresses

3. **Explain the discovery process:**
   > "I found these registers by searching the esp32c6 PAC crate source code.
   > The PAC is auto-generated from Espressif's SVD files, so it's always accurate.
   > No datasheet needed!"

4. **Highlight key registers for the task:**
   - For LED control: `OUT_W1TS`, `OUT_W1TC`, `ENABLE_W1TS`
   - For reading inputs: `IN`
   - For UART: `FIFO`, `STATUS`

5. **Generate GDB commands if helpful:**
   ```bash
   python3 scripts/find-registers.py GPIO --gdb
   ```

## Example Interaction

**User asks:** "How do I find the GPIO registers?"

**You respond:**
```
Let me search the PAC crate for GPIO registers...

[runs: python3 scripts/find-registers.py GPIO]

Found GPIO peripheral!

Base Address: 0x60091000

Key registers for LED control:
- ENABLE_W1TS (0x24): 0x60091024 - Enable GPIO as output
- OUT_W1TS (0x08): 0x60091008 - Set GPIO high (turn ON)
- OUT_W1TC (0x0C): 0x6009100C - Clear GPIO low (turn OFF)

For GPIO8 (your LED), use bit mask: 0x100 (1 << 8)

Here's how we discovered this: The esp32c6 PAC crate contains all
register definitions in Rust source code. I searched src/lib.rs for
the GPIO base address, then src/gpio.rs for register offsets.

Want me to generate GDB commands for these registers?
```

## Educational Value

This teaches:
1. **Register discovery workflow** - Find hardware info from source code
2. **PAC crate structure** - Understanding auto-generated peripheral access
3. **Address calculation** - Base + offset = absolute address
4. **Rust for embedded** - Hardware abstraction is self-documenting
5. **Practical GDB** - How to translate register info into GDB commands

## Troubleshooting

**If PAC crate not found:**
```bash
# Add esp32c6 PAC to any project to download it
cargo add esp32c6
```

**If peripheral not found:**
```bash
# List all available peripherals
python3 scripts/find-registers.py --all
```

## Related Tools

- `GPIO_REGISTERS.md` - Pre-documented GPIO register map
- `/gdb-blinky` - Interactive lesson using discovered registers
- `gdb_blinky.gdb` - GDB script generated from register addresses

## Source Code

See `scripts/find-registers.py` for the implementation.

The tool uses regex to parse:
- `src/lib.rs`: Base addresses
- `src/<peripheral>.rs`: Register offsets and descriptions

This is faster than reading the TRM and always matches the HAL version you're using.
