# VTU Project - STM32 Embedded Systems Development Report

**Date:** December 3, 2025  
**Developer:** Arshi Sabah  
**Platform:** STM32 Microcontrollers (STM32F103, STM32G491)  
**Programming Language:** Rust (Embedded)  
**Development Environment:** VS Code with probe-rs

---

## Table of Contents
1. [Project Overview](#project-overview)
2. [Project 1: stm32_ttl_example](#project-1-stm32_ttl_example)
3. [Project 2: task2 - Multi-MCU UART Communication](#project-2-task2---multi-mcu-uart-communication)
4. [Project 3: task3 - M66 GSM Module Integration](#project-3-task3---m66-gsm-module-integration)
5. [Project 4: task4 - MathCal Serial Calculator](#project-4-task4---mathcal-serial-calculator)
6. [Project 5: task5 - GPIO Control with Button and LED](#project-5-task5---gpio-control-with-button-and-led)
7. [Technologies & Protocols Used](#technologies--protocols-used)
8. [Hardware Setup](#hardware-setup)
9. [Build & Flash Instructions](#build--flash-instructions)
10. [Testing & Validation](#testing--validation)
11. [Conclusion](#conclusion)

---

## Project Overview

This repository contains five embedded Rust projects demonstrating various communication protocols and peripheral usage on STM32 microcontrollers. All projects showcase:
- Bare-metal embedded Rust programming
- UART serial communication
- GPIO digital input/output
- Real-time embedded systems design
- Hardware abstraction layer (HAL) usage
- Cross-compilation for ARM Cortex-M processors

**Key Technologies:**
- **Language:** Rust (no_std, embedded)
- **Microcontrollers:** STM32F103C8T6 (Blue Pill), STM32F103RB (Nucleo), STM32G491
- **Protocols:** UART/USART, GPIO
- **Tools:** cargo, probe-rs, ST-Link V2

---

## Project 1: stm32_ttl_example

### Description
Simple "ping-pong" communication demonstrating basic UART functionality with LED feedback.

### Technical Specifications
- **Microcontroller:** STM32F103C8T6
- **Clock Speed:** 72 MHz
- **Protocol:** UART (USART2)
- **Baud Rate:** 115,200 bps
- **Pins Used:**
  - PA2 (TX) - Transmit to ST-Link VCP
  - PA3 (RX) - Receive from ST-Link VCP
  - PC13 - LED indicator

### Functionality
| Input | Output | LED Status |
|-------|--------|-----------|
| `hi` | `hello` | ON (Low) |
| Any other | `Invalid message` | OFF (High) |

### Key Features
- Real-time message parsing
- Ring buffer for incoming data (20 bytes)
- LED visual feedback
- Newline/carriage return detection
- Buffer overflow protection

### Dependencies
```toml
cortex-m = "0.7"
cortex-m-rt = "0.7"
stm32f1xx-hal = "0.10"
panic-halt = "0.2"
```

### Code Highlights
- Uses ST-Link Virtual COM Port (no external TTL module needed)
- Message validation with exact string matching
- Startup greeting message sent on boot

---

## Project 2: task2 - Multi-MCU UART Communication

### Description
Dual-microcontroller project showcasing UART communication on different STM32 families.

### 2.1 STM32F103 Implementation

**Technical Specifications:**
- **Microcontroller:** STM32F103C8T6
- **Clock:** 72 MHz (HSE: 8 MHz)
- **Protocol:** UART (USART1)
- **Baud Rate:** 9600 bps
- **Memory:** 64KB Flash, 20KB RAM

**Pins:**
- PA9 (TX)
- PA10 (RX)

**Features:**
- External crystal oscillator usage (HSE)
- Clock tree configuration (SYSCLK: 72MHz, PCLK1: 36MHz)
- `heapless::String` for dynamic string handling
- Message echoing functionality

### 2.2 STM32G491 Implementation

**Technical Specifications:**
- **Microcontroller:** STM32G491
- **Architecture:** ARM Cortex-M4F with FPU
- **Protocol:** UART (USART2)
- **Baud Rate:** 115,200 bps
- **Target:** thumbv7em-none-eabihf

**Pins:**
- PA2 (TX) - AF7 (Alternate Function 7)
- PA3 (RX) - AF7

**Features:**
- Direct register manipulation (no HAL)
- Manual clock tree configuration (RCC)
- GPIO alternate function setup
- Low-level peripheral control
- Demonstrates advanced embedded programming

**Key Differences from STM32F103:**
- Modern STM32G4 series architecture
- Hardware floating-point unit
- Different register structure
- More advanced peripherals

### Purpose
Demonstrates portability of embedded Rust across different STM32 families and programming approaches (HAL vs. register-level).

---

## Project 3: task3 - M66 GSM Module Integration

### Description
Integration of Quectel M66 GSM module for wireless communication with AT command interface.

### Technical Specifications
- **Microcontroller:** STM32F103C8T6
- **Clock Speed:** 72 MHz
- **Dual UART Configuration:**
  - **USART1:** M66 GSM module communication
  - **USART2:** Debug output (ST-Link VCP)
- **Baud Rate:** 115,200 bps (both UARTs)

### Hardware Setup
```
M66 GSM Module ↔ STM32F103 (USART1: PA9/PA10)
Debug Output   ↔ STM32F103 (USART2: PA2/PA3) ↔ PC via ST-Link
```

### Functionality
| M66 Receives | M66 Responds | LED | Debug Output |
|-------------|--------------|-----|--------------|
| `hii` | `hello` | ON | "Received 'hii', sent 'hello'" |
| Other text | `invalid msg` | OFF | "Invalid message received" |

### Key Features
- **Dual UART operation** for simultaneous GSM and debug communication
- **AT Command initialization:**
  - `AT` - Module health check
  - `ATE0` - Disable command echo
- **3-second boot delay** for M66 module startup
- **128-byte receive buffer** for GSM messages
- **Substring matching** for flexible message detection
- **Real-time debug monitoring** via second UART
- **LED status indicator** (PC13)

### AT Command Interface
The code initializes the M66 with standard AT commands:
```rust
"AT\r\n"    // Test connection
"ATE0\r\n"  // Disable echo
```

### Use Cases
- SMS message processing
- Remote device control via GSM
- IoT communication over cellular network
- M2M (Machine-to-Machine) applications

---

## Project 4: task4 - MathCal Serial Calculator

### Description
Interactive serial calculator performing arithmetic operations via UART commands.

### Technical Specifications
- **Microcontroller:** STM32F103C8T6
- **Protocol:** UART (USART2)
- **Baud Rate:** 115,200 bps
- **Pins:** PA2 (TX), PA3 (RX)
- **Connection:** ST-Link Virtual COM Port

### Supported Operations
| Operation | Command Format | Example | Result |
|-----------|---------------|---------|--------|
| Addition | `add <n1> <n2>` | `add 5 3` | `8` |
| Subtraction | `sub <n1> <n2>` | `sub 10 4` | `6` |
| Multiplication | `mul <n1> <n2>` | `mul 7 8` | `56` |
| Division | `div <n1> <n2>` | `div 20 5` | `4` |

### Input Format
```
<operation> <number1> <number2>
```
- **Operations:** add, sub, mul, div
- **Numbers:** Signed 32-bit integers (i32)
- **Delimiter:** Space-separated
- **Line ending:** Newline or carriage return

### Error Handling
- **Division by zero:** Returns `"Error: Div by 0"`
- **Invalid input:** Returns `"Invalid input"`
- **Buffer overflow:** Auto-clears and resets (32-byte limit)

### Key Features
- **Real-time parsing** of incoming serial commands
- **heapless::String<32>** for no-allocation string formatting
- **Whitespace-tolerant** input parsing
- **Startup instructions** displayed on connection:
  ```
  STM32F103 Math Serial
  Send: <op> <num1> <num2>
  Ops: add, sub, mul, div
  ```

### Code Architecture
```rust
fn process_message(msg: &str) -> heapless::String<32>
```
- Splits input into operation and operands
- Pattern matching for operation selection
- Safe integer parsing with error handling
- Returns formatted result string

### Build Optimization
- **Release profile:** `opt-level = "z"` (size optimization)
- **LTO enabled:** Link-time optimization
- **Binary size:** ~5KB (much smaller than debug build)
- **Flash time:** <1 second

---

## Project 5: task5 - GPIO Control with Button and LED

### Description
Simple GPIO input/output demonstration using onboard button and LED on STM32 Nucleo board. Press button to control LED state - the foundation of embedded digital I/O.

### Technical Specifications
- **Microcontroller:** STM32F103RB (Nucleo-64 board)
- **Clock Speed:** 72 MHz
- **Protocol:** None (Direct GPIO)
- **Pins Used:**
  - PC13 - User button (B1) input
  - PA5 - User LED (LD2) output

### Hardware Features (Nucleo Board)
- **Onboard User Button (B1):** Blue tactile button on PC13
- **Onboard User LED (LD2):** Green LED on PA5
- **No external components required**

### Functionality
| Button State | PC13 Reading | LED (PA5) | Description |
|-------------|--------------|-----------|-------------|
| Not pressed | HIGH | OFF | Default state |
| Pressed | LOW | ON | Button connects to GND |

### Key Features
- **Pull-up input configuration** on PC13
- **Push-pull output** on PA5
- **Real-time button polling** in main loop
- **Software debouncing** with delay
- **No external wiring needed** (uses onboard components)

### GPIO Configuration

#### Input (Button):
```rust
let button = gpioc.pc13.into_pull_up_input(&mut gpioc.crh);
```
- Internal pull-up resistor enabled
- Button pressed = LOW (0V)
- Button released = HIGH (3.3V)

#### Output (LED):
```rust
let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
```
- Push-pull output mode
- set_high() = LED ON
- set_low() = LED OFF

### Code Logic
```rust
loop {
    if button.is_low() {        // Check button state
        led.set_high();         // Turn LED ON
    } else {
        led.set_low();          // Turn LED OFF
    }
    cortex_m::asm::delay(10_000);  // Debounce delay
}
```

### Dependencies
```toml
cortex-m = "0.7"
cortex-m-rt = "0.7"
stm32f1xx-hal = { version = "0.10", features = ["stm32f103", "rt", "medium"] }
panic-halt = "0.2"
```

### Use Cases
- Learning basic GPIO operations
- Understanding pull-up/pull-down resistors
- Button debouncing techniques
- Foundation for user interfaces
- Testing embedded systems basics

### Alternative Configurations

#### For Blue Pill (STM32F103C8T6):
```rust
// External button on PA0
let button = gpioa.pa0.into_pull_up_input(&mut gpioa.crl);

// Onboard LED on PC13 (active-low)
let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
```

#### External Components (Optional):
```
PA0 ──── Switch ──── GND         (Input)
PA5 ──── 330Ω ──── LED ──── GND  (Output)
```

### Build & Flash
```bash
cd d:/Vtu-Project/task5
cargo run --release
```

### Binary Size
- Debug build: ~2KB
- Release build: ~1KB (minimal GPIO code)

---

## Technologies & Protocols Used

### Programming Language
**Rust (Embedded)**
- `#![no_std]` - No standard library (bare-metal)
- `#![no_main]` - Custom entry point
- Memory-safe systems programming
- Zero-cost abstractions

### Communication Protocols

#### GPIO (General Purpose Input/Output)
**Used in Task 5**
- **Type:** Digital input/output
- **Modes:** 
  - Input: Pull-up, Pull-down, Floating
  - Output: Push-pull, Open-drain
- **Use cases:** Buttons, LEDs, switches, digital sensors
- **Advantages:**
  - Direct hardware control
  - No protocol overhead
  - Real-time response
  - Simple to implement

#### UART/USART (Universal Asynchronous Receiver-Transmitter)
**Used in Projects 1-4**
- **Type:** Asynchronous, full-duplex
- **Wiring:** 2-wire (TX, RX) + GND
- **Baud rates:** 9,600 - 115,200 bps
- **Configuration:** 8N1 (8 data bits, no parity, 1 stop bit)
- **Advantages:**
  - Simple to implement
  - Low pin count
  - Universal support
  - Good for medium-speed communication

### Hardware Abstraction Layers (HAL)

#### stm32f1xx-hal (v0.10.0)
- High-level peripheral abstractions
- Type-safe GPIO configuration
- Clock tree management
- Serial port drivers

#### Direct Register Access (STM32G491)
- Manual RCC configuration
- GPIO MODER/OTYPER/OSPEEDR control
- USART BRR calculation
- Lower-level control, more complexity

### Embedded Crates

| Crate | Version | Purpose |
|-------|---------|---------|
| `cortex-m` | 0.7 | Cortex-M processor support |
| `cortex-m-rt` | 0.7 | Runtime & startup code |
| `embedded-hal` | 0.2 | Hardware abstraction traits |
| `heapless` | 0.8 | No-allocation data structures |
| `nb` | 1.1 | Non-blocking operations |
| `panic-halt` | 0.2 | Panic handler (halt on panic) |

---

## Hardware Setup

### Common Components
- **Microcontroller Board:** STM32F103C8T6 "Blue Pill"
- **Debugger/Programmer:** ST-Link V2 (genuine or clone)
- **Connection:** USB cable for ST-Link
- **Power:** 3.3V from ST-Link or external supply

### Pin Configurations

#### Project 1 & 4 (stm32_ttl_example, MathCal)
```
STM32F103          ST-Link VCP
PA2 (TX)    ↔     Virtual COM Port
PA3 (RX)    ↔     Virtual COM Port
PC13        →     Onboard LED
GND         ↔     GND
```

#### Project 2 (task2/stm32f103)
```
STM32F103          TTL Module
PA9 (TX)    ↔     RX
PA10 (RX)   ↔     TX
GND         ↔     GND
```

#### Project 3 (task3 - GSM)
```
STM32F103          M66 GSM Module
PA9 (TX)    ↔     RX
PA10 (RX)   ↔     TX
GND         ↔     GND
3.3V/5V     →     VCC

STM32F103          ST-Link VCP (Debug)
PA2 (TX)    ↔     Virtual COM Port
PA3 (RX)    ↔     Virtual COM Port
```

#### Project 5 (task5 - GPIO)
**Nucleo-F103RB Board:**
```
Onboard Components (No external wiring needed)
PC13    →    User Button (B1) - Blue button
PA5     →    User LED (LD2) - Green LED
```

**Blue Pill Board (Alternative):**
```
STM32F103          External Components
PA0       ↔       Switch ↔ GND
PA5       →       330Ω → LED+ → LED- → GND
PC13      →       Onboard LED
```

### Wiring Diagrams

**UART Connection (TX/RX):**
```
Device A TX  →  Device B RX
Device A RX  ←  Device B TX
GND         ↔  GND
```

**ST-Link Connection:**
```
ST-Link          STM32F103
SWDIO    ↔      SWDIO
SWCLK    ↔      SWCLK
GND      ↔      GND
3.3V     →      3.3V (optional)
```

---

## Build & Flash Instructions

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add ARM target
rustup target add thumbv7m-none-eabi
rustup target add thumbv7em-none-eabihf  # For STM32G491

# Install probe-rs
cargo install probe-rs --features cli
```

### Building Projects

#### Development Build (with debug info)
```bash
cd d:/Vtu-Project/<project-name>
cargo build
```

#### Release Build (optimized, smaller binary)
```bash
cargo build --release
```

### Flashing to STM32

#### Method 1: Build + Flash + Run (using runner)
```bash
cargo run --release
```

#### Method 2: Flash only (using probe-rs directly)
```bash
cargo flash --chip STM32F103C8 --release
```

#### Method 3: Build then flash separately
```bash
cargo build --release
probe-rs run --chip STM32F103C8 target/thumbv7m-none-eabi/release/<binary-name>
```

### Build Outputs

| Build Type | Size | Flash Time | Use Case |
|-----------|------|------------|----------|
| Debug | ~15KB | 2-3s | Development, debugging |
| Release | ~5KB | <1s | Production, deployment |

### Common Issues & Solutions

#### Issue: "No loadable segments found"
**Solution:** Use `-Tlink.x` in rustflags, not `-Tmemory.x`

#### Issue: "JtagDmaError" after flashing
**Solution:** This is normal - firmware is already running. Ignore the error.

#### Issue: Slow flashing speed
**Solution:** Use release builds (`cargo run --release`)

---

## Testing & Validation

### Test Equipment
- Serial terminal software:
  - Arduino IDE Serial Monitor
  - PuTTY
  - TeraTerm
  - screen (Linux/Mac)
- Logic analyzer (optional, for debugging)
- Multimeter (voltage verification)

### Testing Procedures

#### Project 1: stm32_ttl_example
1. Flash firmware to STM32
2. Open ST-Link VCP in Device Manager (Windows)
3. Connect serial terminal at 115,200 baud
4. Send `hi` → Expect `hello` + LED ON
5. Send `test` → Expect `Invalid message` + LED OFF

**Expected Serial Output:**
```
STM32F103 Ready - Send 'hi' to test
```

#### Project 4: MathCal
1. Flash firmware
2. Connect serial terminal at 115,200 baud
3. Test each operation:

**Test Cases:**
| Test | Input | Expected Output |
|------|-------|----------------|
| Addition | `add 100 50` | `150` |
| Subtraction | `sub 100 50` | `50` |
| Multiplication | `mul 12 8` | `96` |
| Division | `div 100 5` | `20` |
| Div by zero | `div 10 0` | `Error: Div by 0` |
| Invalid | `xyz 1 2` | `Invalid input` |
| Negative | `add -5 3` | `-2` |

**Startup Message:**
```
STM32F103 Math Serial
Send: <op> <num1> <num2>
Ops: add, sub, mul, div
```

#### Project 3: M66 GSM
1. Connect M66 module to STM32
2. Power on M66 (wait 3s for boot)
3. Flash firmware
4. Open debug serial terminal (115,200 baud)
5. Send data to M66 via SMS or serial

**Debug Output:**
```
STM32F103 M66 GSM Module Ready
Initializing M66...
M66 Initialized. Send 'hii' via SMS or data connection
```

#### Project 5: GPIO Control
**For Nucleo Board:**
1. Flash firmware to STM32F103RB Nucleo
2. No external wiring needed
3. Press blue user button (B1)
   → Green LED (LD2) turns ON
4. Release button
   → LED turns OFF

**For Blue Pill:**
1. Flash firmware
2. Connect jumper wire from A0 to GND
   → LED turns ON
3. Remove wire
   → LED turns OFF

**Test Cases:**
| Action | Expected Result |
|--------|----------------|
| Button pressed | LED ON |
| Button released | LED OFF |
| Continuous press | LED stays ON |
| Rapid press/release | LED blinks (with debounce) |

### Validation Criteria
✅ Successful compilation without errors  
✅ Firmware flashes to target device  
✅ Serial communication established  
✅ Correct responses to all test inputs  
✅ Error handling works as expected  
✅ LED indicators function properly  
✅ No buffer overflows or crashes  
✅ Stable operation over extended periods  

---

## Conclusion

### Project Summary
This repository demonstrates comprehensive embedded systems development using Rust on STM32 microcontrollers:

1. **stm32_ttl_example** - Foundation for UART communication
2. **task2** - Multi-platform compatibility (F103 & G491)
3. **task3** - External module integration (M66 GSM)
4. **task4** - Real-world application (serial calculator)
5. **task5** - GPIO fundamentals (button input, LED output)

### Key Achievements
- ✅ **Memory-safe embedded programming** with Rust
- ✅ **Zero-cost abstractions** maintaining performance
- ✅ **Cross-platform compatibility** (STM32F1, STM32G4)
- ✅ **Protocol implementation** (UART, AT commands, GPIO)
- ✅ **Real-time data processing** with constrained resources
- ✅ **Production-ready** error handling and validation
- ✅ **Direct hardware control** via GPIO

### Technical Skills Demonstrated
- Bare-metal embedded Rust programming
- UART/USART protocol implementation
- GPIO digital input/output control
- Hardware abstraction layer usage
- Direct register manipulation
- Memory management without allocation
- Real-time embedded systems design
- Cross-compilation for ARM Cortex-M
- Debugging with probe-rs and ST-Link
- Button debouncing techniques

### Future Enhancements
- Add SPI communication examples
- Implement I2C sensor integration
- Add CAN bus support
- Integrate RTOS (Real-Time Operating System)
- Add unit testing framework
- Implement DMA for efficient data transfer
- Add low-power mode demonstrations
- Create web interface via Wi-Fi module

### Learning Outcomes
This project collection provides:
- Practical embedded Rust experience
- Understanding of communication protocols
- Hardware-software integration skills
- Real-world problem-solving approaches
- Production-quality code practices

---

**Repository:** github.com/arshisabah/vtu  
**Branch:** master  
**Maintained by:** Arshi Sabah  
**Last Updated:** December 3, 2025

---

## Appendices

### Appendix A: Memory Layout
```
STM32F103C8T6:
Flash: 0x08000000 - 0x08010000 (64KB)
RAM:   0x20000000 - 0x20005000 (20KB)
```

### Appendix B: Clock Configurations
```
STM32F103:
- HSE: 8 MHz (external crystal)
- SYSCLK: 72 MHz (via PLL)
- PCLK1: 36 MHz (APB1)
- PCLK2: 72 MHz (APB2)
```

### Appendix C: Serial Terminal Settings
```
Baud Rate: 115,200 (most projects) or 9,600 (task2/stm32f103)
Data Bits: 8
Parity: None
Stop Bits: 1
Flow Control: None
```

---

**End of Report**
