# Rust Embedded RTOS - Real-Time Operating Systems for Microcontrollers

*Comprehensive guide to Real-Time Operating Systems (RTOS) written in Rust for embedded controllers, including Arduino (AVR), Raspberry Pi, ARM Cortex-M, and ESP32 platforms.*

---

## 🎯 **Core Concept**

**Embedded Rust RTOS** refers to operating systems and runtime frameworks written in Rust that provide real-time task scheduling, hardware abstraction, and memory safety guarantees for resource-constrained embedded systems. Unlike traditional C/C++ RTOS solutions, Rust RTOS projects leverage the type system and ownership model to prevent common embedded bugs (use-after-free, data races, null pointer dereferences) at compile time.

**Key Characteristics**:

- **Memory safety without garbage collection**: Zero-cost abstractions with compile-time guarantees
- **Concurrency without data races**: Ownership prevents simultaneous mutable access
- **No runtime overhead**: Most safety checks happen at compile time
- **Interoperability**: Can interface with existing C/C++ embedded ecosystems

**Why Embedded Rust Matters**:

1. **Safety-critical systems**: Medical devices, automotive, aerospace require memory safety
2. **Reducing bugs**: 70% of security vulnerabilities stem from memory unsafety (Microsoft research)
3. **Modern abstractions**: Async/await, iterators, pattern matching in embedded context
4. **Growing adoption**: Google (OpenTitan), Espressif (ESP32), Amazon (FreeRTOS-rust)

---

## 🧠 **Mental Models**

### **The Safety Spectrum**

```
Bare Metal C/Assembly          Rust RTOS              Linux on SBC
    ↓                            ↓                      ↓
Maximum control            Safety + Performance    Maximum abstraction
No safety                  Compile-time checks     Runtime overhead
Minimal overhead           Zero-cost abstractions  Full OS features
Manual memory              Ownership system        Garbage collection/malloc
```

### **RTOS Architecture Comparison**

```
Traditional RTOS (FreeRTOS):
┌─────────────────────────┐
│ Application (C/C++)     │ ← Bugs possible: use-after-free, races
├─────────────────────────┤
│ RTOS Kernel             │ ← Runtime task scheduler
├─────────────────────────┤
│ Hardware (bare metal)   │
└─────────────────────────┘

Rust RTOS (Tock/Embassy):
┌─────────────────────────┐
│ Application (Rust)      │ ← Compile-time safety guarantees
├─────────────────────────┤
│ RTOS Runtime/Executor   │ ← Zero-cost or minimal overhead
├─────────────────────────┤
│ Hardware (bare metal)   │
└─────────────────────────┘
Ownership prevents common bugs before runtime!
```

### **Embedded Platform Hierarchy**

```
Complexity:    Low ←────────────────────→ High
Resources:     Minimal ←─────────────────→ Abundant

Arduino (AVR)      ARM Cortex-M       ESP32         Raspberry Pi
2KB RAM            32-512KB RAM       520KB RAM     1-8GB RAM
16 MHz             48-168 MHz         240 MHz       1.5 GHz
No OS              RTOS               RTOS/Linux    Full Linux
avr-hal            Embassy/RTIC/Tock  Embassy       std Rust + rppal
```

---

## 🔍 **Detailed Content**

### **Production-Ready Rust RTOS Projects**

#### **1. Tock OS** - Security-Focused Microkernel

**Architecture**: Microkernel with memory protection

- **Website**: <https://www.tockos.org/>
- **GitHub**: <https://github.com/tock/tock>
- **Target Platforms**: ARM Cortex-M (nRF52, SAM4L, STM32, etc.)

**Key Features**:

```rust
// Process isolation with Memory Protection Unit (MPU)
// Each app runs in its own memory space
struct Process {
    memory_region: &'static mut [u8],
    syscall_interface: SyscallDriver,
}

// Capsules - kernel modules with capability-based security
trait Capsule {
    fn command(&self, cmd: usize, arg: usize) -> Result<(), ErrorCode>;
}
```

**Production Use Cases**:

- **Google OpenTitan**: Security chips for server firmware protection
- **Research projects**: Universities use for OS education
- **IoT security**: Sandboxed untrusted code execution

**Mental Model**: Think of Tock as "micro-Linux" - processes, syscalls, but for microcontrollers with MPU enforcement.

