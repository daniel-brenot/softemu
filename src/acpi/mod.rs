pub mod tables;
pub mod device;
pub mod power;
pub mod manager;

pub use manager::AcpiManager;
pub use tables::*;
pub use device::AcpiDevice;
pub use power::PowerState;


/// ACPI version constants
pub const ACPI_VERSION_1_0: u8 = 1;
pub const ACPI_VERSION_2_0: u8 = 2;
pub const ACPI_VERSION_6_0: u8 = 6;

/// ACPI table signatures
pub const RSDP_SIGNATURE: &[u8; 8] = b"RSD PTR ";
pub const RSDT_SIGNATURE: &[u8; 4] = b"RSDT";
pub const XSDT_SIGNATURE: &[u8; 4] = b"XSDT";
pub const FADT_SIGNATURE: &[u8; 4] = b"FACP";
pub const DSDT_SIGNATURE: &[u8; 4] = b"DSDT";
pub const SSDT_SIGNATURE: &[u8; 4] = b"SSDT";
pub const MADT_SIGNATURE: &[u8; 4] = b"APIC";
pub const MCFG_SIGNATURE: &[u8; 4] = b"MCFG";

/// ACPI table locations
pub const RSDP_ADDRESS: u64 = 0xE0000;  // EBDA area
pub const ACPI_TABLES_BASE: u64 = 0x100000;  // 1MB boundary

/// ACPI power states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpiPowerState {
    S0,  // Working
    S1,  // CPU stopped, RAM refresh
    S2,  // CPU off, RAM refresh
    S3,  // Suspend to RAM
    S4,  // Suspend to disk
    S5,  // Soft off
}

/// ACPI device states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpiDeviceState {
    D0,  // Fully on
    D1,  // Low power
    D2,  // Lower power
    D3,  // Off
}

/// ACPI processor states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpiProcessorState {
    C0,  // Running
    C1,  // Halt
    C2,  // Stop clock
    C3,  // Deep sleep
}

/// ACPI event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpiEvent {
    PowerButton,
    SleepButton,
    LidSwitch,
    Thermal,
    Battery,
    Processor,
    Device,
}

/// ACPI error types
#[derive(thiserror::Error, Debug)]
pub enum AcpiError {
    #[error("Invalid ACPI table signature")]
    InvalidSignature,
    
    #[error("ACPI table checksum error")]
    ChecksumError,
    
    #[error("ACPI table not found: {0}")]
    TableNotFound(String),
    
    #[error("ACPI device error: {0}")]
    DeviceError(String),
    
    #[error("ACPI power management error: {0}")]
    PowerError(String),
    
    #[error("ACPI memory allocation error")]
    MemoryError,
}

impl From<AcpiError> for crate::EmulatorError {
    fn from(err: AcpiError) -> Self {
        crate::EmulatorError::Device(err.to_string())
    }
}

/// ACPI result type
pub type AcpiResult<T> = std::result::Result<T, AcpiError>;
