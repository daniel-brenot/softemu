use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    pub fn execute_or(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid OR instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst | src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_out(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid OUT instruction".to_string()));
        }

        let port = self.get_operand_value(instruction, 0, state)? as u16;
        let value = self.get_operand_value(instruction, 1, state)?;
        // For now, ignore I/O operations (placeholder)
        log::debug!("OUT to port 0x{:x}: 0x{:x}", port, value);
        Ok(())
    }

    pub fn execute_outsb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Output byte from [RSI] to port DX
        let port = (state.registers.rdx & 0xFFFF) as u16;
        let src_addr = state.registers.rsi;
        
        let value = state.read_u8(src_addr)?;
        log::debug!("OUTSB to port 0x{:x}: 0x{:x}", port, value);
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(1);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(1);
        }
        Ok(())
    }

    pub fn execute_outsw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Output word from [RSI] to port DX
        let port = (state.registers.rdx & 0xFFFF) as u16;
        let src_addr = state.registers.rsi;
        
        let value = state.read_u16(src_addr)?;
        log::debug!("OUTSW to port 0x{:x}: 0x{:x}", port, value);
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(2);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(2);
        }
        Ok(())
    }

    pub fn execute_outsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Output doubleword from [RSI] to port DX
        let port = (state.registers.rdx & 0xFFFF) as u16;
        let src_addr = state.registers.rsi;
        
        let value = state.read_u32(src_addr)?;
        log::debug!("OUTSD to port 0x{:x}: 0x{:x}", port, value);
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(4);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(4);
        }
        Ok(())
    }
}