---

#### **2. Embassy** - Modern Async RTOS

**Architecture**: Async executor with cooperative multitasking

- **Website**: <https://embassy.dev/>
- **GitHub**: <https://github.com/embassy-rs/embassy>
- **Target Platforms**: ARM Cortex-M, RISC-V, ESP32, RP2040

**Key Features**:

```rust
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

// Async task - compiles to efficient state machine
#[embassy_executor::task]
async fn blink_led(pin: AnyPin) {
    let mut led = Output::new(pin, Level::Low);
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}

// Spawn multiple concurrent tasks
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    spawner.spawn(blink_led(p.PIN_13)).unwrap();
    spawner.spawn(read_sensor(p.ADC1)).unwrap();
    spawner.spawn(handle_uart(p.USART1)).unwrap();
}
```

**Production Use Cases**:

- **Battery-powered IoT**: Power-efficient async I/O reduces current draw
- **ESP32 projects**: Official Embassy support from Espressif
- **Industrial sensors**: Multi-sensor coordination with async

**Mental Model**: Embassy is "Tokio for embedded" - async/await without heap allocation, perfect for concurrent I/O-bound tasks.

---

#### **3. RTIC (Real-Time Interrupt-driven Concurrency)** - Zero-Cost Hard Real-Time

**Architecture**: Compile-time task scheduling with hardware interrupts

- **Website**: <https://rtic.rs/>
- **GitHub**: <https://github.com/rtic-rs/rtic>
- **Target Platforms**: ARM Cortex-M only

**Key Features**:

```rust
#[rtic::app(device = stm32f4::stm32f401, peripherals = true)]
mod app {
    #[shared]
    struct Shared {
        counter: u32,
    }
    
    #[local]
    struct Local {}
    
    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        // Hardware initialization
        (Shared { counter: 0 }, Local {})
    }
    
    // Task with priority (higher number = higher priority)
    #[task(binds = TIM2, priority = 2, shared = [counter])]
    fn timer_interrupt(mut cx: timer_interrupt::Context) {
        cx.shared.counter.lock(|c| *c += 1);
    }
    
    // Lower priority task
    #[task(priority = 1, shared = [counter])]
    async fn process_data(mut cx: process_data::Context) {
        let count = cx.shared.counter.lock(|c| *c);
        // Process...
    }
}
```

**Production Use Cases**:

- **Motor control**: Deterministic timing for PWM and encoder reading
- **Industrial automation**: PLC-like control with hard deadlines
- **Safety-critical**: Automotive, medical devices requiring WCET guarantees

**Mental Model**: RTIC is "interrupt controller + Rust type system" - hardware preemption with compile-time resource allocation.

---

#### **4. FreeRTOS-rust** - Rust Bindings to Battle-Tested C Kernel

**Architecture**: Safe Rust wrapper over FreeRTOS C API

- **GitHub**: <https://github.com/lobaro/FreeRTOS-rust>
- **Target Platforms**: Any platform FreeRTOS supports (50+ architectures)

**Key Features**:

```rust
use freertos_rust::*;

fn main() {
    // Create FreeRTOS task with Rust closure
    Task::new()
        .name("blink")
        .stack_size(1024)
        .priority(TaskPriority(1))
        .start(|| {
            loop {
                toggle_led();
                CurrentTask::delay(Duration::ms(500));
            }
        })
        .unwrap();
    
    // Start FreeRTOS scheduler
    FreeRtosUtils::start_scheduler();
}
```

**Production Use Cases**:

- **Legacy migration**: Gradual Rust adoption in existing FreeRTOS projects
- **Certification**: FreeRTOS has DO-178C/IEC-61508 certified versions
- **Multi-platform**: Widest hardware support

**Mental Model**: "Best of both worlds" - FreeRTOS's proven kernel with Rust's safety on top.

---

#### **5. Drone OS** - Research-Focused Fiber Scheduler

**Architecture**: Fiber-based cooperative multitasking with compile-time memory safety

- **Website**: <https://www.drone-os.com/>
- **GitHub**: <https://github.com/drone-os/drone>
- **Target Platforms**: ARM Cortex-M

**Key Features**:

- Dynamic memory with compile-time guarantees (heap pools)
- Fiber-based lightweight threads
- Interrupt-driven I/O with Rust futures

