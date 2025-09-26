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
}