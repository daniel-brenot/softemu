use crate::memory::mmio::MmioDevice;
use crate::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, interval};

/// Timer device for system timing
pub struct TimerDevice {
    counter: u64,
    reload_value: u64,
    control: u32,
    interrupt_enabled: bool,
    interrupt_callback: Option<Arc<dyn Fn(u8) + Send + Sync>>,
}

impl TimerDevice {
    pub fn new() -> Self {
        Self {
            counter: 0,
            reload_value: 1000000, // 1 second at 1MHz
            control: 0,
            interrupt_enabled: false,
            interrupt_callback: None,
        }
    }

    pub fn set_interrupt_callback<F>(&mut self, callback: F)
    where
        F: Fn(u8) + Send + Sync + 'static,
    {
        self.interrupt_callback = Some(Arc::new(callback));
    }

    pub fn tick(&mut self) {
        if self.control & 0x1 != 0 { // Timer enabled
            if self.counter > 0 {
                self.counter -= 1;
            } else {
                // Timer expired
                self.counter = self.reload_value;
                
                // Trigger interrupt if enabled
                if self.interrupt_enabled {
                    if let Some(callback) = &self.interrupt_callback {
                        callback(0x20); // Timer interrupt vector
                    }
                }
            }
        }
    }

    pub fn start(&mut self) {
        self.control |= 0x1;
    }

    pub fn stop(&mut self) {
        self.control &= !0x1;
    }

    pub fn is_running(&self) -> bool {
        self.control & 0x1 != 0
    }

    pub fn set_frequency(&mut self, frequency: u64) {
        // Assuming 1MHz base frequency
        self.reload_value = 1000000 / frequency;
    }

    pub fn get_frequency(&self) -> u64 {
        if self.reload_value > 0 {
            1000000 / self.reload_value
        } else {
            0
        }
    }
}

impl MmioDevice for TimerDevice {
    fn read(&self, offset: u64, size: u8) -> Result<u64> {
        match offset {
            0 => Ok(self.counter), // Counter register
            8 => Ok(self.reload_value), // Reload register
            16 => Ok(self.control as u64), // Control register
            24 => Ok(if self.interrupt_enabled { 1 } else { 0 }), // Interrupt enable
            32 => Ok(self.get_frequency()), // Frequency register
            _ => Ok(0),
        }
    }

    fn write(&mut self, offset: u64, value: u64, size: u8) -> Result<()> {
        match offset {
            0 => {
                // Counter register
                self.counter = value;
                Ok(())
            }
            8 => {
                // Reload register
                self.reload_value = value;
                Ok(())
            }
            16 => {
                // Control register
                self.control = (value & 0xFFFFFFFF) as u32;
                Ok(())
            }
            24 => {
                // Interrupt enable
                self.interrupt_enabled = (value & 0x1) != 0;
                Ok(())
            }
            32 => {
                // Frequency register
                self.set_frequency(value);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn name(&self) -> &str {
        "Timer"
    }

    fn size(&self) -> u64 {
        0x100 // 256 bytes
    }
}

/// Timer manager for handling multiple timers
pub struct TimerManager {
    timers: Vec<Arc<Mutex<TimerDevice>>>,
    running: bool,
}

impl TimerManager {
    pub fn new() -> Self {
        Self {
            timers: Vec::new(),
            running: false,
        }
    }

    pub fn add_timer(&mut self, timer: TimerDevice) -> usize {
        let timer = Arc::new(Mutex::new(timer));
        self.timers.push(timer);
        self.timers.len() - 1
    }

    pub async fn start(&mut self) -> Result<()> {
        self.running = true;
        
        // Start timer processing task
        let timers = self.timers.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_micros(1)); // 1MHz tick rate
            
            loop {
                interval.tick().await;
                
                for timer in &timers {
                    let mut timer = timer.lock().await;
                    timer.tick();
                }
            }
        });

        log::info!("Timer manager started with {} timers", self.timers.len());
        Ok(())
    }

    pub fn stop(&mut self) {
        self.running = false;
        log::info!("Timer manager stopped");
    }

    pub fn get_timer(&self, index: usize) -> Option<Arc<Mutex<TimerDevice>>> {
        self.timers.get(index).cloned()
    }

    pub fn timer_count(&self) -> usize {
        self.timers.len()
    }
}
