# Lesson Flow Plan
## ESP32-C6 Modern Rust Firmware Development

**Philosophy**: Start simple → Add concurrency → Add state → Add peripherals → Add connectivity

**Stack**: esp-hal 1.0.0 + Embassy + Simple Enums

---

## 📚 Overview

This learning path teaches modern embedded Rust development through progressive complexity:
- **Phase 1**: Foundation (blocking code, basic GPIO)
- **Phase 2**: Concurrency (Embassy async/await)
- **Phase 3**: State Management (enum-based FSM)
- **Phase 4**: Peripherals (I2C, SPI, UART)
- **Phase 5**: Connectivity (WiFi, BLE)
- **Phase 6**: Complete Systems

---

## 🎯 Complete Lesson Sequence

### Phase 1: Foundation (Weeks 1-2)

#### ✅ Lesson 01: Blinky
**Status**: Complete
**Duration**: 15 min
**Concepts**: GPIO output, blocking delay, logging
**Hardware**: LED on GPIO8

**What you learn:**
- esp-hal 1.0.0 initialization
- GPIO output configuration
- Blocking delays
- Serial logging patterns

**Code**: `lessons/01-blinky/`

---

#### 🔜 Lesson 02: Button Input + Debouncing
**Duration**: 20 min
**Concepts**: GPIO input, pull-up resistors, debouncing
**Hardware**: Button on GPIO9, LED on GPIO8

**What you learn:**
- GPIO input configuration
- Reading digital inputs
- Software debouncing
- Simple state tracking (button pressed/released)

**Key code pattern:**
```rust
let button = Input::new(peripherals.GPIO9, Pull::Up);
loop {
    if button.is_low() {
        // Button pressed (active low)
        led.set_high();
    } else {
        led.set_low();
    }
    delay.delay_millis(50); // Debounce
}
```

**Skills unlocked:**
- ✅ Read GPIO inputs
- ✅ Handle button debouncing
- ✅ Coordinate input and output

---

#### 🔜 Lesson 03: Simple State Machine (LED Modes)
**Duration**: 25 min
**Concepts**: Enum-based state, match expressions
**Hardware**: Button on GPIO9, LED on GPIO8

**What you learn:**
- State management with enums
- Match expressions for state transitions
- Button press detection for mode switching

**Key code pattern:**
```rust
enum LedMode {
    Off,
    On,
    SlowBlink,
    FastBlink,
}

let mut mode = LedMode::Off;

loop {
    if button_pressed() {
        mode = match mode {
            LedMode::Off => LedMode::On,
            LedMode::On => LedMode::SlowBlink,
            LedMode::SlowBlink => LedMode::FastBlink,
            LedMode::FastBlink => LedMode::Off,
        };
    }

    match mode {
        LedMode::Off => led.set_low(),
        LedMode::On => led.set_high(),
        LedMode::SlowBlink => { /* blink slow */ },
        LedMode::FastBlink => { /* blink fast */ },
    }
}
```

**Skills unlocked:**
- ✅ Enum-based state machines
- ✅ State transitions
- ✅ Simple user interaction patterns

---

### Phase 2: Async Concurrency with Embassy (Weeks 3-4)

#### 🔜 Lesson 04: Embassy Async Blinky
**Duration**: 30 min
**Concepts**: Embassy executor, async tasks, non-blocking delays
**Hardware**: LED on GPIO8

**What you learn:**
- Embassy executor setup
- Async task definition
- Non-blocking delays with `Timer::after()`
- Difference between blocking and async code

**Key code pattern:**
```rust
#[embassy_executor::task]
async fn blink_task() {
    loop {
        led.set_high();
        Timer::after_millis(1000).await;
        led.set_low();
        Timer::after_millis(1000).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    spawner.spawn(blink_task()).unwrap();
}
```

**Skills unlocked:**
- ✅ Embassy executor
- ✅ Async/await syntax
- ✅ Non-blocking delays
- ✅ Task spawning

---

#### 🔜 Lesson 05: Multiple Concurrent Tasks
**Duration**: 30 min
**Concepts**: Multiple async tasks, task concurrency
**Hardware**: 3 LEDs on GPIO8, GPIO9, GPIO10

**What you learn:**
- Running multiple tasks concurrently
- Independent task timing
- Resource ownership (each LED owned by its task)

