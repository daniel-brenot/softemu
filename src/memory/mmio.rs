use crate::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// MMIO device trait
pub trait MmioDevice: Send + Sync {
    /// Read from MMIO device
    fn read(&self, offset: u64, size: u8) -> Result<u64>;
    
    /// Write to MMIO device
    fn write(&mut self, offset: u64, value: u64, size: u8) -> Result<()>;
    
    /// Get device name
    fn name(&self) -> &str;
    
    /// Get device size
    fn size(&self) -> u64;
}

/// MMIO device manager
pub struct MmioManager {
    devices: HashMap<u64, Box<dyn MmioDevice>>,
    device_ranges: Vec<(u64, u64, String)>, // (start, end, name)
}

impl std::fmt::Debug for MmioManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MmioManager")
            .field("device_ranges", &self.device_ranges)
            .field("device_count", &self.devices.len())
            .finish()
    }
}

impl MmioManager {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
            device_ranges: Vec::new(),
        }
    }

    /// Register an MMIO device
    pub fn register_device(&mut self, start_addr: u64, device: Box<dyn MmioDevice>) -> Result<()> {
        let size = device.size();
        let name = device.name().to_string();
        let end_addr = start_addr + size - 1;

        // Check for overlaps
        for (existing_start, existing_end, existing_name) in &self.device_ranges {
            if start_addr <= *existing_end && end_addr >= *existing_start {
                return Err(crate::EmulatorError::Device(
                    format!("Device {} overlaps with existing device {}", name, existing_name)
                ));
            }
        }

        self.devices.insert(start_addr, device);
        self.device_ranges.push((start_addr, end_addr, name.clone()));
        self.device_ranges.sort_by_key(|(start, _, _)| *start);

        log::info!("Registered MMIO device {} at 0x{:x}-0x{:x}", name, start_addr, end_addr);
        Ok(())
    }

    /// Handle MMIO read
    pub fn read(&self, addr: u64, size: u8) -> Result<u64> {
        if let Some((device_addr, device)) = self.find_device(addr) {
            let offset = addr - device_addr;
            device.read(offset, size)
        } else {
            Err(crate::EmulatorError::Device(format!("No MMIO device at address 0x{:x}", addr)))
        }
    }

    /// Handle MMIO write
    pub fn write(&mut self, addr: u64, value: u64, size: u8) -> Result<()> {
        if let Some((device_addr, device)) = self.find_device_mut(addr) {
            let offset = addr - device_addr;
            device.write(offset, value, size)
        } else {
            Err(crate::EmulatorError::Device(format!("No MMIO device at address 0x{:x}", addr)))
        }
    }

    /// Check if an address is in MMIO space
    pub fn is_mmio_address(&self, addr: u64) -> bool {
        self.find_device(addr).is_some()
    }

    /// Find device for an address
    fn find_device(&self, addr: u64) -> Option<(u64, &dyn MmioDevice)> {
        for (start_addr, device) in &self.devices {
            if addr >= *start_addr && addr < *start_addr + device.size() {
                return Some((*start_addr, device.as_ref()));
            }
        }
        None
    }

    /// Find device for an address (mutable)
    fn find_device_mut(&mut self, addr: u64) -> Option<(u64, &mut dyn MmioDevice)> {
        for (start_addr, device) in &mut self.devices {
            if addr >= *start_addr && addr < *start_addr + device.size() {
                return Some((*start_addr, device.as_mut()));
            }
        }
        None
    }

    /// List all registered devices
    pub fn list_devices(&self) -> Vec<(u64, u64, String)> {
        self.device_ranges.clone()
    }
}

/// Simple UART device for console output
pub struct UartDevice {
    data: u8,
    status: u8,
    control: u8,
}

impl UartDevice {
    pub fn new() -> Self {
        Self {
            data: 0,
            status: 0x20, // Transmit ready
            control: 0,
        }
    }
}

impl MmioDevice for UartDevice {
    fn read(&self, offset: u64, size: u8) -> Result<u64> {
        match offset {
            0 => Ok(self.data as u64), // Data register
            1 => Ok(self.status as u64), // Status register
            2 => Ok(self.control as u64), // Control register
            _ => Ok(0),
        }
    }

    fn write(&mut self, offset: u64, value: u64, size: u8) -> Result<()> {
        match offset {
            0 => {
                // Data register - output character
                if size == 1 {
                    let ch = (value & 0xFF) as u8;
                    if ch != 0 {
                        print!("{}", ch as char);
                        std::io::Write::flush(&mut std::io::stdout())?;
                    }
                }
                Ok(())
            }
            1 => {
                // Status register
                self.status = (value & 0xFF) as u8;
                Ok(())
            }
            2 => {
                // Control register
                self.control = (value & 0xFF) as u8;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn name(&self) -> &str {
        "UART"
    }

    fn size(&self) -> u64 {
        0x100 // 256 bytes
    }
}

/// Timer device
pub struct TimerDevice {
    counter: u64,
    reload_value: u64,
    control: u32,
}

impl TimerDevice {
    pub fn new() -> Self {
        Self {
            counter: 0,
            reload_value: 0,
            control: 0,
        }
    }

    pub fn tick(&mut self) {
        if self.control & 0x1 != 0 { // Timer enabled
            if self.counter > 0 {
                self.counter -= 1;
            } else {
                self.counter = self.reload_value;
                // Trigger interrupt would go here
            }
        }
    }
}

impl MmioDevice for TimerDevice {
    fn read(&self, offset: u64, size: u8) -> Result<u64> {
        match offset {
            0 => Ok(self.counter), // Counter register
            8 => Ok(self.reload_value), // Reload register
            16 => Ok(self.control as u64), // Control register
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
