use crate::{cpu::{fault::Fault, registers::CpuRegisters}, memory::MemoryManager};
use uluru::LRUCache;

/// Gate types for interrupt descriptor table entries
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateType {
    /// Task gate
    TaskGate = 0x5,
    /// Interrupt gate (16-bit)
    InterruptGate16 = 0x6,
    /// Trap gate (16-bit)
    TrapGate16 = 0x7,
    /// Interrupt gate (32-bit)
    InterruptGate32 = 0xE,
    /// Trap gate (32-bit)
    TrapGate32 = 0xF,
    // If value is none of the above, it is invalid
    Invalid = 0xFF,
}

/// Decoded type_attr field from interrupt descriptor table entry
#[derive(Debug, Clone, Copy)]
pub struct TypeAttr {
    /// Present bit (bit 7)
    pub present: bool,
    /// Descriptor Privilege Level (bits 5-6)
    pub dpl: u8,
    /// Gate type (bits 0-3)
    pub gate_type: GateType,
}

impl TypeAttr {
    /// Decode the 8-bit type_attr field
    pub fn decode(type_attr: u8) -> Self {
        let present = (type_attr & 0x80) != 0;
        let dpl = (type_attr & 0x60) >> 5;
        let gate_type_raw = type_attr & 0x0F;
        
        let gate_type = match gate_type_raw {
            0x5 => GateType::TaskGate,
            0x6 => GateType::InterruptGate16,
            0x7 => GateType::TrapGate16,
            0xE => GateType::InterruptGate32,
            0xF => GateType::TrapGate32,
            _ => GateType::Invalid
        };
        
        Self {
            present,
            dpl,
            gate_type,
        }
    }
}

/// Cached mappiinf of virtual to physical addresses
#[derive(Debug)]
pub struct TlbEntry {
    pub virtual_addr: u64,
    pub physical_addr: u64,
    pub flags: u64,
}

/// CPU execution state
#[derive(Debug)]
pub struct CpuState {
    /// Core ID
    pub core_id: u32,
    /// CPU registers
    pub registers: CpuRegisters,
    /// TLB for data access
    pub data_tlb: LRUCache<TlbEntry, 64>,
    /// TLB for instruction access
    pub instruction_tlb: LRUCache<TlbEntry, 128>,
    /// TLB for shared access
    pub shared_tlb: LRUCache<TlbEntry, 2048>,
}

impl CpuState {
    pub fn new(core_id: u32) -> Self {
        Self {
            core_id,
            registers: CpuRegisters::new(),
            data_tlb: LRUCache::new(),
            instruction_tlb: LRUCache::new(),
            shared_tlb: LRUCache::new(),
        }
    }

    /// Handle an interrupt
    pub fn handle_interrupt(&mut self, memory: &MemoryManager, fault: u8) -> Result<(), Fault> {
        if fault * 8 > self.registers.idtr.limit as u8 {
            self.handle_interrupt(memory, Fault::GeneralProtection as u8);
        }
        // Save current state to stack before jumping to interrupt handler
        memory.write_u64(self.registers.rsp, self.registers.rip)?;
        self.registers.rsp -= 8;
        
        memory.write_u64(self.registers.rsp, self.registers.cs as u64)?;
        self.registers.rsp -= 8;
        
        memory.write_u64(self.registers.rsp, self.registers.rflags)?;
        self.registers.rsp -= 8;
        
        // Set interrupt flag
        self.registers.rflags |= 0x200;
        
        // Use the fault number to point to the entry in the IDT
        let fault_vector_address = self.registers.idtr.base + (fault as u64 * 16);

        // Get each of the values from the IDT entry
        let offset_low = memory.read_u16(fault_vector_address)?;
        let selector = memory.read_u16(fault_vector_address + 2)?;
        let ist = memory.read_u8(fault_vector_address + 4)?;
        let type_attr_raw = memory.read_u8(fault_vector_address + 5)?;
        let offset_mid = memory.read_u16(fault_vector_address + 6)?;
        let offset_high = memory.read_u32(fault_vector_address + 8)?;
        // Zero is reserved but not used
        // let zero = memory.read_u32(fault_vector_address + 12)?;

        // Decode the type_attr field
        let type_attr = TypeAttr::decode(type_attr_raw);

        // Calculate the fault vector address
        let fault_vector_address = (offset_high as u64) << 32 | (offset_mid as u64) << 16 | offset_low as u64;

        // Set the interrupt gate
        


        Ok(())
    }

    fn handle_real_mode_interrupt(&mut self, memory: &MemoryManager, fault: u8) {
        // TODO: Implement real mode interrupt handler
        unimplemented!();
    }

    pub fn is_virtual8086_mode(&self) -> bool {
        self.registers.rflags & 0x100 != 0
    }

    pub fn is_x16_protected_mode(&self) -> bool {
        self.registers.cs == 0x08
    }

    pub fn is_x16_real_mode(&self) -> bool {
        self.registers.cs == 0x00
    }

    pub fn is_x32_mode(&self) -> bool {
        self.registers.cs == 0x10
    }

    pub fn is_x64_mode(&self) -> bool {
        self.registers.msr_efer & 0x80000000 != 0
    }

    /// Get the core ID
    pub fn core_id(&self) -> u32 {
        self.core_id
    }
}
