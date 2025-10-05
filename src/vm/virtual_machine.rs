
use crate::memory::{GuestMemory, MemoryManager, MmioManager};
use crate::cpu::CpuCore;
use crate::devices::{SerialConsole, TimerDevice, InterruptController};
use crate::network::NetworkManager;
use crate::acpi::AcpiManager;
use crate::Result;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

/// Synchronous virtual machine implementation
pub struct VirtualMachine {
    pub cpu_cores: Vec<CpuCore>,
    // pub network_manager: NetworkManager,
    pub memory_manager: Arc<MemoryManager>,
    pub interrupt_controller: InterruptController,
    pub timer_manager: TimerDevice,
    pub acpi_manager: AcpiManager,
    pub running: bool,
    pub kernel_loaded: bool,
}

impl VirtualMachine {
    /// Create a new virtual machine
    pub fn new(memory_size: u64, cpu_core_count: u32) -> Result<Self> {
        // Create guest memory
        let guest_memory = GuestMemory::new(memory_size)?;
        
        // Create MMIO manager
        let mut mmio_manager = MmioManager::new();
        
        // Register MMIO devices
        let console = Box::new(SerialConsole::new());
        mmio_manager.register_device(0x3F8, console)?; // COM1

        let timer = Box::new(TimerDevice::new());
        mmio_manager.register_device(0x40, timer)?; // PIT
        
        // Create shared MMIO manager reference
        
        // Define MMIO space size (first 1MB is reserved for MMIO)
        let mmio_space_size = 0x100000; // 1MB
        
        // Create memory manager
        let memory_manager = Arc::new(MemoryManager::new(guest_memory, mmio_manager, mmio_space_size));
        
        // Create interrupt controller
        let interrupt_controller = InterruptController::new();
        
        // Create timer manager
        let timer_manager = TimerDevice::new();
        
        // Create network manager
        let network_manager = NetworkManager::new();
        
        // Create ACPI manager
        let mut acpi_manager = AcpiManager::new();
        acpi_manager.initialize(&*memory_manager.get_guest_memory())?;

        acpi_manager.create_default_devices()?;
        
        // Create CPU cores
        let mut cpu_cores = Vec::new();
        for i in 0..cpu_core_count {
            let cpu_core = CpuCore::new(i);
            cpu_cores.push(cpu_core);
        }
        
        Ok(Self {
            cpu_cores,
            memory_manager,
            // network_manager,
            interrupt_controller,
            timer_manager,
            acpi_manager,
            running: false,
            kernel_loaded: false,
        })
    }

    /// Load a kernel into the virtual machine
    pub fn load_kernel_from_path(&mut self, kernel_path: &Path) -> Result<()> {
        log::info!("Loading kernel from {:?}", kernel_path);
        
        // TODO: Use linux-loader to load the kernel
        let kernel_data = std::fs::read(kernel_path)?;
        
        // For now, we'll do a simple kernel load at a fixed address
        let kernel_start = 0x100000; // 1MB
        self.memory_manager.write_slice(kernel_start, &kernel_data)?;
        
        // Set up initial CPU state
        if let Some(cpu_core) = self.cpu_cores.first_mut() {
            cpu_core.start()?;
        }
        
        // Set up interrupt handlers
        self.setup_interrupt_handlers()?;
        
        self.kernel_loaded = true;
        log::info!("Kernel loaded successfully");
        Ok(())
    }

    /// Load a kernel into memory
    pub fn load_kernel(&mut self, kernel_data: &[u8]) -> Result<()> {
        let kernel_start = 0x100000; // 1MB
        self.memory_manager.write_slice(kernel_start, kernel_data)?;
        
        self.kernel_loaded = true;
        log::info!("Kernel loaded successfully");
        Ok(())
    }

    /// Load an initrd into memory
    pub fn load_initrd(&mut self, initrd_data: &[u8]) -> Result<()> {
        let initrd_start = 0x200000; // 2MB
        
        self.memory_manager.write_slice(initrd_start, initrd_data)?;
        
        log::info!("Initrd loaded at 0x{:x}", initrd_start);
        Ok(())
    }