**Status**: Less mature than Tock/Embassy, more experimental

---

### **Embedded Platform-Specific Implementations**

#### **Arduino (AVR Architecture)**

**Main Framework**: AVR-HAL

- **GitHub**: <https://github.com/Rahix/avr-hal>
- **Supported Boards**: Uno (ATmega328P), Mega 2560, Leonardo, Nano

**Complete Blink Example**:

```rust
#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _; // Panic handler

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    // Configure pin D13 (built-in LED) as output
    let mut led = pins.d13.into_output();
    
    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
```

**Development Workflow**:

```powershell
# Install AVR toolchain and programmer
cargo install ravedude

# Create new project
cargo new --bin arduino-blink
cd arduino-blink

# Add dependencies to Cargo.toml
# [dependencies]
# arduino-hal = "0.1"
# panic-halt = "0.2"

# Build and flash to Arduino Uno
cargo build --release
ravedude uno -cb 57600
```

**Advanced Example - Serial Communication**:

```rust
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    
    ufmt::uwriteln!(&mut serial, "Arduino Rust UART Example\r").unwrap();
    
    let mut counter = 0u32;
    loop {
        ufmt::uwriteln!(&mut serial, "Count: {}\r", counter).unwrap();
        counter += 1;
        arduino_hal::delay_ms(1000);
    }
}
```

**Limitations**:

- `#![no_std]` - No standard library (use `core` only)
- 2KB RAM on Uno - Very constrained heap
- Limited debugging - Serial output or JTAG/debugWIRE
- Smaller ecosystem than Arduino C++ libraries

**When to Use Arduino Rust**:

- ✅ Learning embedded Rust on familiar hardware
- ✅ Simple sensor/actuator projects
- ✅ Ultra-low power applications (µW range)
- ❌ Complex algorithms needing >2KB RAM
- ❌ When Arduino C++ libraries are essential

---

#### **Raspberry Pi (ARM/Linux)**

**Approach 1: Standard Rust with Linux** ⭐ Recommended

Since Raspberry Pi runs full Linux, you get **standard Rust** with `std`:

```rust
// Cargo.toml
[dependencies]
rppal = "0.14"  // GPIO, PWM, SPI, I2C, UART access

// src/main.rs
use rppal::gpio::Gpio;
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gpio = Gpio::new()?;
    
    // GPIO17 as output (physical pin 11)
    let mut led = gpio.get(17)?.into_output();
    
    println!("Blinking LED on GPIO17...");
    
    loop {
        led.set_high();
        thread::sleep(Duration::from_millis(500));
        
        led.set_low();
        thread::sleep(Duration::from_millis(500));
    }
}
```

**Advanced - PWM Servo Control**:

```rust
use rppal::pwm::{Channel, Pwm, Polarity};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pwm = Pwm::with_frequency(
        Channel::Pwm0,
        50.0,  // 50 Hz for servo
        0.075, // 7.5% duty cycle = 1.5ms pulse = center position
        Polarity::Normal,
        true   // Enable
    )?;
    
    // Sweep servo from 0° to 180°
    for duty_cycle in (50..=125).step_by(5) {
        pwm.set_duty_cycle((duty_cycle as f64) / 1000.0)?;
        thread::sleep(Duration::from_millis(100));
    }
    
    Ok(())
}
```

**Cross-Compilation Workflow**:

```powershell
# On development PC (Windows/Linux/macOS)
rustup target add armv7-unknown-linux-gnueabihf  # Pi 2/3/4 (32-bit)
# OR
rustup target add aarch64-unknown-linux-gnu      # Pi 3/4/Zero 2 (64-bit)

# Build for Pi
cargo build --release --target=armv7-unknown-linux-gnueabihf

# Deploy to Pi
scp target/armv7-unknown-linux-gnueabihf/release/myapp pi@raspberrypi.local:~/
ssh pi@raspberrypi.local
./myapp
```

**Using `cross` for Easy Cross-Compilation**:

```powershell
# Install cross (Docker-based cross-compilation)
cargo install cross

# Build with cross (handles toolchain automatically)
cross build --release --target=armv7-unknown-linux-gnueabihf

# Or use custom Docker image with rppal dependencies
cross build --release
```

**Approach 2: Bare-Metal Raspberry Pi** (Advanced)

Run Rust **without Linux** on Raspberry Pi hardware:

- **Repository**: <https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials>
- Direct bootloader development
- Full hardware control (GPU, interrupts, etc.)
- Much more complex - for OS development education

---

### **RTOS Comparison Matrix**

| **RTOS** | **Maturity** | **Architecture** | **Memory Protection** | **Real-Time** | **Async** | **Production Use** | **Best For** |
|----------|--------------|------------------|----------------------|---------------|-----------|-------------------|--------------|
| **Tock** | High | Microkernel | ✅ MPU-based | Soft | ❌ | Google OpenTitan | Security-critical, untrusted code |
| **Embassy** | High | Async executor | ❌ | Soft | ✅ | ESP32 IoT | Power-efficient, concurrent I/O |
| **RTIC** | High | Interrupt scheduler | ❌ | Hard | Limited | Industrial | Deterministic timing, motor control |
| **FreeRTOS-rust** | Medium | Preemptive kernel | Varies | Hard | ❌ | Migration projects | Leveraging FreeRTOS ecosystem |
| **Drone** | Low | Fiber scheduler | ❌ | Soft | Fiber-based | Research | Experimental projects |

**Real-Time Classifications**:

- **Hard Real-Time**: Missing deadline = system failure (RTIC, FreeRTOS)
- **Soft Real-Time**: Missing deadline = degraded performance (Tock, Embassy, Drone)

---

### **Platform Comparison: Arduino vs Raspberry Pi**

| **Aspect** | **Arduino (AVR-HAL)** | **Raspberry Pi (rppal)** |
|------------|----------------------|--------------------------|
| **Development Complexity** | High (bare-metal, no_std) | Low (std Rust + Linux) |
| **Tooling** | Custom (ravedude, avr-gcc) | Standard (cargo, rustup) |
| **Debugging** | Serial prints, debugWIRE | gdb, println!, full Linux tools |
| **Memory** | 2KB RAM (Uno), 8KB (Mega) | 1-8GB RAM |
| **Storage** | 32KB Flash (Uno) | 8-64GB SD card |
| **CPU** | 16 MHz 8-bit (Uno) | 1.5 GHz quad-core ARM |
| **Real-Time** | ✅ Deterministic (bare-metal) | ❌ Linux scheduling jitter |
| **Power Consumption** | µW to mW | Watts |
| **GPIO Performance** | Nanosecond bit-banging | Microsecond (kernel overhead) |
| **Networking** | Shields required | Built-in WiFi/Ethernet |
| **Cost** | $3-25 | $15-75 |
| **Typical Use** | Sensors, simple control loops | Complex apps, vision, ML inference |

---

## 💡 **Key Takeaways**

1. **Rust embedded ecosystem is mature** - Multiple production-ready RTOS options for different use cases
2. **Safety without runtime cost** - Compile-time checks eliminate common embedded bugs with zero overhead
3. **Embassy dominates modern embedded** - Async/await brings ergonomic concurrency to microcontrollers
4. **Raspberry Pi simplest entry point** - Full `std` library with standard Rust tooling
5. **Arduino more challenging** - Bare-metal `no_std` environment requires embedded-specific knowledge
6. **Choose RTOS based on constraints**:
   - **Security**: Tock (MPU isolation)
   - **Power efficiency**: Embassy (async I/O)
   - **Hard real-time**: RTIC or FreeRTOS-rust
   - **Legacy integration**: FreeRTOS-rust
7. **Cross-compilation common** - Build on powerful PC, deploy to embedded target

---

## 🔗 **Integration Points**

### **Builds On**

- [[ownership-fundamentals]] - Ownership prevents embedded memory bugs
- [[smart-pointers]] - Box/Rc patterns in constrained environments
- [[async-await-basics]] - Embassy's async runtime foundation
- [[concurrency-fundamentals]] - Thread safety in embedded context

### **Enables**

- **Future Learning**:
  - [[embedded-hal-traits]] - Hardware abstraction layer design
  - [[no-std-programming]] - Programming without standard library
  - [[interrupt-driven-architecture]] - RTIC interrupt model
  - [[memory-mapped-io]] - Direct hardware register access
  - [[bootloader-development]] - Bare-metal Raspberry Pi OS

- **Practical Applications**:
  - IoT sensor networks with Embassy async
  - Motor control systems with RTIC hard real-time
  - Security tokens with Tock process isolation
  - Home automation with Raspberry Pi + rppal

