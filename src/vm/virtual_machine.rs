use anyhow::{bail, Context};

use crate::memory::mmio::UartDevice;
use crate::memory::GuestMemory;
use crate::cpu::{CpuCore, CpuState};
use crate::devices::{ConsoleDevice, TimerDevice, InterruptController};
use crate::memory::MmioManager;
use crate::network::{NetworkDevice, NetworkManager};
use crate::acpi::AcpiManager;
use crate::Result;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Synchronous virtual machine implementation
pub struct VirtualMachine {
    memory: GuestMemory,
    cpu_cores: Vec<Arc<Mutex<CpuCore>>>,
    mmio_manager: Arc<Mutex<MmioManager>>,
    network_manager: Arc<Mutex<NetworkManager>>,
    interrupt_controller: Arc<Mutex<InterruptController>>,
    timer_manager: Arc<Mutex<TimerDevice>>,
    acpi_manager: Arc<Mutex<AcpiManager>>,
    running: bool,
    kernel_loaded: bool,
}

impl VirtualMachine {
    /// Create a new virtual machine
    pub fn new(memory_size: u64, cpu_cores: u32) -> Result<Self> {
        // Create guest memory
        let memory = GuestMemory::new(memory_size)?;
        
        // Create MMIO manager
        let mut mmio_manager = MmioManager::new();
        
        // Register MMIO devices
        let uart = Box::new(UartDevice::new());
        mmio_manager.register_device(0x3F8, uart)?; // COM1

        let timer = Box::new(TimerDevice::new());
        mmio_manager.register_device(0x40, timer)?; // PIT
        
        let console = Box::new(ConsoleDevice::new(80, 25));
        mmio_manager.register_device(0xB8000, console)?; // VGA text buffer
        
        // Create interrupt controller
        let interrupt_controller = InterruptController::new();
        
        // Create timer manager
        let timer_manager = TimerDevice::new();
        
        // Create network manager
        let network_manager = NetworkManager::new();
        
        // Create ACPI manager
        let mut acpi_manager = AcpiManager::new();
        acpi_manager.initialize(memory.clone())?;
        acpi_manager.create_default_devices()?;
        
        // Create CPU cores
        let mut cpu_cores_vec = Vec::new();
        for i in 0..cpu_cores {
            let cpu_state = CpuState::new(memory.clone());
            let cpu_core = CpuCore::new(Arc::new(Mutex::new(cpu_state)), i);
            cpu_cores_vec.push(Arc::new(Mutex::new(cpu_core)));
        }
        
        Ok(Self {
            memory,
            cpu_cores: cpu_cores_vec,
            mmio_manager: Arc::new(Mutex::new(mmio_manager)),
            network_manager: Arc::new(Mutex::new(network_manager)),
            interrupt_controller: Arc::new(Mutex::new(interrupt_controller)),
            timer_manager: Arc::new(Mutex::new(timer_manager)),
            acpi_manager: Arc::new(Mutex::new(acpi_manager)),
            running: false,
            kernel_loaded: false,
        })
    }