    // /// Enable network support
    // pub fn enable_network(&mut self, interface_name: &str) -> Result<()> {
    //     log::info!("Enabling network on interface {}", interface_name);
        
    //     let network_device = NetworkDevice::new(interface_name)?;
    //     let mut network_manager = self.network_manager.lock().map_err(|e| e.into())?;
    //     network_manager.add_device(network_device)?;
    //     network_manager.start()?;
        
    //     log::info!("Network enabled successfully");
    //     Ok(())
    // }

    /// Set up interrupt handlers
    fn setup_interrupt_handlers(&mut self) -> Result<()> {
        
        // Timer interrupt handler
        let timer_handler = || -> Result<()> {
            log::debug!("Timer interrupt");
            Ok(())
        };
        // Timer interrupt vector
        self.interrupt_controller.register_handler(0x20, timer_handler);
        
        // System call handler
        let syscall_handler = || -> Result<()> {
            log::debug!("System call interrupt");
            Ok(())
        };
        self.interrupt_controller.register_handler(0x80, syscall_handler); // System call vector
        
        Ok(())
    }

    /// Run the virtual machine synchronously
    pub fn run(&mut self) -> Result<()> {
        if !self.kernel_loaded {
            return Err(crate::EmulatorError::Cpu("Kernel not loaded".to_string()));
        }
        
        self.running = true;
        log::info!("Starting synchronous virtual machine execution");
        
        // Set up interrupt handlers
        self.setup_interrupt_handlers()?;
        
        // Initialize CPU cores
        for (i, cpu_core) in self.cpu_cores.iter().enumerate() {
            cpu_core.start()?;
            log::info!("CPU core {} initialized", i);
        }
        
        // Main execution loop
        let mut last_timer_tick = Instant::now();
        let timer_interval = Duration::from_millis(10);
        
        while self.running {
            // Process timer interrupts
            if last_timer_tick.elapsed() >= timer_interval {
                
                self.timer_manager.tick();
                
                if let Err(e) = self.interrupt_controller.process_interrupts() {
                    log::error!("Error processing interrupts: {}", e);
                }
                
                
                // Process ACPI events
                if let Err(e) = self.process_acpi_events() {
                    log::error!("Error processing ACPI events: {}", e);
                }
                
                last_timer_tick = Instant::now();
            }
            
            // Execute CPU cycles
            for (i, cpu_core) in self.cpu_cores.iter().enumerate().rev() {
                thread::spawn(move || {
                    loop {

                    }
                });
            }
            
            // Small delay to prevent 100% CPU usage
            // std::thread::sleep(Duration::from_micros(1));
        }
        
        log::info!("Virtual machine execution completed");
        Ok(())
    }

    /// Check if the VM is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Get the number of CPU cores
    pub fn cpu_count(&self) -> usize {
        self.cpu_cores.len()
    }

    /// Get memory size
    pub fn memory_size(&self) -> u64 {
        self.memory_manager.total_address_space_size()
    }

    /// Read from memory with MMIO routing
    pub fn read_memory(&self, addr: u64, size: u8) -> Result<u64> {
        match size {
            1 => Ok(self.memory_manager.read_u8(addr)? as u64),
            2 => Ok(self.memory_manager.read_u16(addr)? as u64),
            4 => Ok(self.memory_manager.read_u32(addr)? as u64),
            8 => Ok(self.memory_manager.read_u64(addr)?),
            _ => Err(crate::EmulatorError::Memory(format!("Unsupported read size: {}", size))),
        }
    }

    /// Write to memory with MMIO routing
    pub fn write_memory(&mut self, addr: u64, value: u64, size: u8) -> Result<()> {
        match size {
            1 => self.memory_manager.write_u8(addr, value as u8),
            2 => self.memory_manager.write_u16(addr, value as u16),
            4 => self.memory_manager.write_u32(addr, value as u32),
            8 => self.memory_manager.write_u64(addr, value),
            _ => Err(crate::EmulatorError::Memory(format!("Unsupported write size: {}", size))),
        }
    }
}
