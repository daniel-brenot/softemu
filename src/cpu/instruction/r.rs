use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    pub fn execute_ret(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Pop return address
        let return_addr = state.read_u64(state.registers.rsp)?;
        state.registers.rsp += 8;
        
        // Jump to return address
        state.registers.rip = return_addr;
        Ok(())
    }

    pub fn execute_rdmsr(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 0 {
            return Err(crate::EmulatorError::Cpu("Invalid RDMSR instruction".to_string()));
        }

        let ecx = (state.registers.rcx & 0xFFFFFFFF) as u32;
        match ecx {
            0x1B => {
                // IA32_APIC_BASE
                state.registers.rax = 0xFEE00000;
                state.registers.rdx = 0;
            }
            _ => {
                // Unsupported MSR
                state.registers.rax = 0;
                state.registers.rdx = 0;
            }
        }
        Ok(())
    }

    pub fn execute_rdpmc(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 0 {
            return Err(crate::EmulatorError::Cpu("Invalid RDPMC instruction".to_string()));
        }

        let ecx = (state.registers.rcx & 0xFFFFFFFF) as u32;
        // Return dummy performance counter values
        state.registers.rax = ecx as u64;
        state.registers.rdx = 0;
        Ok(())
    }

    pub fn execute_rdrand(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid RDRAND instruction".to_string()));
        }

        // Generate a pseudo-random number
        let random_value = (state.registers.rax ^ state.registers.rcx ^ state.registers.rdx) as u64;
        self.set_operand_value(instruction, 0, random_value, state)?;
        state.registers.set_flag(RFlags::CARRY, true); // Indicate success
        Ok(())
    }

    pub fn execute_rdseed(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid RDSEED instruction".to_string()));
        }

        // Generate a pseudo-random number (simplified)
        let random_value = (state.registers.rax ^ state.registers.rcx ^ state.registers.rdx) as u64;
        self.set_operand_value(instruction, 0, random_value, state)?;
        state.registers.set_flag(RFlags::CARRY, true); // Indicate success
        Ok(())
    }

    pub fn execute_rdtsc(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 0 {
            return Err(crate::EmulatorError::Cpu("Invalid RDTSC instruction".to_string()));
        }

        // Return dummy timestamp counter values
        state.registers.rax = 0x12345678;
        state.registers.rdx = 0x9ABCDEF0;
        Ok(())
    }

    pub fn execute_rdtscp(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 0 {
            return Err(crate::EmulatorError::Cpu("Invalid RDTSCP instruction".to_string()));
        }

        // Return dummy timestamp counter values
        state.registers.rax = 0x12345678;
        state.registers.rdx = 0x9ABCDEF0;
        state.registers.rcx = 0; // Processor ID
        Ok(())
    }

    pub fn execute_rsm(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Resume from System Management Mode (simplified - just log for now)
        log::debug!("RSM instruction executed");
        Ok(())
    }
}