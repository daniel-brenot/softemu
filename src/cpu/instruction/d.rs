use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    pub fn execute_dec(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid DEC instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let result = src.wrapping_sub(1);
        self.set_operand_value(instruction, 0, result, state)?;
        
        // Update flags (DEC doesn't affect carry flag)
        state.registers.set_flag(RFlags::ZERO, result == 0);
        state.registers.set_flag(RFlags::SIGN, (result & 0x8000000000000000) != 0);
        state.registers.set_flag(RFlags::OVERFLOW, result == 0x7FFFFFFFFFFFFFFF);
        
        let low_byte = (result & 0xFF) as u8;
        let parity = low_byte.count_ones() % 2 == 0;
        state.registers.set_flag(RFlags::PARITY, parity);
        Ok(())
    }

    pub fn execute_div(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid DIV instruction".to_string()));
        }

        let divisor = self.get_operand_value(instruction, 0, state)?;
        if divisor == 0 {
            return Err(crate::EmulatorError::Cpu("Division by zero".to_string()));
        }

        let dividend = (state.registers.rdx as u128) << 64 | state.registers.rax as u128;
        let quotient = dividend / (divisor as u128);
        let remainder = dividend % (divisor as u128);
        
        state.registers.rax = quotient as u64;
        state.registers.rdx = remainder as u64;
        Ok(())
    }

    pub fn execute_idiv(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid IDIV instruction".to_string()));
        }

        let divisor = self.get_operand_value(instruction, 0, state)? as i64;
        if divisor == 0 {
            return Err(crate::EmulatorError::Cpu("Division by zero".to_string()));
        }

        let dividend = ((state.registers.rdx as i128) << 64) | (state.registers.rax as i128);
        let quotient = dividend / (divisor as i128);
        let remainder = dividend % (divisor as i128);
        
        state.registers.rax = quotient as u64;
        state.registers.rdx = remainder as u64;
        Ok(())
    }

    pub fn execute_daa(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Decimal Adjust After Addition
        let al = (state.registers.rax & 0xFF) as u8;
        let mut result = al;
        let mut cf = false;

        if (al & 0x0F) > 9 || state.registers.get_flag(RFlags::AUXILIARY) {
            result += 6;
            state.registers.set_flag(RFlags::AUXILIARY, true);
        } else {
            state.registers.set_flag(RFlags::AUXILIARY, false);
        }

        if al > 0x99 || state.registers.get_flag(RFlags::CARRY) {
            result += 0x60;
            cf = true;
        }

        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFFFF00) | (result as u64);
        state.registers.set_flag(RFlags::CARRY, cf);
        self.update_logical_flags(result as u64, state);
        Ok(())
    }

    pub fn execute_das(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Decimal Adjust After Subtraction
        let al = (state.registers.rax & 0xFF) as u8;
        let mut result = al;
        let mut cf = false;

        if (al & 0x0F) > 9 || state.registers.get_flag(RFlags::AUXILIARY) {
            result -= 6;
            state.registers.set_flag(RFlags::AUXILIARY, true);
        } else {
            state.registers.set_flag(RFlags::AUXILIARY, false);
        }

        if al > 0x99 || state.registers.get_flag(RFlags::CARRY) {
            result -= 0x60;
            cf = true;
        }

        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFFFF00) | (result as u64);
        state.registers.set_flag(RFlags::CARRY, cf);
        self.update_logical_flags(result as u64, state);
        Ok(())
    }

    // Missing D instructions
    pub fn execute_db(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("DB instruction executed");
        Ok(())
    }

    pub fn execute_dd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("DD instruction executed");
        Ok(())
    }

    pub fn execute_dq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("DQ instruction executed");
        Ok(())
    }

    pub fn execute_dw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("DW instruction executed");
        Ok(())
    }

    // SIMD division instructions (simplified implementations)
    pub fn execute_divpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("DIVPD instruction executed");
        Ok(())
    }

    pub fn execute_divps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("DIVPS instruction executed");
        Ok(())
    }

    pub fn execute_divsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("DIVSD instruction executed");
        Ok(())
    }

    pub fn execute_divss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("DIVSS instruction executed");
        Ok(())
    }

    // SIMD dot product instructions
    pub fn execute_dppd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("DPPD instruction executed");
        Ok(())
    }

    pub fn execute_dpps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("DPPS instruction executed");
        Ok(())
    }

    pub fn execute_dmint(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("DMINT instruction executed");
        Ok(())
    }

    pub fn execute_delay(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("DELAY instruction executed");
        Ok(())
    }
}
