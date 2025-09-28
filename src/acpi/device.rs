use super::{AcpiPowerState, AcpiDeviceState, AcpiEvent, AcpiResult};

/// ACPI device types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpiDeviceType {
    Processor,
    Memory,
    Io,
    Pci,
    Thermal,
    Battery,
    Fan,
    Button,
    Lid,
    Generic,
}

/// ACPI device information
#[derive(Debug, Clone)]
pub struct AcpiDeviceInfo {
    pub name: String,
    pub device_type: AcpiDeviceType,
    pub hid: String,        // Hardware ID
    pub cid: Vec<String>,   // Compatible IDs
    pub uid: String,        // Unique ID
    pub path: String,       // ACPI path
    pub power_state: AcpiPowerState,
    pub device_state: AcpiDeviceState,
}

/// ACPI device operations
pub trait AcpiDeviceOps {
    /// Get device information
    fn get_info(&self) -> &AcpiDeviceInfo;
    
    /// Set power state
    fn set_power_state(&mut self, state: AcpiPowerState) -> AcpiResult<()>;
    
    /// Get power state
    fn get_power_state(&self) -> AcpiPowerState;
    
    /// Set device state
    fn set_device_state(&mut self, state: AcpiDeviceState) -> AcpiResult<()>;
    
    /// Get device state
    fn get_device_state(&self) -> AcpiDeviceState;
    
    /// Handle ACPI event
    fn handle_event(&mut self, event: AcpiEvent) -> AcpiResult<()>;
    
    /// Read from device
    fn read(&self, offset: u64, size: u8) -> AcpiResult<u64>;
    
    /// Write to device
    fn write(&mut self, offset: u64, value: u64, size: u8) -> AcpiResult<()>;
    
    /// Initialize device
    fn initialize(&mut self) -> AcpiResult<()>;
    
    /// Shutdown device
    fn shutdown(&mut self) -> AcpiResult<()>;
}

/// Generic ACPI device implementation
pub struct AcpiDevice {
    info: AcpiDeviceInfo,
    data: Vec<u8>,
    event_handlers: Vec<Box<dyn Fn(AcpiEvent) -> AcpiResult<()> + Send + Sync>>,
}

impl AcpiDevice {
    pub fn new(
        name: String,
        device_type: AcpiDeviceType,
        hid: String,
        path: String,
    ) -> Self {
        Self {
            info: AcpiDeviceInfo {
                name,
                device_type,
                hid,
                cid: Vec::new(),
                uid: format!("{:x}", 0x12345678), // Fixed UID for now
                path,
                power_state: AcpiPowerState::S0,
                device_state: AcpiDeviceState::D0,
            },
            data: vec![0; 1024], // 1KB device data
            event_handlers: Vec::new(),
        }
    }

    pub fn add_compatible_id(&mut self, cid: String) {
        self.info.cid.push(cid);
    }

    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(AcpiEvent) -> AcpiResult<()> + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }
}

impl AcpiDeviceOps for AcpiDevice {
    fn get_info(&self) -> &AcpiDeviceInfo {
        &self.info
    }

    fn set_power_state(&mut self, state: AcpiPowerState) -> AcpiResult<()> {
        self.info.power_state = state;
        Ok(())
    }

    fn get_power_state(&self) -> AcpiPowerState {
        self.info.power_state
    }

    fn set_device_state(&mut self, state: AcpiDeviceState) -> AcpiResult<()> {
        self.info.device_state = state;
        Ok(())
    }

    fn get_device_state(&self) -> AcpiDeviceState {
        self.info.device_state
    }

    fn handle_event(&mut self, event: AcpiEvent) -> AcpiResult<()> {
        for handler in &self.event_handlers {
            handler(event)?;
        }
        Ok(())
    }

    fn read(&self, offset: u64, size: u8) -> AcpiResult<u64> {
        if offset as usize + size as usize > self.data.len() {
            return Err(super::AcpiError::DeviceError("Read out of bounds".to_string()));
        }

        let mut value = 0u64;
        for i in 0..size {
            let byte_offset = (offset + i as u64) as usize;
            if byte_offset < self.data.len() {
                value |= (self.data[byte_offset] as u64) << (i * 8);
            }
        }
        Ok(value)
    }

    fn write(&mut self, offset: u64, value: u64, size: u8) -> AcpiResult<()> {
        if offset as usize + size as usize > self.data.len() {
            return Err(super::AcpiError::DeviceError("Write out of bounds".to_string()));
        }

        for i in 0..size {
            let byte_offset = (offset + i as u64) as usize;
            if byte_offset < self.data.len() {
                self.data[byte_offset] = ((value >> (i * 8)) & 0xFF) as u8;
            }
        }
        Ok(())
    }

