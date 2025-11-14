# Future Lessons

This directory contains lessons that depend on features not yet working on the current hardware/tooling setup.

## Lessons 08-09: RTT-Based Logging (Deferred)

**Original Goal:** High-speed structured logging using SEGGER RTT (Real-Time Transfer) via JTAG

**Why Deferred:**
- probe-rs 0.30.0 has incomplete RTT support for ESP32-C6 built-in USB-JTAG on macOS
- Error: "Failed to attach to RTT: Timeout" with USB disconnection issues
- Extensive debugging documented in `08-defmt-rtt-logging/COMPREHENSIVE_DEBUG_REPORT.md`

**Status:**
- Code is correct and well-structured
- Configuration is verified (defmt, defmt-rtt, RTT control block at 0x40800dbc)
- Issue is in probe-rs tool layer, not our code
- RTT may work with:
  - External J-Link probe (requires proper wiring)
  - Linux host (may have better USB driver support)
  - Future probe-rs versions with improved ESP32-C6 support
  - OpenOCD instead of probe-rs

**See Also:**
- `08-defmt-rtt-logging/RTT_DEBUGGING_LOG.md` - Technical investigation
- `08-defmt-rtt-logging/COMPREHENSIVE_DEBUG_REPORT.md` - 20-page debugging report
- `.claude/commands/esp32-debug.md` - Updated with RTT debugging techniques

**When to Revisit:**
1. probe-rs releases improved ESP32-C6 RTT support
2. J-Link probe with proper connector available
3. Test on Linux platform
4. Or use as reference for future RTT work

---

## Replacement: Lessons 08-09 (USB CDC High-Speed Streaming)

**New Approach:** Achieve same logging goals using USB CDC (virtual serial port)
- ✅ Works with existing ESP32-C6 USB connection
- ✅ High-speed data streaming (up to 12 Mbps USB Full Speed)
- ✅ Structured logging with machine-parseable format
- ✅ GDB integration for debugging
- ✅ Python parser for real-time data visualization

**Location:** `lessons/08-usb-cdc-streaming/` (to be created)

---

**Last Updated:** 2025-11-12
**Reason:** probe-rs RTT compatibility issues with ESP32-C6 USB-JTAG on macOS
