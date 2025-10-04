use crate::cpu::registers::CpuRegisters;

/// CPU execution state
#[derive(Debug)]
pub struct CpuState {
    pub registers: CpuRegisters,
    pub running: bool,
    pub halted: bool,
    pub interrupt_pending: bool,
    pub interrupt_vector: u8,
    /// 0 = kernel, 3 = user
    pub privilege_level: u8, 
    pub paging_enabled: bool,
}

impl CpuState {
    pub fn new() -> Self {
        Self {
            registers: CpuRegisters::new(),
            running: false,
            halted: false,
            interrupt_pending: false,
            interrupt_vector: 0,
            // Start in kernel mode
            privilege_level: 0,
            paging_enabled: true,
        }
    }

    pub fn start(&mut self) {
        self.running = true;
        self.halted = false;
    }

    pub fn halt(&mut self) {
        self.running = false;
        self.halted = true;
    }

    pub fn trigger_interrupt(&mut self, vector: u8) {
        self.interrupt_pending = true;
        self.interrupt_vector = vector;
    }

    pub fn clear_interrupt(&mut self) {
        self.interrupt_pending = false;
        self.interrupt_vector = 0;
    }

    pub fn set_privilege_level(&mut self, level: u8) {
        self.privilege_level = level;
    }

    pub fn is_kernel_mode(&self) -> bool {
        self.privilege_level == 0
    }

    pub fn is_user_mode(&self) -> bool {
        self.privilege_level == 3
    }
}