    fn initialize(&mut self) -> AcpiResult<()> {
        // Initialize device-specific data
        self.data.fill(0);
        Ok(())
    }

    fn shutdown(&mut self) -> AcpiResult<()> {
        // Cleanup device-specific resources
        self.data.fill(0);
        Ok(())
    }
}

/// ACPI processor device
pub struct AcpiProcessor {
    device: AcpiDevice,
    processor_id: u8,
    apic_id: u8,
    cpu_state: super::AcpiProcessorState,
}

impl AcpiProcessor {
    pub fn new(processor_id: u8, apic_id: u8) -> Self {
        let mut device = AcpiDevice::new(
            format!("CPU{}", processor_id),
            AcpiDeviceType::Processor,
            "ACPI0007".to_string(), // Processor device HID
            format!("\\_SB.CPU{}", processor_id),
        );
        
        Self {
            device,
            processor_id,
            apic_id,
            cpu_state: super::AcpiProcessorState::C0,
        }
    }

    pub fn get_processor_id(&self) -> u8 {
        self.processor_id
    }

    pub fn get_apic_id(&self) -> u8 {
        self.apic_id
    }

    pub fn get_cpu_state(&self) -> super::AcpiProcessorState {
        self.cpu_state
    }

    pub fn set_cpu_state(&mut self, state: super::AcpiProcessorState) -> AcpiResult<()> {
        self.cpu_state = state;
        Ok(())
    }
}

impl AcpiDeviceOps for AcpiProcessor {
    fn get_info(&self) -> &AcpiDeviceInfo {
        self.device.get_info()
    }

    fn set_power_state(&mut self, state: AcpiPowerState) -> AcpiResult<()> {
        self.device.set_power_state(state)
    }

    fn get_power_state(&self) -> AcpiPowerState {
        self.device.get_power_state()
    }

    fn set_device_state(&mut self, state: AcpiDeviceState) -> AcpiResult<()> {
        self.device.set_device_state(state)
    }

    fn get_device_state(&self) -> AcpiDeviceState {
        self.device.get_device_state()
    }

    fn handle_event(&mut self, event: AcpiEvent) -> AcpiResult<()> {
        match event {
            AcpiEvent::Processor => {
                // Handle processor-specific events
                Ok(())
            }
            _ => self.device.handle_event(event),
        }
    }

    fn read(&self, offset: u64, size: u8) -> AcpiResult<u64> {
        self.device.read(offset, size)
    }

    fn write(&mut self, offset: u64, value: u64, size: u8) -> AcpiResult<()> {
        self.device.write(offset, value, size)
    }

    fn initialize(&mut self) -> AcpiResult<()> {
        self.device.initialize()
    }

    fn shutdown(&mut self) -> AcpiResult<()> {
        self.device.shutdown()
    }
}

/// ACPI thermal device
pub struct AcpiThermal {
    device: AcpiDevice,
    temperature: f32,
    critical_temperature: f32,
    passive_temperature: f32,
}

impl AcpiThermal {
    pub fn new() -> Self {
        let device = AcpiDevice::new(
            "THERMAL".to_string(),
            AcpiDeviceType::Thermal,
            "ACPI000C".to_string(), // Thermal zone device HID
            "\\_TZ.THM0".to_string(),
        );

        Self {
            device,
            temperature: 25.0, // 25°C
            critical_temperature: 85.0, // 85°C
            passive_temperature: 75.0, // 75°C
        }
    }

    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }

    pub fn set_temperature(&mut self, temp: f32) -> AcpiResult<()> {
        self.temperature = temp;
        
        // Check for thermal events
        if temp >= self.critical_temperature {
            self.device.handle_event(AcpiEvent::Thermal)?;
        }
        
        Ok(())
    }

    pub fn get_critical_temperature(&self) -> f32 {
        self.critical_temperature
    }

    pub fn get_passive_temperature(&self) -> f32 {
        self.passive_temperature
    }
}

impl AcpiDeviceOps for AcpiThermal {
    fn get_info(&self) -> &AcpiDeviceInfo {
        self.device.get_info()
    }

    fn set_power_state(&mut self, state: AcpiPowerState) -> AcpiResult<()> {
        self.device.set_power_state(state)
    }

    fn get_power_state(&self) -> AcpiPowerState {
        self.device.get_power_state()
    }

