use crate::cpu::{CpuCore, CpuState};
use crate::memory::{GuestMemory, MmioManager};
use crate::memory::mmio::{UartDevice, TimerDevice as MmioTimerDevice};
use crate::network::{NetworkDevice, NetworkManager};
use crate::devices::{ConsoleDevice, TimerDevice, InterruptController};
use crate::devices::interrupt::vectors;
use crate::Result;
use linux_loader::loader;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, interval};

/// Virtual machine implementation
pub struct VirtualMachine {
    memory: GuestMemory,
    cpu_cores: Vec<Arc<Mutex<CpuCore>>>,
    mmio_manager: Arc<Mutex<MmioManager>>,
    network_manager: Arc<Mutex<NetworkManager>>,
    interrupt_controller: Arc<Mutex<InterruptController>>,
    timer_manager: Arc<Mutex<TimerDevice>>,
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
        
        let timer = Box::new(MmioTimerDevice::new());
        mmio_manager.register_device(0x40, timer)?; // PIT
        
        let console = Box::new(ConsoleDevice::new(80, 25));
        mmio_manager.register_device(0xB8000, console)?; // VGA text buffer
        
        // Create interrupt controller
        let interrupt_controller = InterruptController::new();
        
        // Create timer manager
        let timer_manager = TimerDevice::new();
        
        // Create network manager
        let network_manager = NetworkManager::new();
        
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
            running: false,
            kernel_loaded: false,
        })
    }

    /// Load a kernel into the virtual machine
    pub async fn load_kernel(&mut self, kernel_path: &Path) -> Result<()> {
        log::info!("Loading kernel from {:?}", kernel_path);
        
        // Use linux-loader to load the kernel
        let kernel_data = std::fs::read(kernel_path)?;
        
        // For now, we'll do a simple kernel load at a fixed address
        // In a real implementation, this would use the linux-loader crate properly
        let kernel_start = 0x100000; // 1MB
        self.memory.write_slice(kernel_start, &kernel_data)?;
        
        // Set up initial CPU state
        if let Some(cpu_core) = self.cpu_cores.first() {
            let mut cpu_core = cpu_core.lock().await;
            cpu_core.start().await?;
        }
        
        // Set up interrupt handlers
        self.setup_interrupt_handlers().await?;
        
        self.kernel_loaded = true;
        log::info!("Kernel loaded successfully");
        Ok(())
    }

    /// Load an initrd into the virtual machine
    pub async fn load_initrd(&mut self, initrd_path: &Path) -> Result<()> {
        log::info!("Loading initrd from {:?}", initrd_path);
        
        let initrd_data = std::fs::read(initrd_path)?;
        let initrd_start = 0x2000000; // 32MB
        self.memory.write_slice(initrd_start, &initrd_data)?;
        
        log::info!("Initrd loaded successfully");
        Ok(())
    }

    /// Enable network support
    pub async fn enable_network(&mut self, interface_name: &str) -> Result<()> {
        log::info!("Enabling network on interface {}", interface_name);
        
        let network_device = NetworkDevice::new(interface_name)?;
        let mut network_manager = self.network_manager.lock().await;
        network_manager.add_device(network_device)?;
        network_manager.start().await?;
        
        log::info!("Network enabled successfully");
        Ok(())
    }

    /// Set up interrupt handlers
    async fn setup_interrupt_handlers(&mut self) -> Result<()> {
        let interrupt_controller = self.interrupt_controller.clone();
        let timer_manager = self.timer_manager.clone();
        
        // Timer interrupt handler
        let timer_handler = {
            let interrupt_controller = interrupt_controller.clone();
            move || -> Result<()> {
                // Handle timer interrupt
                log::debug!("Timer interrupt");
                Ok(())
            }
        };
        
        let mut ic = self.interrupt_controller.lock().await;
        ic.register_handler(vectors::TIMER, timer_handler);
        
        // System call handler
        let syscall_handler = {
            let interrupt_controller = interrupt_controller.clone();
            move || -> Result<()> {
                // Handle system call
                log::debug!("System call interrupt");
                Ok(())
            }
        };
        
        ic.register_handler(vectors::SYSTEM_CALL, syscall_handler);
        
        Ok(())
    }

    /// Run the virtual machine
    pub async fn run(&mut self) -> Result<()> {
        if !self.kernel_loaded {
            return Err(crate::EmulatorError::Cpu("Kernel not loaded".to_string()));
        }
        
        self.running = true;
        log::info!("Starting virtual machine execution");
        
        // Start timer
        let timer_manager = self.timer_manager.clone();
        let interrupt_controller = self.interrupt_controller.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(10));
            loop {
                interval.tick().await;
                
                let mut timer = timer_manager.lock().await;
                timer.tick();
                
                let mut ic = interrupt_controller.lock().await;
                if let Err(e) = ic.process_interrupts() {
                    log::error!("Error processing interrupts: {}", e);
                }
            }
        });
        
        // Start CPU cores
        let mut cpu_tasks = Vec::new();
        for (i, cpu_core) in self.cpu_cores.iter().enumerate() {
            let cpu_core = cpu_core.clone();
            let task = tokio::spawn(async move {
                let mut cpu_core = cpu_core.lock().await;
                cpu_core.start().await?;
                
                loop {
                    match cpu_core.execute_cycle().await {
                        Ok(continue_execution) => {
                            if !continue_execution {
                                break;
                            }
                        }
                        Err(e) => {
                            log::error!("CPU core {} error: {}", i, e);
                            break;
                        }
                    }
                    
                    // Yield to other tasks
                    tokio::task::yield_now().await;
                }
                
                Ok::<(), crate::EmulatorError>(())
            });
            cpu_tasks.push(task);
        }
        
        // Wait for all CPU cores to complete
        for (i, task) in cpu_tasks.into_iter().enumerate() {
            match task.await {
                Ok(Ok(())) => {
                    log::info!("CPU core {} completed successfully", i);
                }
                Ok(Err(e)) => {
                    log::error!("CPU core {} failed: {}", i, e);
                }
                Err(e) => {
                    log::error!("CPU core {} task failed: {}", i, e);
                }
            }
        }
        
        self.running = false;
        log::info!("Virtual machine execution completed");
        Ok(())
    }

    /// Stop the virtual machine
    pub async fn stop(&mut self) -> Result<()> {
        self.running = false;
        
        // Stop all CPU cores
        for cpu_core in &self.cpu_cores {
            let mut cpu_core = cpu_core.lock().await;
            cpu_core.stop().await?;
        }
        
        // Stop network manager
        let mut network_manager = self.network_manager.lock().await;
        network_manager.stop();
        
        log::info!("Virtual machine stopped");
        Ok(())
    }

    /// Get memory reference
    pub fn get_memory(&self) -> &GuestMemory {
        &self.memory
    }

    pub fn get_memory_mut(&mut self) -> &mut GuestMemory {
        &mut self.memory
    }

    /// Get MMIO manager reference
    pub fn get_mmio_manager(&self) -> Arc<Mutex<MmioManager>> {
        self.mmio_manager.clone()
    }

    /// Check if the VM is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Get CPU core count
    pub fn cpu_core_count(&self) -> usize {
        self.cpu_cores.len()
    }
}
