# Tandem Debug Daemon Design (v2.0)
## LLM-Friendly Interface for UART + GDB Variable Streaming

**Version**: 2.0 (Enhanced with GDB Python API)
**Purpose**: Provide structured, machine-readable access to UART stream and GDB state for AI assistants like Claude Code.
**Last Updated**: 2025-01-13

---

## Why We Need This

### The Problem

**For Humans**: Raw UART output and GDB commands are fine
```bash
# UART
A=245|B=102|C=5|D=false

# GDB
(gdb) p SENSOR_X
$1 = 245
(gdb) set SLOT_A.ptr = &TEMPERATURE
```

**For LLMs**: Need structured, queryable data
- Can't easily parse streaming UART text
- GDB interactive session requires complex automation
- No unified view of system state
- No time-series history
- No easy way to trigger actions

### The Solution: Unified Daemon

Single process that:
1. **Reads UART** stream continuously
2. **Controls GDB** via pygdbmi or direct MI interface
3. **Exposes APIs** (HTTP REST + WebSocket)
4. **Maintains state** (current values, history, config)
5. **Provides tools** (CLI, web UI, programmatic access)

---

## Architecture

```
┌──────────────────────────────────────────────────────────┐
│  Tandem Debug Daemon                                     │
│                                                          │
│  ┌────────────────────────────────────────────────────┐ │
│  │  Input Layer                                       │ │
│  │  ┌──────────────┐         ┌──────────────────┐    │ │
│  │  │ UART Reader  │         │ GDB Controller   │    │ │
│  │  │ (pyserial)   │         │ (pygdbmi)        │    │ │
│  │  └──────┬───────┘         └────────┬─────────┘    │ │
│  │         │                           │              │ │
│  └─────────┼───────────────────────────┼──────────────┘ │
│            │                           │                │
│            ▼                           ▼                │
│  ┌────────────────────────────────────────────────────┐ │
│  │  State Manager                                     │ │
│  │  - Current UART values                             │ │
│  │  - Variable metadata (addresses, types)            │ │
│  │  - Slot configuration                              │ │
│  │  - Time-series history                             │ │
│  │  - Change notifications                            │ │
│  └────────────────────────────────────────────────────┘ │
│                          │                              │
│  ┌───────────────────────┴───────────────────────────┐ │
│  │  API Layer                                        │ │
│  │  ┌────────┐  ┌──────────┐  ┌──────────────────┐  │ │
│  │  │REST API│  │WebSocket │  │ CLI Interface    │  │ │
│  │  │(Flask) │  │(aiohttp) │  │ (argparse/rich)  │  │ │
│  │  └────────┘  └──────────┘  └──────────────────┘  │ │
│  └───────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────┘
         │              │                    │
         ▼              ▼                    ▼
   Claude Code    Web Dashboard        Terminal User
```

---

## API Specification

### REST API (HTTP)

#### **GET /api/state**
Returns complete system state.

**Response**:
```json
{
  "timestamp": "2025-01-13T19:45:32.123Z",
  "uart_stream": {
    "A": {"value": 245, "last_updated": "2025-01-13T19:45:32.100Z"},
    "B": {"value": 102, "last_updated": "2025-01-13T19:45:32.100Z"},
    "C": {"value": 5, "last_updated": "2025-01-13T19:45:32.100Z"},
    "D": {"value": false, "last_updated": "2025-01-13T19:45:32.100Z"}
  },
  "variables": {
    "SENSOR_X": {
      "address": "0x3fc80100",
      "type": "i32",
      "value": 245,
      "size": 4
    },
    "SENSOR_Y": {
      "address": "0x3fc80104",
      "type": "i32",
      "value": 102,
      "size": 4
    },
    "TEMPERATURE": {
      "address": "0x3fc8010c",
      "type": "i32",
      "value": 23,
      "size": 4
    }
  },
  "slots": {
    "SLOT_A": {
      "ptr": "0x3fc80100",
      "type": "i32",
      "points_to": "SENSOR_X",
      "current_value": 245
    },
    "SLOT_B": {
      "ptr": "0x3fc80104",
      "type": "i32",
      "points_to": "SENSOR_Y",
      "current_value": 102
    },
    "SLOT_C": {
      "ptr": "0x3fc80118",
      "type": "u16",
      "points_to": "LOOP_COUNTER",
      "current_value": 5
    },
    "SLOT_D": {
      "ptr": "0x3fc8011c",
      "type": "bool",
      "points_to": "LED_STATE",
      "current_value": false
    }
  }
}
```

