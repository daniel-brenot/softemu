use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    pub fn execute_nop(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // NOP - No Operation
        // Do nothing
        Ok(())
    }

    pub fn execute_neg(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid NEG instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let result = 0u64.wrapping_sub(src);
        
        self.set_operand_value(instruction, 0, result, state)?;
        self.update_arithmetic_flags(result, src, 0, true, state);
        Ok(())
    }

    pub fn execute_not(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid NOT instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let result = !src;
        
        self.set_operand_value(instruction, 0, result, state)?;
        // NOT doesn't affect flags
        Ok(())
    }
}