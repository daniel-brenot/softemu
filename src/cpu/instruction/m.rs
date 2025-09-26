use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {

    pub fn execute_mov(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid MOV instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        self.set_operand_value(instruction, 1, src, state)?;
        Ok(())
    }

    pub fn execute_movsb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Move byte from [RSI] to [RDI]
        let src_addr = state.registers.rsi;
        let dst_addr = state.registers.rdi;
        
        let value = state.read_u8(src_addr)?;
        state.write_u8(dst_addr, value)?;
        
        // Update pointers based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(1);
            state.registers.rdi = state.registers.rdi.wrapping_sub(1);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(1);
            state.registers.rdi = state.registers.rdi.wrapping_add(1);
        }
        Ok(())
    }

    pub fn execute_movsw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Move word from [RSI] to [RDI]
        let src_addr = state.registers.rsi;
        let dst_addr = state.registers.rdi;
        
        let value = state.read_u16(src_addr)?;
        state.write_u16(dst_addr, value)?;
        
        // Update pointers based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(2);
            state.registers.rdi = state.registers.rdi.wrapping_sub(2);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(2);
            state.registers.rdi = state.registers.rdi.wrapping_add(2);
        }
        Ok(())
    }

    pub fn execute_movsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Move doubleword from [RSI] to [RDI]
        let src_addr = state.registers.rsi;
        let dst_addr = state.registers.rdi;
        
        let value = state.read_u32(src_addr)?;
        state.write_u32(dst_addr, value)?;
        
        // Update pointers based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(4);
            state.registers.rdi = state.registers.rdi.wrapping_sub(4);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(4);
            state.registers.rdi = state.registers.rdi.wrapping_add(4);
        }
        Ok(())
    }

    pub fn execute_movsq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Move quadword from [RSI] to [RDI]
        let src_addr = state.registers.rsi;
        let dst_addr = state.registers.rdi;
        
        let value = state.read_u64(src_addr)?;
        state.write_u64(dst_addr, value)?;
        
        // Update pointers based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(8);
            state.registers.rdi = state.registers.rdi.wrapping_sub(8);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(8);
            state.registers.rdi = state.registers.rdi.wrapping_add(8);
        }
        Ok(())
    }

    pub fn execute_movsx(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid MOVSX instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        // Sign extend based on source size (simplified - assume 32-bit to 64-bit)
        let result = src as i32 as i64 as u64;
        self.set_operand_value(instruction, 1, result, state)?;
        Ok(())
    }

    pub fn execute_movzx(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid MOVZX instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        // Zero extend (no change needed for 64-bit)
        self.set_operand_value(instruction, 1, src, state)?;
        Ok(())
    }

    pub fn execute_mul(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid MUL instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let rax = state.registers.rax;
        let result = rax.wrapping_mul(src);
        
        // Store result in RAX:RDX (64-bit result in RAX, overflow in RDX)
        state.registers.rax = result;
        state.registers.rdx = if result < rax { 1 } else { 0 };
        
        // Update flags
        state.registers.set_flag(RFlags::CARRY, state.registers.rdx != 0);
        state.registers.set_flag(RFlags::OVERFLOW, state.registers.rdx != 0);
        Ok(())
    }

    pub fn execute_mfence(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Memory Fence (memory ordering)
        // For now, just do nothing
        Ok(())
    }
}