    fn set_device_state(&mut self, state: AcpiDeviceState) -> AcpiResult<()> {
        self.device.set_device_state(state)
    }

    fn get_device_state(&self) -> AcpiDeviceState {
        self.device.get_device_state()
    }

    fn handle_event(&mut self, event: AcpiEvent) -> AcpiResult<()> {
        self.device.handle_event(event)
    }

    fn read(&self, offset: u64, size: u8) -> AcpiResult<u64> {
        match offset {
            0x00 => Ok(self.temperature.to_bits() as u64),
            0x04 => Ok(self.critical_temperature.to_bits() as u64),
            0x08 => Ok(self.passive_temperature.to_bits() as u64),
            _ => self.device.read(offset, size),
        }
    }

    fn write(&mut self, offset: u64, value: u64, size: u8) -> AcpiResult<()> {
        match offset {
            0x00 => {
                self.temperature = f32::from_bits(value as u32);
                Ok(())
            }
            0x04 => {
                self.critical_temperature = f32::from_bits(value as u32);
                Ok(())
            }
            0x08 => {
                self.passive_temperature = f32::from_bits(value as u32);
                Ok(())
            }
            _ => self.device.write(offset, value, size),
        }
    }

    fn initialize(&mut self) -> AcpiResult<()> {
        self.device.initialize()
    }

    fn shutdown(&mut self) -> AcpiResult<()> {
        self.device.shutdown()
    }
}

/// ACPI button device
pub struct AcpiButton {
    device: AcpiDevice,
    button_type: ButtonType,
    pressed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonType {
    Power,
    Sleep,
    Lid,
}

impl AcpiButton {
    pub fn new(button_type: ButtonType) -> Self {
        let (name, hid, path) = match button_type {
            ButtonType::Power => (
                "PWRB".to_string(),
                "PNP0C0C".to_string(), // Power button HID
                "\\_SB.PWRB".to_string(),
            ),
            ButtonType::Sleep => (
                "SLPB".to_string(),
                "PNP0C0E".to_string(), // Sleep button HID
                "\\_SB.SLPB".to_string(),
            ),
            ButtonType::Lid => (
                "LID0".to_string(),
                "PNP0C0D".to_string(), // Lid switch HID
                "\\_SB.LID0".to_string(),
            ),
        };

        let device = AcpiDevice::new(
            name,
            AcpiDeviceType::Button,
            hid,
            path,
        );

        Self {
            device,
            button_type,
            pressed: false,
        }
    }

    pub fn get_button_type(&self) -> ButtonType {
        self.button_type
    }

    pub fn is_pressed(&self) -> bool {
        self.pressed
    }

    pub fn press(&mut self) -> AcpiResult<()> {
        self.pressed = true;
        let event = match self.button_type {
            ButtonType::Power => AcpiEvent::PowerButton,
            ButtonType::Sleep => AcpiEvent::SleepButton,
            ButtonType::Lid => AcpiEvent::LidSwitch,
        };
        self.device.handle_event(event)
    }

    pub fn release(&mut self) -> AcpiResult<()> {
        self.pressed = false;
        Ok(())
    }
}

impl AcpiDeviceOps for AcpiButton {
    fn get_info(&self) -> &AcpiDeviceInfo {
        self.device.get_info()
    }

    fn set_power_state(&mut self, state: AcpiPowerState) -> AcpiResult<()> {
        self.device.set_power_state(state)
    }

    fn get_power_state(&self) -> AcpiPowerState {
        self.device.get_power_state()
    }

    fn set_device_state(&mut self, state: AcpiDeviceState) -> AcpiResult<()> {
        self.device.set_device_state(state)
    }

    fn get_device_state(&self) -> AcpiDeviceState {
        self.device.get_device_state()
    }

    fn handle_event(&mut self, event: AcpiEvent) -> AcpiResult<()> {
        self.device.handle_event(event)
    }

    fn read(&self, offset: u64, size: u8) -> AcpiResult<u64> {
        match offset {
            0x00 => Ok(self.pressed as u64),
            _ => self.device.read(offset, size),
        }
    }

    fn write(&mut self, offset: u64, value: u64, size: u8) -> AcpiResult<()> {
        match offset {
            0x00 => {
                self.pressed = value != 0;
                Ok(())
            }
            _ => self.device.write(offset, value, size),
        }
    }

    fn initialize(&mut self) -> AcpiResult<()> {
        self.device.initialize()
    }

    fn shutdown(&mut self) -> AcpiResult<()> {
        self.device.shutdown()
    }
}
