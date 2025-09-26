use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    pub fn execute_jmp(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let target = self.get_operand_value(instruction, 0, state)?;
        state.registers.rip = target;
        Ok(())
    }

    pub fn execute_je(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if state.registers.get_flag(RFlags::ZERO) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jne(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if !state.registers.get_flag(RFlags::ZERO) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jl(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let sign = state.registers.get_flag(RFlags::SIGN);
        let overflow = state.registers.get_flag(RFlags::OVERFLOW);
        if sign != overflow {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jle(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let zero = state.registers.get_flag(RFlags::ZERO);
        let sign = state.registers.get_flag(RFlags::SIGN);
        let overflow = state.registers.get_flag(RFlags::OVERFLOW);
        if zero || (sign != overflow) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jg(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let zero = state.registers.get_flag(RFlags::ZERO);
        let sign = state.registers.get_flag(RFlags::SIGN);
        let overflow = state.registers.get_flag(RFlags::OVERFLOW);
        if !zero && (sign == overflow) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jge(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let sign = state.registers.get_flag(RFlags::SIGN);
        let overflow = state.registers.get_flag(RFlags::OVERFLOW);
        if sign == overflow {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jb(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if state.registers.get_flag(RFlags::CARRY) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jbe(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let carry = state.registers.get_flag(RFlags::CARRY);
        let zero = state.registers.get_flag(RFlags::ZERO);
        if carry || zero {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_ja(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let carry = state.registers.get_flag(RFlags::CARRY);
        let zero = state.registers.get_flag(RFlags::ZERO);
        if !carry && !zero {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jae(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if !state.registers.get_flag(RFlags::CARRY) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_js(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if state.registers.get_flag(RFlags::SIGN) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jns(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if !state.registers.get_flag(RFlags::SIGN) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jo(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if state.registers.get_flag(RFlags::OVERFLOW) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jno(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if !state.registers.get_flag(RFlags::OVERFLOW) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jcxz(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid JCXZ instruction".to_string()));
        }

        // Jump if CX (16-bit part of RCX) is zero
        if (state.registers.rcx & 0xFFFF) == 0 {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jecxz(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid JECXZ instruction".to_string()));
        }

        // Jump if ECX (32-bit part of RCX) is zero
        if (state.registers.rcx & 0xFFFFFFFF) == 0 {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jrcxz(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid JRCXZ instruction".to_string()));
        }

        // Jump if RCX (64-bit) is zero
        if state.registers.rcx == 0 {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }
}