---

#### **GET /api/variables**
List all available variables with metadata.

**Response**:
```json
{
  "variables": [
    {
      "name": "SENSOR_X",
      "address": "0x3fc80100",
      "type": "i32",
      "size": 4,
      "value": 245,
      "is_streamed": true,
      "streamed_as": "A"
    },
    {
      "name": "TEMPERATURE",
      "address": "0x3fc8010c",
      "type": "i32",
      "size": 4,
      "value": 23,
      "is_streamed": false
    }
  ]
}
```

---

#### **GET /api/slots**
Get current slot configuration.

**Response**:
```json
{
  "SLOT_A": {
    "ptr": "0x3fc80100",
    "type": "i32",
    "points_to": "SENSOR_X"
  },
  "SLOT_B": {
    "ptr": "0x3fc80104",
    "type": "i32",
    "points_to": "SENSOR_Y"
  }
}
```

---

#### **POST /api/redirect**
Change what a slot points to.

**Request**:
```json
{
  "slot": "SLOT_A",
  "variable": "TEMPERATURE"
}
```

**Response**:
```json
{
  "status": "success",
  "slot": "SLOT_A",
  "old_target": "SENSOR_X",
  "new_target": "TEMPERATURE",
  "new_ptr": "0x3fc8010c"
}
```

---

#### **POST /api/set_variable**
Inject a value into a variable via GDB.

**Request**:
```json
{
  "variable": "SENSOR_X",
  "value": 9999
}
```

**Response**:
```json
{
  "status": "success",
  "variable": "SENSOR_X",
  "old_value": 245,
  "new_value": 9999
}
```

---

#### **GET /api/history**
Get time-series data for a variable or slot.

**Query Parameters**:
- `var` or `slot`: Which to query (e.g., `var=SENSOR_X` or `slot=A`)
- `duration`: Seconds of history (default: 60)
- `samples`: Max samples to return (default: 100)

**Request**:
```
GET /api/history?slot=A&duration=60&samples=100
```

**Response**:
```json
{
  "slot": "A",
  "duration_seconds": 60,
  "samples": [
    {"timestamp": "2025-01-13T19:45:30.000Z", "value": 240},
    {"timestamp": "2025-01-13T19:45:30.200Z", "value": 242},
    {"timestamp": "2025-01-13T19:45:30.400Z", "value": 245}
  ]
}
```

---

### WebSocket API

#### **Connection**
```
ws://localhost:8080/stream
```

#### **Events**

**UART Update**:
```json
{
  "event": "uart_update",
  "timestamp": "2025-01-13T19:45:32.123Z",
  "data": {
    "A": 245,
    "B": 102,
    "C": 5,
    "D": false
  }
}
```

**Variable Changed** (via GDB injection):
```json
{
  "event": "variable_changed",
  "timestamp": "2025-01-13T19:45:33.456Z",
  "variable": "SENSOR_X",
  "old_value": 245,
  "new_value": 9999
}
```

**Slot Redirected**:
```json
{
  "event": "slot_redirected",
  "timestamp": "2025-01-13T19:45:34.789Z",
  "slot": "SLOT_A",
  "old_target": "SENSOR_X",
  "new_target": "TEMPERATURE",
  "new_ptr": "0x3fc8010c"
}
```

---

## CLI Interface

### Starting the Daemon