### **Related Concepts**

- [[rust-for-linux]] - Rust in Linux kernel development
- [[webassembly-embedded]] - WASM on embedded (wasm3)
- [[formal-verification]] - Proving RTOS correctness
- [[zero-cost-abstractions]] - How Rust achieves embedded performance

### **Mission Integration Possibilities**

- **Mission 11: Embedded Data Structures** - Implement Mission 1-10 data structures in `no_std` environment
- **Mission 12: Real-Time Scheduler** - Build simple RTOS task scheduler using Rust traits
- **Mission 13: Hardware Abstraction** - Create embedded-hal implementations for custom hardware

### **Daily Study Applications**

- Week 9: Port daily study examples to Arduino/Raspberry Pi
- Week 10: Implement AoC problems on constrained embedded hardware (2KB RAM challenge!)
- Week 11: Compare `std` vs `no_std` implementations of same algorithm

---

## 📚 **Resources**

### **Official Documentation**

- [Embedded Rust Book](https://docs.rust-embedded.org/book/) - Comprehensive embedded Rust guide
- [Embassy Book](https://embassy.dev/book/) - Async embedded development
- [RTIC Book](https://rtic.rs/dev/book/en/) - Interrupt-driven concurrency
- [AVR-HAL Documentation](https://rahix.github.io/avr-hal/) - Arduino development
- [rppal Documentation](https://docs.rs/rppal/) - Raspberry Pi GPIO library

### **Hardware Resources**

- [Discovery Book](https://docs.rust-embedded.org/discovery/) - ARM Cortex-M beginner tutorial with STM32F3
- [Rust on ESP](https://esp-rs.github.io/book/) - ESP32 development in Rust
- [RP2040 HAL](https://github.com/rp-rs/rp-hal) - Raspberry Pi Pico (RP2040 chip)

### **Community & Tools**

- [Awesome Embedded Rust](https://github.com/rust-embedded/awesome-embedded-rust) - Curated embedded resources
- [probe-rs](https://probe.rs/) - Debugging and flashing tool for embedded Rust
- [defmt](https://defmt.ferrous-systems.com/) - Efficient logging for embedded
- [embedded-hal](https://docs.rs/embedded-hal/) - Hardware abstraction traits

### **Workspace Examples (Future)**

- `embedded/arduino_blink/` - AVR-HAL blink example
- `embedded/rpi_gpio/` - Raspberry Pi GPIO with rppal
- `embedded/embassy_async/` - ESP32 async example
- `embedded/rtic_motor_control/` - Hard real-time motor control

---

## 🎯 **Learning Path**

### **Beginner: Start with Raspberry Pi**

1. Install Rust on Raspberry Pi OS
2. Run standard Rust programs (full `std` support)
3. Use `rppal` for GPIO control
4. Build simple sensor reading application

**Why**: Familiar development experience, full debugging, gentle embedded introduction

### **Intermediate: Arduino AVR Development**

1. Install `ravedude` and AVR toolchain
2. Understand `#![no_std]` environment
3. Implement blink with `arduino-hal`
4. Learn serial communication for debugging

**Why**: True embedded constraints, bare-metal experience, common hobbyist platform

### **Advanced: ARM Cortex-M with RTOS**

1. Get STM32 Discovery board (~$10)
2. Follow Discovery Book tutorials
3. Implement tasks with Embassy async
4. Compare Embassy vs RTIC for same problem

**Why**: Production-grade RTOS patterns, modern embedded development, career-applicable skills

### **Expert: Bare-Metal OS Development**

1. Study Raspberry Pi bare-metal tutorials
2. Implement basic bootloader
3. Write interrupt handlers
4. Create simple task scheduler

**Why**: Deep understanding of hardware/software interface, OS internals knowledge

---

*Last Updated: November 17, 2025*

*Tags: #embedded #rtos #arduino #raspberry-pi #embassy #tock #rtic #freertos #avr-hal #rppal #no-std #async-embedded #real-time #iot #advanced*

*Links: [[zettel-index]] | [[Smart Pointers MOC]] | [[ownership-fundamentals]] | [[async-await-basics]] | [[concurrency-fundamentals]] | [[no-std-programming]] | [[embedded-hal-traits]]*
