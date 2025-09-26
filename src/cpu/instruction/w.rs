use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    pub fn execute_wait(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // WAIT instruction - wait for floating point operations
        // For now, just do nothing (no floating point unit)
        Ok(())
    }

    pub fn execute_wbinvd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Write Back and Invalidate Cache (simplified - just log for now)
        log::debug!("WBINVD instruction executed");
        Ok(())
    }

    pub fn execute_wrmsr(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 0 {
            return Err(crate::EmulatorError::Cpu("Invalid WRMSR instruction".to_string()));
        }

        let ecx = (state.registers.rcx & 0xFFFFFFFF) as u32;
        let eax = (state.registers.rax & 0xFFFFFFFF) as u32;
        let edx = (state.registers.rdx & 0xFFFFFFFF) as u32;
        
        match ecx {
            0x1B => {
                // IA32_APIC_BASE
                log::debug!("WRMSR: IA32_APIC_BASE = 0x{:08x}{:08x}", edx, eax);
            }
            _ => log::debug!("WRMSR: Unsupported MSR 0x{:08x} = 0x{:08x}{:08x}", ecx, edx, eax),
        }
        Ok(())
    }
}