**Key code pattern:**
```rust
#[embassy_executor::task]
async fn blink_slow(mut led: Output<'static>) {
    loop {
        led.toggle();
        Timer::after_millis(1000).await;
    }
}

#[embassy_executor::task]
async fn blink_fast(mut led: Output<'static>) {
    loop {
        led.toggle();
        Timer::after_millis(200).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    spawner.spawn(blink_slow(led1)).unwrap();
    spawner.spawn(blink_fast(led2)).unwrap();
}
```

**Skills unlocked:**
- ✅ Concurrent task execution
- ✅ Independent timing for each task
- ✅ Resource ownership patterns

---

#### 🔜 Lesson 06: Task Communication with Channels
**Duration**: 35 min
**Concepts**: Embassy channels, task communication
**Hardware**: Button on GPIO9, LED on GPIO8

**What you learn:**
- Channel creation and usage
- Sending messages between tasks
- Producer-consumer patterns

**Key code pattern:**
```rust
enum Command {
    TurnOn,
    TurnOff,
    Blink(u32), // blink interval in ms
}

static CHANNEL: Channel<ThreadModeRawMutex, Command, 10> = Channel::new();

#[embassy_executor::task]
async fn button_task() {
    loop {
        if button.wait_for_low().await.is_ok() {
            CHANNEL.send(Command::Blink(500)).await;
        }
    }
}

#[embassy_executor::task]
async fn led_task() {
    loop {
        match CHANNEL.receive().await {
            Command::TurnOn => led.set_high(),
            Command::TurnOff => led.set_low(),
            Command::Blink(interval) => { /* blink logic */ },
        }
    }
}
```

**Skills unlocked:**
- ✅ Inter-task communication
- ✅ Message passing
- ✅ Async channels
- ✅ Producer-consumer pattern

---

#### 🔜 Lesson 07: Shared State with Mutexes
**Duration**: 30 min
**Concepts**: Shared mutable state, Embassy Mutex
**Hardware**: Button on GPIO9, LED on GPIO8

**What you learn:**
- Sharing state between tasks
- Mutex for safe concurrent access
- Lock-free alternatives (Signal)

**Key code pattern:**
```rust
static COUNTER: Mutex<ThreadModeRawMutex, u32> = Mutex::new(0);

#[embassy_executor::task]
async fn increment_task() {
    loop {
        Timer::after_secs(1).await;
        let mut counter = COUNTER.lock().await;
        *counter += 1;
    }
}

#[embassy_executor::task]
async fn display_task() {
    loop {
        Timer::after_secs(5).await;
        let counter = COUNTER.lock().await;
        info!("Count: {}", *counter);
    }
}
```

**Skills unlocked:**
- ✅ Shared state management
- ✅ Mutex usage
- ✅ Lock/unlock patterns
- ✅ Race condition prevention

---

### Phase 3: Real-World State Machines (Week 5)

#### 🔜 Lesson 08: Traffic Light FSM
**Duration**: 40 min
**Concepts**: Complex state machine, timed transitions
**Hardware**: 3 LEDs (Red, Yellow, Green)

**What you learn:**
- Multi-state finite state machine
- Timed state transitions
- Safety-critical state logic

**Key code pattern:**
```rust
enum TrafficState {
    Red,
    RedYellow,
    Green,
    Yellow,
}

#[embassy_executor::task]
async fn traffic_light() {
    let mut state = TrafficState::Red;

    loop {
        match state {
            TrafficState::Red => {
                set_lights(true, false, false);
                Timer::after_secs(5).await;
                state = TrafficState::RedYellow;
            }
            TrafficState::RedYellow => {
                set_lights(true, true, false);
                Timer::after_secs(2).await;
                state = TrafficState::Green;
            }
            TrafficState::Green => {
                set_lights(false, false, true);
                Timer::after_secs(5).await;
                state = TrafficState::Yellow;
            }
            TrafficState::Yellow => {
                set_lights(false, true, false);
                Timer::after_secs(2).await;
                state = TrafficState::Red;
            }
        }
    }
}
```

**Skills unlocked:**
- ✅ Complex state machines
- ✅ Timed transitions
- ✅ Multiple outputs coordination
- ✅ Safety logic patterns

---

