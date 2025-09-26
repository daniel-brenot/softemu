use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    pub fn execute_loop(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LOOP instruction".to_string()));
        }

        // Decrement RCX (counter register)
        state.registers.rcx = state.registers.rcx.wrapping_sub(1);
        
        // If RCX != 0, jump to target
        if state.registers.rcx != 0 {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_loope(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LOOPE instruction".to_string()));
        }

        // Decrement RCX (counter register)
        state.registers.rcx = state.registers.rcx.wrapping_sub(1);
        
        // If RCX != 0 AND zero flag is set, jump to target
        if state.registers.rcx != 0 && state.registers.get_flag(RFlags::ZERO) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_loopne(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LOOPNE instruction".to_string()));
        }

        // Decrement RCX (counter register)
        state.registers.rcx = state.registers.rcx.wrapping_sub(1);
        
        // If RCX != 0 AND zero flag is clear, jump to target
        if state.registers.rcx != 0 && !state.registers.get_flag(RFlags::ZERO) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_lodsb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Load byte from [RSI] into AL
        let src_addr = state.registers.rsi;
        
        let value = state.read_u8(src_addr)?;
        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFFFF00) | (value as u64);
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(1);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(1);
        }
        Ok(())
    }

    pub fn execute_lodsw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Load word from [RSI] into AX
        let src_addr = state.registers.rsi;
        
        let value = state.read_u16(src_addr)?;
        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFF0000) | (value as u64);
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(2);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(2);
        }
        Ok(())
    }

    pub fn execute_lodsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Load doubleword from [RSI] into EAX
        let src_addr = state.registers.rsi;
        
        let value = state.read_u32(src_addr)?;
        state.registers.rax = (state.registers.rax & 0xFFFFFFFF00000000) | (value as u64);
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(4);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(4);
        }
        Ok(())
    }

    pub fn execute_lodsq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Load quadword from [RSI] into RAX
        let src_addr = state.registers.rsi;
        
        let value = state.read_u64(src_addr)?;
        state.registers.rax = value;
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(8);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(8);
        }
        Ok(())
    }

    pub fn execute_lgdt(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LGDT instruction".to_string()));
        }
        // Load Global Descriptor Table (simplified - just log for now)
        log::debug!("LGDT instruction executed");
        Ok(())
    }

    pub fn execute_lidt(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LIDT instruction".to_string()));
        }
        // Load Interrupt Descriptor Table (simplified - just log for now)
        log::debug!("LIDT instruction executed");
        Ok(())
    }

    pub fn execute_ltr(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LTR instruction".to_string()));
        }
        // Load Task Register (simplified - just log for now)
        log::debug!("LTR instruction executed");
        Ok(())
    }

    pub fn execute_lmsw(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LMSW instruction".to_string()));
        }
        // Load Machine Status Word (simplified - just log for now)
        log::debug!("LMSW instruction executed");
        Ok(())
    }

    pub fn execute_lahf(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Load flags into AH register
        let flags = state.registers.get_flags();
        let ah_value = (flags.bits() & 0xFF) as u8;
        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFFFF00) | (ah_value as u64);
        Ok(())
    }

    pub fn execute_lds(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LDS instruction".to_string()));
        }
        // Load DS segment and offset (simplified - just log for now)
        log::debug!("LDS instruction executed");
        Ok(())
    }

    pub fn execute_les(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LES instruction".to_string()));
        }
        // Load ES segment and offset (simplified - just log for now)
        log::debug!("LES instruction executed");
        Ok(())
    }

    pub fn execute_lfs(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LFS instruction".to_string()));
        }
        // Load FS segment and offset (simplified - just log for now)
        log::debug!("LFS instruction executed");
        Ok(())
    }

    pub fn execute_lgs(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LGS instruction".to_string()));
        }
        // Load GS segment and offset (simplified - just log for now)
        log::debug!("LGS instruction executed");
        Ok(())
    }

    pub fn execute_lss(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LSS instruction".to_string()));
        }
        // Load SS segment and offset (simplified - just log for now)
        log::debug!("LSS instruction executed");
        Ok(())
    }

    pub fn execute_lea(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LEA instruction".to_string()));
        }

        // LEA loads the effective address of the source operand into the destination
        // For now, we'll use the source operand value as the address
        let src = self.get_operand_value(instruction, 0, state)?;
        self.set_operand_value(instruction, 1, src, state)?;
        Ok(())
    }

    pub fn execute_leave(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Leave procedure (opposite of ENTER)
        state.registers.rsp = state.registers.rbp;
        state.registers.rbp = state.read_u64(state.registers.rsp)?;
        state.registers.rsp += 8;
        Ok(())
    }

    pub fn execute_lfence(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Load Fence (memory ordering)
        // For now, just do nothing
        Ok(())
    }

    pub fn execute_lldt(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LLDT instruction".to_string()));
        }
        // Load Local Descriptor Table (simplified - just log for now)
        log::debug!("LLDT instruction executed");
        Ok(())
    }

    pub fn execute_lar(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LAR instruction".to_string()));
        }
        // Load Access Rights (simplified - just log for now)
        log::debug!("LAR instruction executed");
        Ok(())
    }

    pub fn execute_lsl(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LSL instruction".to_string()));
        }
        // Load Segment Limit (simplified - just log for now)
        log::debug!("LSL instruction executed");
        Ok(())
    }
}