    /// Load a kernel into the virtual machine
    pub fn load_kernel_from_path(&mut self, kernel_path: &Path) -> Result<()> {
        log::info!("Loading kernel from {:?}", kernel_path);
        
        // Use linux-loader to load the kernel
        let kernel_data = std::fs::read(kernel_path)?;
        
        // For now, we'll do a simple kernel load at a fixed address
        // In a real implementation, this would use the linux-loader crate properly
        let kernel_start = 0x100000; // 1MB
        self.memory.write_slice(kernel_start, &kernel_data)?;
        
        // Set up initial CPU state
        if let Some(cpu_core) = self.cpu_cores.first() {
            let mut cpu_core = cpu_core.lock().map_err(|_| { crate::EmulatorError::Cpu("Failed to lock CPU core".to_string()) })?;
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
        self.memory.write_slice(kernel_start, kernel_data)?;
        self.kernel_loaded = true;
        log::info!("Kernel loaded successfully");
        Ok(())
    }

    /// Get a reference to the guest memory
    pub fn get_memory(&self) -> &GuestMemory {
        &self.memory
    }

    /// Get a mutable reference to the guest memory
    pub fn get_memory_mut(&mut self) -> &mut GuestMemory {
        &mut self.memory
    }


    /// Load an initrd into memory
    pub fn load_initrd(&mut self, initrd_data: &[u8]) -> Result<()> {
        let initrd_start = 0x200000; // 2MB
        self.memory.write_slice(initrd_start, initrd_data)?;
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
        let mut ic = self.interrupt_controller.lock().unwrap();
        
        // Timer interrupt handler
        let timer_handler = || -> Result<()> {
            log::debug!("Timer interrupt");
            Ok(())
        };
        ic.register_handler(0x20, timer_handler); // Timer interrupt vector
        
        // System call handler
        let syscall_handler = || -> Result<()> {
            log::debug!("System call interrupt");
            Ok(())
        };
        ic.register_handler(0x80, syscall_handler); // System call vector
        
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
            let mut cpu_core = cpu_core.lock().unwrap();
            cpu_core.start()?;
            log::info!("CPU core {} initialized", i);
        }
        
        // Main execution loop
        let mut last_timer_tick = Instant::now();
        let timer_interval = Duration::from_millis(10);
        
        while self.running {
            // Process timer interrupts
            if last_timer_tick.elapsed() >= timer_interval {
                {
                    let mut timer = self.timer_manager.lock().unwrap();
                    timer.tick();
                }
                
                {
                    let mut ic = self.interrupt_controller.lock().unwrap();
                    if let Err(e) = ic.process_interrupts() {
                        log::error!("Error processing interrupts: {}", e);
                    }
                }
                
                // Process ACPI events
                if let Err(e) = self.process_acpi_events() {
                    log::error!("Error processing ACPI events: {}", e);
                }
                
                last_timer_tick = Instant::now();
            }
            
            // Execute CPU cycles
            for (i, cpu_core) in self.cpu_cores.iter().enumerate() {
                let mut cpu_core = cpu_core.lock().unwrap();
                match cpu_core.execute_cycle() {
                    Ok(continue_execution) => {
                        if !continue_execution {
                            log::info!("CPU core {} halted", i);
                            self.running = false;
                            break;
                        }
                    }
                    Err(e) => {
                        log::error!("CPU core {} error: {}", i, e);
                        self.running = false;
                        break;
                    }
                }
            }
            
            // Small delay to prevent 100% CPU usage
            // std::thread::sleep(Duration::from_micros(1));
        }
        
        log::info!("Virtual machine execution completed");
        Ok(())
    }

    /// Stop the virtual machine
    pub fn stop(&mut self) -> Result<()> {
        self.running = false;
        log::info!("Stopping virtual machine");
        
        // Stop all CPU cores
        for (i, cpu_core) in self.cpu_cores.iter().enumerate() {
            let mut cpu_core = cpu_core.lock().unwrap();
            cpu_core.stop()?;
            log::info!("CPU core {} stopped", i);
        }
        
        // Stop network manager
        let mut network_manager = self.network_manager.lock().unwrap();
        network_manager.stop();
        
        // Shutdown ACPI
        let mut acpi_manager = self.acpi_manager.lock().unwrap();
        acpi_manager.shutdown()?;
        
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
        self.memory.size()
    }

    /// Get ACPI manager
    pub fn get_acpi_manager(&self) -> &Arc<Mutex<AcpiManager>> {
        &self.acpi_manager
    }

    /// Handle power button press
    pub fn handle_power_button(&mut self) -> Result<()> {
        let mut acpi_manager = self.acpi_manager.lock().unwrap();
        acpi_manager.handle_power_button()?;
        Ok(())
    }

    /// Handle sleep button press
    pub fn handle_sleep_button(&mut self) -> Result<()> {
        let mut acpi_manager = self.acpi_manager.lock().unwrap();
        acpi_manager.handle_sleep_button()?;
        Ok(())
    }

    /// Handle lid switch
    pub fn handle_lid_switch(&mut self, closed: bool) -> Result<()> {
        let mut acpi_manager = self.acpi_manager.lock().unwrap();
        acpi_manager.handle_lid_switch(closed)?;
        Ok(())
    }

    /// Get current ACPI power state
    pub fn get_acpi_power_state(&self) -> crate::acpi::AcpiPowerState {
        let acpi_manager = self.acpi_manager.lock().unwrap();
        acpi_manager.get_power_state()
    }

    /// Transition to new ACPI power state
    pub fn transition_acpi_power_state(&mut self, state: crate::acpi::AcpiPowerState) -> Result<()> {
        let mut acpi_manager = self.acpi_manager.lock().unwrap();
        acpi_manager.transition_power_state(state)?;
        Ok(())
    }

    /// Process ACPI events
    pub fn process_acpi_events(&mut self) -> Result<()> {
        let mut acpi_manager = self.acpi_manager.lock().unwrap();
        acpi_manager.process_events()?;
        Ok(())
    }

    /// Get ACPI power consumption
    pub fn get_acpi_power_consumption(&self) -> f32 {
        let acpi_manager = self.acpi_manager.lock().unwrap();
        acpi_manager.get_power_consumption()
    }

    /// Get ACPI wake sources
    pub fn get_acpi_wake_sources(&self) -> Vec<String> {
        let acpi_manager = self.acpi_manager.lock().unwrap();
        acpi_manager.get_wake_sources()
    }
}