#### 🔜 Lesson 09: State with Data (Smart Thermostat)
**Duration**: 45 min
**Concepts**: Enum variants with data, stateful logic
**Hardware**: Button (temperature sensor simulator), LED (heater)

**What you learn:**
- Enum variants carrying data
- State-dependent logic
- Threshold-based control

**Key code pattern:**
```rust
enum ThermostatState {
    Idle { current_temp: f32 },
    Heating { target_temp: f32, current_temp: f32 },
    Cooling { duration_secs: u32 },
}

#[embassy_executor::task]
async fn thermostat() {
    let mut state = ThermostatState::Idle { current_temp: 20.0 };

    loop {
        state = match state {
            ThermostatState::Idle { current_temp } => {
                let temp = read_temp().await;
                if temp < 18.0 {
                    ThermostatState::Heating {
                        target_temp: 21.0,
                        current_temp: temp
                    }
                } else {
                    ThermostatState::Idle { current_temp: temp }
                }
            }
            ThermostatState::Heating { target_temp, current_temp } => {
                heater.set_high();
                Timer::after_secs(5).await;
                let temp = read_temp().await;

                if temp >= target_temp {
                    heater.set_low();
                    ThermostatState::Cooling { duration_secs: 10 }
                } else {
                    ThermostatState::Heating { target_temp, current_temp: temp }
                }
            }
            ThermostatState::Cooling { duration_secs } => {
                if duration_secs == 0 {
                    ThermostatState::Idle { current_temp: read_temp().await }
                } else {
                    Timer::after_secs(1).await;
                    ThermostatState::Cooling { duration_secs: duration_secs - 1 }
                }
            }
        };
    }
}
```

**Skills unlocked:**
- ✅ Enum variants with data
- ✅ Stateful decision making
- ✅ Real-world control logic
- ✅ Threshold-based systems

---

### Phase 4: Peripheral Communication (Weeks 6-7)

#### 🔜 Lesson 10: Async I2C - Temperature Sensor
**Duration**: 45 min
**Concepts**: I2C protocol, async sensor reading, embedded-hal traits
**Hardware**: I2C temperature sensor (e.g., TMP102, BME280)

**What you learn:**
- I2C initialization
- Async I2C transactions
- Reading sensor data
- embedded-hal 1.0 traits

**Key code pattern:**
```rust
use esp_hal::i2c::I2c;
use embedded_hal_async::i2c::I2c as I2cTrait;

#[embassy_executor::task]
async fn sensor_task(mut i2c: I2c<'static, Async>) {
    loop {
        // Read temperature from sensor
        let mut buffer = [0u8; 2];
        match i2c.write_read(SENSOR_ADDR, &[TEMP_REG], &mut buffer).await {
            Ok(_) => {
                let temp = i16::from_be_bytes(buffer) as f32 / 256.0;
                info!("Temperature: {:.2}°C", temp);
            }
            Err(e) => error!("I2C error: {:?}", e),
        }
        Timer::after_secs(2).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let i2c = I2c::new_with_timeout_async(
        peripherals.I2C0,
        peripherals.GPIO6,  // SDA
        peripherals.GPIO7,  // SCL
        100.kHz(),
        Duration::from_millis(100),
    );

    spawner.spawn(sensor_task(i2c)).unwrap();
}
```

**Skills unlocked:**
- ✅ I2C peripheral configuration
- ✅ Async I2C transactions
- ✅ Sensor data parsing
- ✅ Error handling for peripherals
- ✅ embedded-hal traits usage

---

#### 🔜 Lesson 11: I2C Device Driver Pattern
**Duration**: 50 min
**Concepts**: Driver abstraction, device struct, API design
**Hardware**: I2C sensor (BME280 or similar)

**What you learn:**
- Creating reusable driver structs
- Async methods
- Error handling patterns
- Clean API design

