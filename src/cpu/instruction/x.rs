use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    pub fn execute_xor(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid XOR instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst ^ src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_xchg(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid XCHG instruction".to_string()));
        }

        let val1 = self.get_operand_value(instruction, 0, state)?;
        let val2 = self.get_operand_value(instruction, 1, state)?;
        
        self.set_operand_value(instruction, 0, val2, state)?;
        self.set_operand_value(instruction, 1, val1, state)?;
        Ok(())
    }

    pub fn execute_xlat(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Table lookup: AL = [BX + AL]
        let offset = (state.registers.rbx & 0xFFFF) + (state.registers.rax & 0xFF);
        let value = state.read_u8(offset)?;
        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFFFF00) | (value as u64);
        Ok(())
    }
}