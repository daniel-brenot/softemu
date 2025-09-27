use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    pub fn execute_ret(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Pop return address
        let return_addr = state.read_u64(state.registers.rsp)?;
        state.registers.rsp += 8;
        
        // Add immediate operand if present (for RET imm16)
        if instruction.op_count() == 1 {
            let immediate = self.get_operand_value(instruction, 0, state)?;
            state.registers.rsp += immediate;
        }
        
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

    pub fn execute_rcpps(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RCPPS instruction executed");
        Ok(())
    }

    pub fn execute_rcpss(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RCPSS instruction executed");
        Ok(())
    }

    pub fn execute_rdfsbase(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RDFSBASE instruction executed");
        Ok(())
    }

    pub fn execute_rdgsbase(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RDGSBASE instruction executed");
        Ok(())
    }

    pub fn execute_rdpid(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RDPID instruction executed");
        Ok(())
    }

    pub fn execute_rdpkru(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RDPKRU instruction executed");
        Ok(())
    }

    pub fn execute_rdpru(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RDPRU instruction executed");
        Ok(())
    }

    pub fn execute_rdsspd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RDSSPD instruction executed");
        Ok(())
    }

    pub fn execute_rdsspq(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RDSSPQ instruction executed");
        Ok(())
    }

    pub fn execute_reservednop(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RESERVEDNOP instruction executed");
        Ok(())
    }

    pub fn execute_retf(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RETF instruction executed");
        Ok(())
    }

    pub fn execute_rorx(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RORX instruction executed");
        Ok(())
    }

    pub fn execute_roundpd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("ROUNDPD instruction executed");
        Ok(())
    }

    pub fn execute_roundps(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("ROUNDPS instruction executed");
        Ok(())
    }

    pub fn execute_roundsd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("ROUNDSD instruction executed");
        Ok(())
    }

    pub fn execute_roundss(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("ROUNDSS instruction executed");
        Ok(())
    }

    pub fn execute_rsqrtps(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RSQRTPS instruction executed");
        Ok(())
    }

    pub fn execute_rsqrtss(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RSQRTSS instruction executed");
        Ok(())
    }

    pub fn execute_rstorssp(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RSTORSSP instruction executed");
        Ok(())
    }

    pub fn execute_rmpadjust(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RMPADJUST instruction executed");
        Ok(())
    }

    pub fn execute_rmpupdate(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RMPUPDATE instruction executed");
        Ok(())
    }

    pub fn execute_rdshr(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RDSHR instruction executed");
        Ok(())
    }

    pub fn execute_rdm(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RDM instruction executed");
        Ok(())
    }

    pub fn execute_rsdc(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RSDC instruction executed");
        Ok(())
    }

    pub fn execute_rsldt(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RSLDT instruction executed");
        Ok(())
    }

    pub fn execute_rsts(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RSTS instruction executed");
        Ok(())
    }

    pub fn execute_rdudbg(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RDUDBG instruction executed");
        Ok(())
    }

    pub fn execute_rdmsrlist(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RDMSRLIST instruction executed");
        Ok(())
    }

    pub fn execute_rmpquery(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("RMPQUERY instruction executed");
        Ok(())
    }

    pub fn execute_rol(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ROL instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let count = self.get_operand_value(instruction, 1, state)? & 0x3F; // Mask to 6 bits
        
        if count == 0 {
            // No rotation, just update flags
            self.update_rotate_flags(src, src, 0, state);
            return Ok(());
        }
        
        let result = src.rotate_left(count as u32);
        
        // Set carry flag to the last bit rotated out
        let operand_size = self.get_operand_size(instruction, 0);
        let last_bit = (src >> (operand_size as u64 - count)) & 1;
        state.registers.set_flag(RFlags::CARRY, last_bit != 0);
        
        self.set_operand_value(instruction, 0, result, state)?;
        self.update_rotate_flags(result, src, count, state);
        Ok(())
    }

    pub fn execute_ror(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ROR instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let count = self.get_operand_value(instruction, 1, state)? & 0x3F; // Mask to 6 bits
        
        if count == 0 {
            // No rotation, just update flags
            self.update_rotate_flags(src, src, 0, state);
            return Ok(());
        }
        
        let result = src.rotate_right(count as u32);
        
        // Set carry flag to the last bit rotated out
        let last_bit = (src >> (count - 1)) & 1;
        state.registers.set_flag(RFlags::CARRY, last_bit != 0);
        
        self.set_operand_value(instruction, 0, result, state)?;
        self.update_rotate_flags(result, src, count, state);
        Ok(())
    }

    pub fn execute_rcl(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid RCL instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let count = self.get_operand_value(instruction, 1, state)? & 0x3F; // Mask to 6 bits
        
        if count == 0 {
            // No rotation, just update flags
            self.update_rotate_flags(src, src, 0, state);
            return Ok(());
        }
        
        let carry = if state.registers.get_flag(RFlags::CARRY) { 1 } else { 0 };
        let operand_size = self.get_operand_size(instruction, 0);
        let mask = match operand_size {
            8 => 0xFF,
            16 => 0xFFFF,
            32 => 0xFFFFFFFF,
            64 => 0xFFFFFFFFFFFFFFFF,
            _ => 0xFFFFFFFFFFFFFFFF,
        };
        
        // Rotate left through carry
        let result = ((src << count) | (carry << (count - 1))) & mask;
        
        // Set carry flag to the last bit rotated out
        let last_bit = (src >> (operand_size as u64 - count)) & 1;
        state.registers.set_flag(RFlags::CARRY, last_bit != 0);
        
        self.set_operand_value(instruction, 0, result, state)?;
        self.update_rotate_flags(result, src, count, state);
        Ok(())
    }

    pub fn execute_rcr(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid RCR instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let count = self.get_operand_value(instruction, 1, state)? & 0x3F; // Mask to 6 bits
        
        if count == 0 {
            // No rotation, just update flags
            self.update_rotate_flags(src, src, 0, state);
            return Ok(());
        }
        
        let carry = if state.registers.get_flag(RFlags::CARRY) { 1 } else { 0 };
        let operand_size = self.get_operand_size(instruction, 0);
        let mask = match operand_size {
            8 => 0xFF,
            16 => 0xFFFF,
            32 => 0xFFFFFFFF,
            64 => 0xFFFFFFFFFFFFFFFF,
            _ => 0xFFFFFFFFFFFFFFFF,
        };
        
        // Rotate right through carry
        let result = ((src >> count) | (carry << (operand_size as u64 - count))) & mask;
        
        // Set carry flag to the last bit rotated out
        let last_bit = (src >> (count - 1)) & 1;
        state.registers.set_flag(RFlags::CARRY, last_bit != 0);
        
        self.set_operand_value(instruction, 0, result, state)?;
        self.update_rotate_flags(result, src, count, state);
        Ok(())
    }
}
