pub mod cpu;
pub mod devices;
pub mod memory;
pub mod network;
pub mod vm;

pub use vm::VirtualMachine;

/// Error types for the emulator
#[derive(thiserror::Error, Debug)]
pub enum EmulatorError {
    #[error("CPU error: {0}")]
    Cpu(String),
    
    #[error("Memory error: {0}")]
    Memory(String),
    
    #[error("Device error: {0}")]
    Device(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("VM memory error: {0}")]
    VmMemory(#[from] vm_memory::GuestMemoryError),
    
    #[error("Linux loader error: {0}")]
    LinuxLoader(#[from] linux_loader::loader::Error),
    
    #[error("Memfd error: {0}")]
    Memfd(String),
    
    #[error("VM memory error: {0}")]
    VmMemoryError(#[from] vm_memory::Error),
}

pub type Result<T> = std::result::Result<T, EmulatorError>;