**Key code pattern:**
```rust
struct Bme280<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> Bme280<I2C>
where
    I2C: I2cTrait,
{
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }

    pub async fn init(&mut self) -> Result<(), Error> {
        // Read chip ID
        let chip_id = self.read_register(CHIP_ID_REG).await?;
        if chip_id != BME280_CHIP_ID {
            return Err(Error::InvalidChipId);
        }

        // Configure sensor
        self.write_register(CTRL_MEAS_REG, 0x27).await?;
        Ok(())
    }

    pub async fn read_temperature(&mut self) -> Result<f32, Error> {
        let mut buffer = [0u8; 3];
        self.read_registers(TEMP_MSB_REG, &mut buffer).await?;

        let raw = ((buffer[0] as u32) << 12)
                | ((buffer[1] as u32) << 4)
                | ((buffer[2] as u32) >> 4);

        Ok(self.compensate_temperature(raw))
    }

    async fn read_register(&mut self, reg: u8) -> Result<u8, Error> {
        let mut buffer = [0u8; 1];
        self.i2c.write_read(self.address, &[reg], &mut buffer).await
            .map_err(|_| Error::I2cError)?;
        Ok(buffer[0])
    }
}

#[embassy_executor::task]
async fn sensor_task(i2c: I2c<'static, Async>) {
    let mut sensor = Bme280::new(i2c, BME280_ADDRESS);

    if let Err(e) = sensor.init().await {
        error!("Failed to initialize sensor: {:?}", e);
        return;
    }

    loop {
        match sensor.read_temperature().await {
            Ok(temp) => info!("Temperature: {:.2}°C", temp),
            Err(e) => error!("Read error: {:?}", e),
        }
        Timer::after_secs(2).await;
    }
}
```

**Skills unlocked:**
- ✅ Driver struct pattern
- ✅ Generic I2C trait usage
- ✅ Async driver methods
- ✅ Proper error propagation
- ✅ Reusable driver design

---

#### 🔜 Lesson 12: SPI Display Driver
**Duration**: 60 min
**Concepts**: SPI protocol, display control, DMA
**Hardware**: SPI display (ST7789, ILI9341, or similar)

**What you learn:**
- SPI peripheral configuration
- Command/data protocol
- Display initialization
- Async SPI with DMA

**Key code pattern:**
```rust
use esp_hal::spi::{master::Spi, SpiMode};
use esp_hal::dma::{Dma, DmaChannel0};

struct Display<SPI> {
    spi: SPI,
    dc: Output<'static>,   // Data/Command pin
    rst: Output<'static>,  // Reset pin
}

impl<SPI> Display<SPI>
where
    SPI: embedded_hal_async::spi::SpiBus,
{
    pub async fn init(&mut self) -> Result<(), Error> {
        // Hardware reset
        self.rst.set_low();
        Timer::after_millis(10).await;
        self.rst.set_high();
        Timer::after_millis(120).await;

        // Send init commands
        self.write_command(0x01).await?; // Software reset
        Timer::after_millis(150).await;

        self.write_command(0x11).await?; // Sleep out
        Timer::after_millis(255).await;

        self.write_command(0x29).await?; // Display on

        Ok(())
    }

    pub async fn fill_screen(&mut self, color: u16) -> Result<(), Error> {
        self.set_window(0, 0, 240, 320).await?;

        let pixel_count = 240 * 320;
        let color_bytes = color.to_be_bytes();

        self.dc.set_high(); // Data mode
        for _ in 0..pixel_count {
            self.spi.write(&color_bytes).await
                .map_err(|_| Error::SpiError)?;
        }

        Ok(())
    }

    async fn write_command(&mut self, cmd: u8) -> Result<(), Error> {
        self.dc.set_low(); // Command mode
        self.spi.write(&[cmd]).await
            .map_err(|_| Error::SpiError)?;
        Ok(())
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let dma = Dma::new(peripherals.DMA);
    let dma_channel = dma.channel0;

    let spi = Spi::new_with_config(
        peripherals.SPI2,
        esp_hal::spi::master::Config {
            frequency: 40.MHz(),
            mode: SpiMode::Mode0,
            ..Default::default()
        },
    )
    .with_sck(peripherals.GPIO6)
    .with_mosi(peripherals.GPIO7)
    .with_dma(dma_channel);

    let dc = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
    let rst = Output::new(peripherals.GPIO9, Level::High, OutputConfig::default());

    let mut display = Display::new(spi, dc, rst);
    display.init().await.unwrap();

    loop {
        display.fill_screen(0xF800).await.unwrap(); // Red
        Timer::after_secs(1).await;
        display.fill_screen(0x07E0).await.unwrap(); // Green
        Timer::after_secs(1).await;
    }
}
```