```bash
# Basic usage
$ python3 tandem_daemon.py \
    --uart /dev/cu.usbserial-A50285BI \
    --gdb localhost:3333

Tandem Debug Daemon v1.0
========================
✓ UART connected: /dev/cu.usbserial-A50285BI @ 115200
✓ GDB connected: localhost:3333
✓ REST API: http://localhost:8080
✓ WebSocket: ws://localhost:8080/stream
✓ Ready for connections

[19:45:32] UART: A=245|B=102|C=5|D=false
[19:45:32] Parsed: 4 variables updated
```

### Interactive Commands

```bash
# In daemon REPL
> status
Connected to firmware. Streaming 4 slots.

> list vars
Available variables:
  SENSOR_X      @ 0x3fc80100 = 245 (i32) [streamed as A]
  SENSOR_Y      @ 0x3fc80104 = 102 (i32) [streamed as B]
  TEMPERATURE   @ 0x3fc8010c = 23  (i32)
  ...

> redirect A TEMPERATURE
✓ SLOT_A -> TEMPERATURE (was SENSOR_X)

> set SENSOR_X 9999
✓ SENSOR_X = 9999 (was 245)

> history A 30
Last 30 seconds of slot A:
  [19:45:00] 240
  [19:45:05] 242
  [19:45:10] 245
  ...
```

---

## Claude Code Integration

### Example: LLM Query via API

**Claude Code**: "What variables are currently being streamed?"

**Request**:
```bash
curl http://localhost:8080/api/slots
```

**Response**:
```json
{
  "SLOT_A": {"points_to": "SENSOR_X", "value": 245},
  "SLOT_B": {"points_to": "SENSOR_Y", "value": 102},
  "SLOT_C": {"points_to": "LOOP_COUNTER", "value": 5},
  "SLOT_D": {"points_to": "LED_STATE", "value": false}
}
```

**Claude Code responds**: "Currently streaming SENSOR_X (245), SENSOR_Y (102), LOOP_COUNTER (5), and LED_STATE (false)."

---

### Example: LLM Action via API

**User to Claude**: "Show me the temperature instead of SENSOR_X"

**Claude Code executes**:
```python
import requests
response = requests.post('http://localhost:8080/api/redirect', json={
    "slot": "SLOT_A",
    "variable": "TEMPERATURE"
})
```

**Response**:
```json
{"status": "success", "new_target": "TEMPERATURE"}
```

**Claude Code responds**: "Done! Slot A now shows TEMPERATURE (currently 23°C)."

---

### Example: LLM Real-Time Monitoring

**Claude Code** (via WebSocket):
```python
import asyncio
import websockets

async def monitor():
    uri = "ws://localhost:8080/stream"
    async with websockets.connect(uri) as ws:
        async for message in ws:
            data = json.loads(message)
            if data['event'] == 'uart_update':
                # Claude can react to data changes
                if data['data']['A'] > 1000:
                    print("⚠️  Alert: Sensor A exceeded threshold!")
```

---

## Implementation Structure

