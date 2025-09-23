# SoftEmu - Software-based x86_64 Emulator

A software-based x86_64 virtual machine emulator written in Rust that runs without hardware virtualization (no KVM, Hyper-V, etc.). This emulator uses MMIO for device communication and includes network support.

## Features

- **Pure Software Emulation**: No hardware virtualization required
- **x86_64 CPU Emulation**: Complete instruction set support using iced-x86
- **Memory Management**: Uses vm-memory crate for efficient guest memory handling
- **MMIO Devices**: UART, Timer, Console, and other devices via MMIO
- **Network Support**: Full network stack with packet processing
- **Cross-Platform**: Runs on Windows, macOS, and Linux
- **Multi-Core**: Supports multiple CPU cores
- **Kernel Loading**: Uses linux-loader for kernel and initrd loading

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    SoftEmu Virtual Machine                 │
├─────────────────────────────────────────────────────────────┤
│  CPU Cores (x86_64)  │  Memory Management  │  MMIO Devices │
│  - Instruction Decode │  - Guest Memory     │  - UART       │
│  - Register State     │  - Address Trans.   │  - Timer      │
│  - Interrupt Handling │  - Memory Mapping   │  - Console    │
├─────────────────────────────────────────────────────────────┤
│  Network Stack        │  Device Manager     │  Interrupts  │
│  - Packet Processing  │  - MMIO Routing     │  - Handlers   │
│  - Protocol Support   │  - Device Discovery │  - Vectors    │
└─────────────────────────────────────────────────────────────┘
```

## Dependencies

- **vm-memory**: Guest memory management
- **vm-device**: Device abstraction
- **linux-loader**: Kernel and initrd loading
- **iced-x86**: x86_64 instruction decoding
- **memfd**: Memory file descriptors
- **tokio**: Async runtime
- **pnet**: Network packet processing

## Building

```bash
# Clone the repository
git clone <repository-url>
cd softemu

# Build the project
cargo build --release

# Run with a kernel
cargo run --release -- --kernel vmlinux --memory 512 --cores 2
```

## Usage

### Basic Usage

```bash
# Run with a kernel image
./target/release/softemu --kernel vmlinux --memory 512

# Run with kernel and initrd
./target/release/softemu --kernel vmlinux --initrd initrd.img --memory 1024

# Run with network support
./target/release/softemu --kernel vmlinux --network eth0 --memory 512

# Run with debug output
./target/release/softemu --kernel vmlinux --debug --memory 512
```

### Command Line Options

- `--kernel <path>`: Path to the kernel image to load
- `--initrd <path>`: Path to the initrd (optional)
- `--memory <size>`: Amount of RAM in MB (default: 512)
- `--cores <count>`: Number of CPU cores to emulate (default: 1)
- `--network <interface>`: Network interface name (optional)
- `--debug`: Enable debug output

### Example Configuration

```rust
use softemu::VirtualMachine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create VM with 1GB RAM and 2 cores
    let mut vm = VirtualMachine::new(1024 * 1024 * 1024, 2)?;
    
    // Load kernel
    vm.load_kernel(std::path::Path::new("vmlinux")).await?;
    
    // Load initrd
    vm.load_initrd(std::path::Path::new("initrd.img")).await?;
    
    // Enable network
    vm.enable_network("eth0").await?;
    
    // Run the VM
    vm.run().await?;
    
    Ok(())
}
```

## MMIO Device Map

| Address Range | Device | Description |
|---------------|--------|-------------|
| 0x3F8-0x3FF  | UART   | Serial console |
| 0x40-0x43    | Timer  | Programmable interval timer |
| 0xB8000      | Console| VGA text buffer |
| 0x100000     | Kernel | Kernel image start |
| 0x2000000    | Initrd | Initrd image start |

## Interrupt Vectors

| Vector | Description |
|--------|-------------|
| 0x20   | Timer interrupt |
| 0x21   | Keyboard interrupt |
| 0x80   | System call |
| 0x0E   | Page fault |
| 0x0D   | General protection fault |

## Network Support

The emulator includes a full network stack with:

- Ethernet frame processing
- IPv4 packet handling
- ICMP ping support
- ARP protocol
- TCP/UDP support (basic)

Network devices are mapped to host network interfaces and can be configured via the `--network` option.

## Development

### Project Structure

```
src/
├── main.rs              # CLI entry point
├── lib.rs               # Library exports
├── cpu/                 # CPU emulation
│   ├── core.rs         # CPU core implementation
│   ├── instruction.rs  # Instruction decoder/executor
│   ├── registers.rs    # CPU registers
│   └── state.rs        # CPU state management
├── memory/              # Memory management
│   ├── guest_memory.rs # Guest memory implementation
│   └── mmio.rs         # MMIO device handling
├── network/             # Network support
│   ├── device.rs       # Network device
│   └── manager.rs      # Network manager
├── devices/             # Virtual devices
│   ├── console.rs      # Console device
│   ├── timer.rs        # Timer device
│   └── interrupt.rs    # Interrupt controller
└── vm/                  # VM orchestration
    └── virtual_machine.rs # Main VM implementation
```

### Adding New Devices

1. Implement the `MmioDevice` trait
2. Register the device with the MMIO manager
3. Add interrupt handlers if needed

```rust
use crate::memory::mmio::MmioDevice;

struct MyDevice {
    // Device state
}

impl MmioDevice for MyDevice {
    fn read(&self, offset: u64, size: u8) -> Result<u64> {
        // Handle read operations
    }
    
    fn write(&mut self, offset: u64, value: u64, size: u8) -> Result<()> {
        // Handle write operations
    }
    
    fn name(&self) -> &str { "MyDevice" }
    fn size(&self) -> u64 { 0x100 }
}
```

## Performance

The emulator is designed for correctness over speed. Performance characteristics:

- **CPU Emulation**: ~1-10 MIPS depending on instruction complexity
- **Memory Access**: Near-native speed for guest memory operations
- **Network**: Limited by host network interface speed
- **I/O**: MMIO operations are fast but not optimized for high throughput

## Limitations

- No hardware acceleration (by design)
- Limited device support (UART, Timer, Console, Network)
- Basic interrupt handling
- No PCIe support (MMIO only)
- Simplified memory management
- No SMP support yet

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Acknowledgments

- [iced-x86](https://github.com/icedland/iced) for x86_64 instruction decoding
- [vm-memory](https://github.com/rust-vmm/vm-memory) for guest memory management
- [linux-loader](https://github.com/rust-vmm/linux-loader) for kernel loading
- [pnet](https://github.com/libpnet/libpnet) for network packet processing