**Skills unlocked:**
- ✅ SPI peripheral configuration
- ✅ DMA setup and usage
- ✅ Display control protocols
- ✅ Command/data modes
- ✅ High-speed data transfer

---

#### 🔜 Lesson 13: UART Communication
**Duration**: 40 min
**Concepts**: Serial communication, async UART, AT commands
**Hardware**: UART GPS module or serial sensor

**What you learn:**
- UART configuration
- Async reading/writing
- Parsing serial data
- Buffered communication

**Key code pattern:**
```rust
use esp_hal::uart::Uart;

#[embassy_executor::task]
async fn uart_task(mut uart: Uart<'static, Async>) {
    let mut buffer = [0u8; 256];

    loop {
        // Read data asynchronously
        match uart.read_async(&mut buffer).await {
            Ok(len) => {
                if let Ok(s) = core::str::from_utf8(&buffer[..len]) {
                    info!("Received: {}", s);

                    // Echo back
                    uart.write_async(b"ACK\r\n").await.ok();
                }
            }
            Err(e) => error!("UART error: {:?}", e),
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let uart = Uart::new_async_with_config(
        peripherals.UART1,
        esp_hal::uart::Config {
            baudrate: 115200,
            data_bits: esp_hal::uart::DataBits::DataBits8,
            parity: esp_hal::uart::Parity::ParityNone,
            stop_bits: esp_hal::uart::StopBits::STOP1,
            ..Default::default()
        },
        peripherals.GPIO4,  // TX
        peripherals.GPIO5,  // RX
    );

    spawner.spawn(uart_task(uart)).unwrap();
}
```

**Skills unlocked:**
- ✅ UART configuration
- ✅ Async serial communication
- ✅ Data parsing
- ✅ Buffered I/O

---

### Phase 5: Connectivity (Weeks 8-9)

#### 🔜 Lesson 14: WiFi Connection Basics
**Duration**: 60 min
**Concepts**: WiFi initialization, connection, esp-wifi crate
**Hardware**: ESP32-C6 (built-in WiFi 6)

**What you learn:**
- esp-wifi setup
- WiFi connection flow
- Network events
- DHCP client

**Key code pattern:**
```rust
use esp_wifi::{initialize, wifi::{ClientConfiguration, Configuration, WifiController, WifiDevice, WifiEvent, WifiStaDevice, WifiState}};
use embassy_net::{Config, Stack, StackResources};

#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    loop {
        if matches!(controller.is_started(), Ok(false)) {
            controller.start().await.unwrap();
        }

        match controller.connect().await {
            Ok(_) => info!("WiFi connected!"),
            Err(e) => {
                error!("Failed to connect: {:?}", e);
                Timer::after_secs(5).await;
            }
        }
    }
}

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<WifiDevice<'static, WifiStaDevice>>) {
    stack.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let init = initialize(
        EspWifiInitFor::Wifi,
        timg0.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    ).unwrap();

    let wifi = peripherals.WIFI;
    let (wifi_interface, controller) = esp_wifi::wifi::new_with_mode(&init, wifi, WifiStaDevice).unwrap();

    let config = Config::dhcpv4(Default::default());

    let seed = 1234; // Use RNG in production
    let stack = &*mk_static!(
        Stack<'static, WifiDevice<'static, WifiStaDevice>>,
        Stack::new(
            wifi_interface,
            config,
            mk_static!(StackResources<3>, StackResources::new()),
            seed
        )
    );

    spawner.spawn(connection(controller)).unwrap();
    spawner.spawn(net_task(stack)).unwrap();

    // Wait for DHCP
    loop {
        if stack.is_link_up() {
            break;
        }
        Timer::after_millis(500).await;
    }

    info!("Waiting for IP...");
    loop {
        if let Some(config) = stack.config_v4() {
            info!("IP: {}", config.address);
            break;
        }
        Timer::after_millis(500).await;
    }
}
```

**Skills unlocked:**
- ✅ WiFi initialization
- ✅ Network stack setup
- ✅ DHCP client
- ✅ Connection management

---

#### 🔜 Lesson 15: HTTP Client (REST API)
**Duration**: 60 min
**Concepts**: TCP sockets, HTTP requests, JSON parsing
**Hardware**: ESP32-C6 with WiFi