```python
# tandem_daemon.py

import serial
import asyncio
import json
from flask import Flask, jsonify, request
from flask_cors import CORS
from pygdbmi.gdbcontroller import GdbController
import websockets

class TandemDebugDaemon:
    def __init__(self, uart_port, gdb_port=3333):
        # UART connection
        self.uart = serial.Serial(uart_port, 115200, timeout=0.1)

        # GDB connection
        self.gdb = GdbController()
        self.gdb.write(f"target extended-remote :{gdb_port}")

        # State storage
        self.state = {
            "uart_stream": {},
            "variables": {},
            "slots": {},
            "history": []
        }

        # WebSocket clients
        self.ws_clients = set()

    async def uart_reader_task(self):
        """Continuously read UART and update state"""
        while True:
            if self.uart.in_waiting:
                line = self.uart.readline().decode('utf-8').strip()
                # Parse: "A=245|B=102|C=5|D=false"
                parsed = self.parse_uart_line(line)
                self.update_state(parsed)
                await self.broadcast_update(parsed)
            await asyncio.sleep(0.01)

    def parse_uart_line(self, line):
        """Parse UART stream format"""
        fields = line.split('|')
        data = {}
        for field in fields:
            name, value = field.split('=')
            # Infer type and convert
            if value == 'true':
                data[name] = True
            elif value == 'false':
                data[name] = False
            else:
                data[name] = int(value)
        return data

    def redirect_slot(self, slot_name, variable_name):
        """Redirect a slot pointer via GDB"""
        # Get variable address
        result = self.gdb.write(f"p/x &{variable_name}")
        addr = self.parse_gdb_address(result)

        # Set slot pointer
        self.gdb.write(f"set {slot_name}.ptr = (unsigned char*){addr}")

        return {
            "status": "success",
            "slot": slot_name,
            "new_target": variable_name,
            "new_ptr": addr
        }

    def set_variable(self, variable_name, value):
        """Inject value into variable via GDB"""
        self.gdb.write(f"set {variable_name} = {value}")
        return {"status": "success"}

    async def broadcast_update(self, data):
        """Send update to all WebSocket clients"""
        message = json.dumps({
            "event": "uart_update",
            "data": data
        })
        for client in self.ws_clients:
            await client.send(message)

# Flask REST API
app = Flask(__name__)
CORS(app)
daemon = None  # Global instance

@app.route('/api/state')
def get_state():
    return jsonify(daemon.state)

@app.route('/api/redirect', methods=['POST'])
def redirect():
    data = request.json
    result = daemon.redirect_slot(data['slot'], data['variable'])
    return jsonify(result)

@app.route('/api/set_variable', methods=['POST'])
def set_var():
    data = request.json
    result = daemon.set_variable(data['variable'], data['value'])
    return jsonify(result)

# Main entry point
if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument('--uart', required=True)
    parser.add_argument('--gdb-port', type=int, default=3333)
    args = parser.parse_args()

    daemon = TandemDebugDaemon(args.uart, args.gdb_port)

    # Start Flask in thread
    # Start WebSocket in thread
    # Start UART reader in thread
    # Run event loop
```

---

## Benefits for LLMs

### 1. **Structured Queries**
```python
# Claude can ask:
GET /api/variables
# Instead of parsing:
(gdb) info variables
```

### 2. **Programmatic Actions**
```python
# Claude can execute:
POST /api/redirect {"slot": "A", "variable": "TEMPERATURE"}
# Instead of:
(gdb) set SLOT_A.ptr = 0x3fc8010c
```

### 3. **Real-Time Awareness**
```python
# Claude receives live updates:
ws://localhost:8080/stream
# Instead of polling UART output
```

### 4. **Historical Context**
```python
# Claude can ask:
GET /api/history?var=SENSOR_X&duration=60
# Gets 60 seconds of data instantly
```

---

## Deployment Options

### Option 1: Standalone Daemon
```bash
# User runs separately
$ python3 tandem_daemon.py --uart /dev/ttyUSB0
```

### Option 2: Claude Code Auto-Start
```python
# Claude detects lesson 08, starts daemon automatically
subprocess.Popen(['python3', 'tandem_daemon.py', '--uart', port])
```

### Option 3: Docker Container
```bash
# Containerized for portability
$ docker run -p 8080:8080 --device=/dev/ttyUSB0 tandem-daemon
```

---

## Questions to Decide

1. **Complexity**: Simple HTTP-only or full WebSocket support?
2. **Dependencies**: Use Flask/aiohttp or keep minimal?
3. **Auto-discovery**: Should daemon auto-find UART port?
4. **Persistence**: Save history to file/database?
5. **Security**: API authentication needed?

---

## Recommendation

**Start with minimal HTTP-only daemon**:
- Flask REST API (5 endpoints)
- No WebSocket initially (KISS principle)
- JSON responses only
- Claude can poll every 1-2 seconds

**Later add**:
- WebSocket for real-time (if needed)
- History database (SQLite)
- Web dashboard (optional)

This keeps the lesson focused on UART+GDB, with daemon as **enabler** not **focus**.

---

**Should I implement the minimal daemon alongside the firmware?** It would make Lesson 08 much more powerful for LLM interaction! 🤖
