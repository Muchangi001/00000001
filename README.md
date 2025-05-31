# STM32F411CEU6 Black Pill - Embedded Rust LED Sequencer

A comprehensive guide to building and flashing embedded Rust applications to the STM32F411CEU6 Black Pill development board via USB DFU.

## Table of Contents
- [Hardware Overview](#hardware-overview)
- [Project Structure](#project-structure)
- [Understanding the Code](#understanding-the-code)
- [Build Configuration](#build-configuration)
- [Development Workflow](#development-workflow)
- [Flashing Methods](#flashing-methods)
- [Troubleshooting](#troubleshooting)
- [Key Concepts](#key-concepts)

## Hardware Overview

### STM32F411CEU6 Black Pill
- **MCU**: STM32F411CEU6 (ARM Cortex-M4F)
- **Flash**: 512KB
- **RAM**: 128KB
- **Clock**: Up to 100MHz (we use 84MHz for USB compatibility)
- **Built-in LED**: PC13 (inverted logic - LOW = ON)
- **USB**: Native USB 2.0 Full Speed with DFU bootloader

### Pin Configuration
- **LED**: PC13 (onboard LED, active LOW)
- **BOOT0**: Boot mode selection pin
- **RESET**: System reset button
- **USB**: Built-in USB connector for programming and power

## Project Structure

```
embedded_00000001/
├── .cargo/
│   └── config.toml          # Rust build configuration
├── src/
│   └── main.rs             # Main application code
├── build.rs                # Build script (handles memory.x)
├── Cargo.toml              # Project dependencies and settings
├── memory.x                # Memory layout definition
└── README.md               # This file
```

### Key Files Explained

#### `memory.x` - Memory Layout
```
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 128K
}
```
- Defines where code and data go in memory
- STM32F411 has 512KB flash starting at 0x08000000
- 128KB RAM starting at 0x20000000
- Used by the linker to place code sections correctly

#### `.cargo/config.toml` - Build Configuration
```toml
[target.thumbv7em-none-eabihf]
rustflags = ["-C", "link-arg=-Tlink.x"]

[build]
target = "thumbv7em-none-eabihf"

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```
- **Target**: `thumbv7em-none-eabihf` for ARM Cortex-M4F
- **Panic Mode**: `abort` (no stack unwinding in embedded)
- **Link Script**: Uses `link.x` from cortex-m-rt crate

#### `Cargo.toml` - Dependencies
- **cortex-m**: Core ARM Cortex-M functionality
- **cortex-m-rt**: Runtime and startup code
- **panic-halt**: Simple panic handler (halts on panic)
- **stm32f4xx-hal**: Hardware Abstraction Layer for STM32F4

## Understanding the Code

### Application Structure

```rust
#![no_std]     // Don't use standard library
#![no_main]    // No standard main function

use panic_halt as _;           // Import panic handler
use cortex_m_rt::entry;        // Entry point macro
use stm32f4xx_hal::{pac, prelude::*};

#[entry]  // This becomes the real main function
fn main() -> ! {
    // Hardware initialization
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    
    // Clock setup (84MHz for USB compatibility)
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(84.MHz()).freeze();
    
    // Create delay abstraction
    let mut delay = cp.SYST.delay(&clocks);
    
    // Configure LED pin
    let gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output();
    
    // Main application loop
    loop {
        // LED sequences...
    }
}
```

### Key Concepts Explained

#### `#![no_std]` and `#![no_main]`
- **no_std**: Embedded systems don't have operating system features like heap allocation, file system, etc.
- **no_main**: We use a custom entry point instead of standard main()

#### Memory Safety
- **Peripherals::take()**: Ensures only one reference to hardware peripherals exists
- **unwrap()**: Safe because we know peripherals exist at startup

#### Clock Configuration
```rust
let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(84.MHz()).freeze();
```
- **HSE**: High Speed External crystal (25MHz on Black Pill)
- **SYSCLK**: System clock (84MHz for USB compatibility)
- **freeze()**: Locks clock configuration

#### GPIO Configuration
```rust
let gpioc = dp.GPIOC.split();
let mut led = gpioc.pc13.into_push_pull_output();
```
- **split()**: Divides GPIO port into individual pins
- **into_push_pull_output()**: Configures pin as output with push-pull driver

### LED Sequences

The application cycles through four different LED patterns:

1. **Fast Blink**: 3 quick blinks (100ms on/off)
2. **Slow Blink**: 2 slower blinks (500ms on/off)
3. **SOS Pattern**: Morse code SOS (···---···)
4. **Breathing Effect**: Simulated PWM fade in/out

#### SOS Pattern Implementation
```rust
// S - three short blinks
for _ in 0..3 {
    led.set_low();              // LED ON (inverted logic)
    delay.delay_ms(200_u32);
    led.set_high();             // LED OFF
    delay.delay_ms(200_u32);
}
```

#### Breathing Effect (PWM Simulation)
```rust
// Fade in effect
for i in 1..=10 {
    led.set_low();
    delay.delay_ms((i * 10) as u32);    // Increasing ON time
    led.set_high();
    delay.delay_ms((100 - (i * 10)) as u32);  // Decreasing OFF time
}
```

## Build Configuration

### Target Architecture
- **thumbv7em-none-eabihf**: ARM Cortex-M4F with hardware floating point
  - `thumb`: Thumb instruction set (16-bit instructions)
  - `v7e`: ARMv7E-M architecture
  - `m`: Microcontroller profile
  - `none`: No operating system
  - `eabi`: Embedded Application Binary Interface
  - `hf`: Hard float (hardware FPU)

### Panic Handling
```toml
[profile.dev]
panic = "abort"
```
- Embedded systems must use `panic = "abort"`
- No stack unwinding (requires heap allocation)
- System halts on panic for predictable behavior

### Link-Time Optimization (LTO)
```toml
[profile.release]
lto = true
```
- Enables aggressive optimization across crate boundaries
- Reduces code size (important for embedded)
- Longer compile times but better performance

## Development Workflow

### 1. Setup Development Environment

#### Install Rust and Tools
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add ARM target
rustup target add thumbv7em-none-eabihf

# Install binary utilities
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

#### Install DFU Tools (Windows)
1. Download STM32CubeProgrammer from ST website
2. Or download dfu-util from http://dfu-util.sourceforge.net/releases/
3. Add to PATH environment variable

### 2. Build Process

#### Manual Build Commands
```cmd
# Build the project
cargo build --release

# Create ELF file with extension (some tools expect .elf)
copy target\thumbv7em-none-eabihf\release\main target\main.elf

# Convert to binary for DFU flashing
cargo objcopy --release -- -O binary target\thumbv7em-none-eabihf\release\main target\main.bin
```

#### Automated Build Script (`build.bat`)
```batch
@echo off
echo Building project...
cargo build --release

if %errorlevel% neq 0 (
    echo Build failed!
    pause
    exit /b 1
)

echo Creating ELF file with extension...
copy target\thumbv7em-none-eabihf\release\main target\main.elf

echo Converting to binary for DFU...
cargo objcopy --release -- -O binary target\thumbv7em-none-eabihf\release\main target\main.bin

echo Files created:
echo - ELF: target\main.elf  
echo - BIN: target\main.bin
echo Done!
pause
```

### 3. File Types Generated

#### ELF File (`.elf`)
- **Purpose**: Contains executable code, debug symbols, and metadata
- **Use**: Programming with STM32CubeProgrammer, debugging with GDB
- **Size**: Larger (includes debug info and symbols)

#### Binary File (`.bin`)
- **Purpose**: Raw binary data only
- **Use**: DFU flashing, bootloaders
- **Size**: Smaller (just the code and data)

## Flashing Methods

### Method 1: USB DFU (Device Firmware Update)

#### Enter DFU Mode
1. **Hold BOOT0 button** (small button near USB connector)
2. **Press and release RESET button** (while holding BOOT0)
3. **Release BOOT0 button**
4. **Connect USB cable**

#### Verify DFU Mode
```cmd
dfu-util -l
```
Expected output:
```
Found DFU: [0483:df11] ver=2200, devnum=1, cfg=1, intf=0, alt=0, 
name="@Internal Flash  /0x08000000/04*016Kg,01*064Kg,07*128Kg"
```

#### Flash via Command Line
```cmd
dfu-util -a 0 -s 0x08000000:leave -D target\main.bin
```
- `-a 0`: Alternate setting 0 (internal flash)
- `-s 0x08000000:leave`: Start address and leave DFU mode after flashing
- `-D`: Download (flash) the file

#### Flash via STM32CubeProgrammer
1. Open STM32CubeProgrammer
2. Select "USB" connection
3. Connect to device
4. Load `target\main.elf` file
5. Click "Download"

### Method 2: ST-Link (if available)
```cmd
# Using OpenOCD
openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c "program target\main.elf verify reset exit"

# Using STM32CubeProgrammer CLI
STM32_Programmer_CLI.exe -c port=SWD -w target\main.elf -v -s
```

## Troubleshooting

### Common Build Errors

#### Error: "unwinding panics are not supported"
**Solution**: Add `panic = "abort"` to all profiles in `Cargo.toml`

#### Error: "linking with `link.exe` failed"
**Problem**: Using Windows linker instead of ARM linker
**Solution**: Create proper `.cargo/config.toml` with target configuration

#### Error: "unknown argument '-Wl,--gc-sections'"
**Problem**: Wrong linker flags for rust-lld
**Solution**: Use `--gc-sections` instead of `-Wl,--gc-sections`

### Common Flashing Issues

#### "No DFU capable USB device available"
**Causes**:
- Not in DFU mode (repeat DFU entry procedure)
- Driver issues (install STM32 USB drivers)
- Bad USB cable (use data cable, not charge-only)

#### "Cannot open DFU device"
**Solutions**:
- Run command prompt as Administrator
- Check Windows Device Manager for driver issues
- Try different USB port

#### "Device not found" in STM32CubeProgrammer
**Solutions**:
- Refresh the connection
- Check USB connection
- Verify DFU mode entry

### Debug Techniques

#### Check Generated Files
```cmd
# Verify ELF file sections
cargo objdump --release -- --section-headers target\thumbv7em-none-eabihf\release\main

# Check binary size
dir target\main.bin

# Verify memory layout
cargo nm --release target\thumbv7em-none-eabihf\release\main | findstr "Reset"
```

#### LED Not Working
- Check pin configuration (PC13 is inverted logic)
- Verify clock setup
- Ensure proper GPIO initialization
- Check for panic conditions

## Key Concepts

### Embedded Systems Constraints
- **No Standard Library**: No heap, file system, or OS services
- **Real-time**: Deterministic timing requirements
- **Resource Limited**: Limited RAM and flash memory
- **Power Conscious**: Battery-powered applications

### ARM Cortex-M4 Features
- **32-bit RISC**: Reduced Instruction Set Computer
- **Thumb Instructions**: 16-bit instructions for code density
- **Hardware FPU**: Single-precision floating-point unit
- **NVIC**: Nested Vectored Interrupt Controller
- **SysTick**: System tick timer for delays

### Memory Layout
```
Flash (512KB): 0x08000000 - 0x0807FFFF
├── Vector Table (1KB)
├── Application Code
└── Constants/Data

RAM (128KB): 0x20000000 - 0x2001FFFF
├── Stack (grows down)
├── Heap (if used)
└── Global Variables
```

### GPIO Concepts
- **Push-Pull Output**: Can source or sink current
- **Open-Drain**: Can only sink current (needs pull-up)
- **Input Modes**: Floating, pull-up, pull-down
- **Inverted Logic**: LED on Black Pill is active LOW

### Clock System
```
HSE (25MHz) → PLL → SYSCLK (84MHz)
├── AHB Bus (84MHz)
├── APB1 Bus (42MHz)
└── APB2 Bus (84MHz)
```

### Development Best Practices
1. **Start Simple**: Basic LED blink before complex features
2. **Use HAL**: Hardware Abstraction Layer for portability
3. **Error Handling**: Use `Result` types and proper error handling
4. **Resource Management**: Use RAII and ownership for safety
5. **Testing**: Test on hardware early and often

### Learning Path
1. **Basic GPIO**: LED control, button input
2. **Timers**: PWM, precise timing
3. **Communication**: UART, SPI, I2C
4. **Interrupts**: External events, timer interrupts
5. **Advanced**: DMA, ADC, USB, wireless

## Additional Resources

### Documentation
- [STM32F411 Reference Manual](https://www.st.com/resource/en/reference_manual/dm00119316.pdf)
- [Cortex-M4 Programming Manual](https://developer.arm.com/documentation/dui0553/latest)
- [Embedded Rust Book](https://docs.rust-embedded.org/book/)

### Tools
- [STM32CubeMX](https://www.st.com/en/development-tools/stm32cubemx.html) - Pin configuration
- [STM32CubeProgrammer](https://www.st.com/en/development-tools/stm32cubeprog.html) - Flashing tool
- [OpenOCD](http://openocd.org/) - Open source debugger

### Community
- [Rust Embedded Working Group](https://github.com/rust-embedded/wg)
- [STM32 Rust Community](https://github.com/stm32-rs)
- [Embedded Rust Matrix Chat](https://matrix.to/#/#rust-embedded:matrix.org)

---

**Happy Embedded Programming!** 🦀⚡

This project demonstrates the power of Rust for embedded systems development, combining memory safety with zero-cost abstractions for reliable, efficient firmware.