**What you learn:**
- TCP socket usage
- HTTP request formatting
- Response parsing
- Simple REST client

**Key code pattern:**
```rust
use embassy_net::{Stack, tcp::TcpSocket};
use embedded_io_async::Write;

#[embassy_executor::task]
async fn http_task(stack: &'static Stack<WifiDevice<'static, WifiStaDevice>>) {
    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];

    loop {
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(10)));

        let remote_endpoint = (Ipv4Address::new(93, 184, 216, 34), 80); // example.com

        info!("Connecting to {:?}...", remote_endpoint);
        match socket.connect(remote_endpoint).await {
            Ok(()) => {
                info!("Connected!");

                let request = b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
                socket.write_all(request).await.unwrap();

                let mut buffer = [0; 1024];
                match socket.read(&mut buffer).await {
                    Ok(n) => {
                        if let Ok(response) = core::str::from_utf8(&buffer[..n]) {
                            info!("Response:\n{}", response);
                        }
                    }
                    Err(e) => error!("Read error: {:?}", e),
                }
            }
            Err(e) => error!("Connect error: {:?}", e),
        }

        Timer::after_secs(30).await;
    }
}
```

**Skills unlocked:**
- ✅ TCP socket usage
- ✅ HTTP protocol basics
- ✅ Request/response handling
- ✅ Network error handling

---

#### 🔜 Lesson 16: MQTT Client (IoT Messaging)
**Duration**: 60 min
**Concepts**: MQTT protocol, pub/sub, QoS
**Hardware**: ESP32-C6 with WiFi

**What you learn:**
- MQTT client setup
- Publishing messages
- Subscribing to topics
- IoT messaging patterns

**Key code pattern:**
```rust
use rust_mqtt::{client::{client::MqttClient, client_config::ClientConfig}, packet::v5::reason_codes::ReasonCode};

#[embassy_executor::task]
async fn mqtt_task(stack: &'static Stack<WifiDevice<'static, WifiStaDevice>>) {
    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];

    let mut config = ClientConfig::new(
        rust_mqtt::client::client_config::MqttVersion::MQTTv5,
        CountingRng(20000),
    );
    config.add_max_subscribe_qos(rust_mqtt::packet::v5::publish_packet::QualityOfService::QoS1);
    config.add_client_id("esp32c6-client");
    config.max_packet_size = 4096;

    let mut client = MqttClient::<_, 5, _>::new(
        stack,
        &mut rx_buffer,
        4096,
        &mut tx_buffer,
        4096,
        config,
    );

    match client.connect_to_broker("broker.hivemq.com").await {
        Ok(()) => info!("Connected to MQTT broker"),
        Err(e) => {
            error!("Failed to connect: {:?}", e);
            return;
        }
    }

    // Subscribe to topic
    client.subscribe_to_topic("esp32/sensors").await.unwrap();

    loop {
        // Publish sensor data
        let payload = b"temperature:23.5";
        client.send_message("esp32/data", payload, rust_mqtt::packet::v5::publish_packet::QualityOfService::QoS1, true).await.unwrap();

        Timer::after_secs(10).await;
    }
}
```

**Skills unlocked:**
- ✅ MQTT protocol
- ✅ Pub/sub patterns
- ✅ IoT messaging
- ✅ QoS levels

---

#### 🔜 Lesson 17: BLE Basics (if available in esp-hal 1.0)
**Duration**: 60 min
**Concepts**: Bluetooth Low Energy, advertising, GATT
**Hardware**: ESP32-C6 (built-in BLE 5.3)

**Note**: BLE support in esp-hal 1.0.0 may be limited. Check documentation.

**What you learn:**
- BLE initialization
- Advertising
- GATT services
- Characteristics

**Status**: To be determined based on esp-hal 1.0.0 BLE support

---

### Phase 6: Complete Systems (Weeks 10-12)

#### 🔜 Lesson 18: Environmental Monitor
**Duration**: 90 min
**Concepts**: Multi-sensor integration, data fusion, cloud reporting
**Hardware**: BME280 (temp/humidity/pressure), display, WiFi

**What you learn:**
- Integrating multiple sensors
- Sensor data processing
- Display updates
- Cloud data upload

**System architecture:**
```
┌─────────────────┐
│  I2C Sensors    │──┐
│  (BME280)       │  │
└─────────────────┘  │
                     ├──> ┌──────────────┐      ┌─────────┐
┌─────────────────┐  │    │  ESP32-C6    │      │  Cloud  │
│  SPI Display    │──┼───>│  Controller  │─WiFi→│  MQTT   │
│  (ST7789)       │  │    │   + State    │      │  Broker │
└─────────────────┘  │    │   Machine    │      └─────────┘
                     │    └──────────────┘
┌─────────────────┐  │
│  Button Input   │──┘
└─────────────────┘
```

**Key patterns:**
- Multiple async sensor tasks
- Shared state for readings
- Display update task
- Network upload task
- Button control task

**Skills unlocked:**
- ✅ Multi-sensor systems
- ✅ Data fusion
- ✅ System architecture
- ✅ Production patterns

---

#### 🔜 Lesson 19: Smart Device with FSM
**Duration**: 90 min
**Concepts**: Complex state machine, event-driven architecture
**Hardware**: Multiple sensors, actuators, display, WiFi

**What you learn:**
- Production-quality state machine
- Event-driven design
- Error recovery
- Persistent state

**State machine:**
```rust
enum DeviceState {
    Initializing,
    Idle { last_reading: SensorData },
    Monitoring { start_time: Instant },
    Alert { alert_type: AlertType, count: u32 },
    NetworkSync { pending_data: Vec<SensorData> },
    Error { error: DeviceError, retry_count: u32 },
    Sleep { wake_time: Instant },
}

enum Event {
    SensorReading(SensorData),
    Threshold Exceeded(f32),
    ButtonPressed,
    NetworkAvailable,
    NetworkLost,
    Error(DeviceError),
    Timer(TimerId),
}
```

**Skills unlocked:**
- ✅ Complex FSM design
- ✅ Event handling
- ✅ Error recovery
- ✅ Production architecture

---

#### 🔜 Lesson 20: Power Management & Optimization
**Duration**: 60 min
**Concepts**: Low-power modes, wake sources, power profiling
**Hardware**: ESP32-C6 with current meter

**What you learn:**
- Deep sleep modes
- Wake sources (timer, GPIO, etc.)
- Power consumption optimization
- Battery-powered design

**Key code pattern:**
```rust
use esp_hal::rtc_cntl::{Rtc, sleep::{Ext0WakeupSource, TimerWakeupSource, WakeupLevel}};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Read sensor
    let temperature = read_sensor().await;
    info!("Temperature: {:.2}°C", temperature);

    // Send to cloud
    send_to_cloud(temperature).await;

    // Enter deep sleep for 60 seconds
    let mut rtc = Rtc::new(peripherals.LPWR);

    let timer = TimerWakeupSource::new(Duration::from_secs(60));
    let gpio = Ext0WakeupSource::new(peripherals.GPIO9, WakeupLevel::High);

    info!("Entering deep sleep...");
    rtc.sleep_deep(&[&timer, &gpio]);
}
```

**Skills unlocked:**
- ✅ Power modes
- ✅ Wake sources
- ✅ Battery optimization
- ✅ Low-power design

---

## 🎥 Video Series Mapping

Each lesson can be a video, or lessons can be combined:

### Video 1: "Getting Started with ESP32-C6 Rust" (20 min)
- Lessons 01-02: Blinky + Button

### Video 2: "State Machines in Rust" (25 min)
- Lesson 03: LED Modes FSM

### Video 3: "Embassy Async Basics" (30 min)
- Lessons 04-05: Async tasks

### Video 4: "Task Communication" (30 min)
- Lessons 06-07: Channels and Mutexes

### Video 5: "Real-World State Machines" (35 min)
- Lessons 08-09: Traffic light + Thermostat

### Video 6: "I2C Sensors" (40 min)
- Lessons 10-11: Sensor + Driver pattern

### Video 7: "SPI Display Control" (40 min)
- Lesson 12: Display driver

### Video 8: "WiFi & IoT" (45 min)
- Lessons 14-16: WiFi + HTTP + MQTT

### Video 9: "Complete IoT Device" (60 min)
- Lessons 18-19: Environmental monitor + Smart device

### Video 10: "Production Optimization" (30 min)
- Lesson 20: Power management

---

## 📊 Lesson Progression Matrix

| Lesson | Blocking | Async | State | I2C | SPI | WiFi | Complexity |
|--------|----------|-------|-------|-----|-----|------|------------|
| 01 | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ⭐ |
| 02 | ✅ | ❌ | Simple | ❌ | ❌ | ❌ | ⭐ |
| 03 | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | ⭐⭐ |
| 04 | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ⭐⭐ |
| 05 | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ⭐⭐ |
| 06 | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ⭐⭐⭐ |
| 07 | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ⭐⭐⭐ |
| 08 | ❌ | ✅ | ✅ | ❌ | ❌ | ❌ | ⭐⭐⭐ |
| 09 | ❌ | ✅ | ✅ | ❌ | ❌ | ❌ | ⭐⭐⭐⭐ |
| 10 | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ | ⭐⭐⭐ |
| 11 | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ | ⭐⭐⭐⭐ |
| 12 | ❌ | ✅ | ❌ | ❌ | ✅ | ❌ | ⭐⭐⭐⭐ |
| 13 | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ⭐⭐⭐ |
| 14 | ❌ | ✅ | ❌ | ❌ | ❌ | ✅ | ⭐⭐⭐⭐ |
| 15 | ❌ | ✅ | ❌ | ❌ | ❌ | ✅ | ⭐⭐⭐⭐ |
| 16 | ❌ | ✅ | ❌ | ❌ | ❌ | ✅ | ⭐⭐⭐⭐ |
| 18 | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ⭐⭐⭐⭐⭐ |
| 19 | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ⭐⭐⭐⭐⭐ |
| 20 | ❌ | ✅ | ✅ | ✅ | ❌ | ✅ | ⭐⭐⭐⭐ |

---

## 🎯 Learning Paths

### Path 1: Quick Start (2 weeks)
Lessons: 01 → 04 → 08 → 10 → 14

**Goal**: Get productive quickly with async + sensors + WiFi

### Path 2: Deep Fundamentals (4 weeks)
Lessons: 01 → 02 → 03 → 04 → 05 → 06 → 07 → 08 → 09

**Goal**: Master every concept thoroughly

### Path 3: IoT Focus (3 weeks)
Lessons: 01 → 04 → 10 → 14 → 15 → 16 → 18

**Goal**: Build cloud-connected devices

### Path 4: Driver Development (3 weeks)
Lessons: 01 → 04 → 10 → 11 → 12 → 13

**Goal**: Create reusable peripheral drivers

---

## 🛠️ Hardware Requirements by Phase

### Phase 1 (Lessons 1-3)
- ESP32-C6 DevKit
- Onboard LED
- 1 external button

### Phase 2 (Lessons 4-9)
- ESP32-C6 DevKit
- 3 LEDs (Red, Yellow, Green)
- 2 buttons
- Resistors (220Ω for LEDs, 10kΩ for buttons)

### Phase 3 (Lessons 10-13)
- ESP32-C6 DevKit
- I2C sensor (BME280 recommended)
- SPI display (ST7789 or similar)
- UART module (optional)
- Jumper wires

### Phase 4 (Lessons 14-17)
- ESP32-C6 DevKit
- WiFi network
- MQTT broker access (can use public broker)

### Phase 5 (Lessons 18-20)
- All above components
- Power supply/battery
- Current meter (for power profiling)

---

## 📝 Next Steps

1. **Review this plan** - Does it match your vision?
2. **Adjust ordering** - Any lessons to move/add/remove?
3. **Hardware check** - Do you have the peripherals?
4. **Create Lesson 02** - Start with button input?
5. **Update SUMMARY.md** - Reference this plan

---

## 🤔 Open Questions

1. **BLE Support**: Is BLE available in esp-hal 1.0.0 yet? (Lesson 17)
2. **Display Library**: Which display library works best with esp-hal 1.0.0?
3. **MQTT Library**: Best MQTT client for no_std async?
4. **Remote Development**: Should we add specific lessons for your RPi setup?
5. **Testing**: Do you want lessons on unit testing embedded code?

---

**This lesson plan balances:**
- ✅ Progressive complexity
- ✅ Practical skills
- ✅ Modern patterns (Embassy + enums)
- ✅ Real-world applications
- ✅ Your remote development workflow

**Ready to start building? 🚀**
