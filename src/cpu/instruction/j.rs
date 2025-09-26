use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    fn execute_jmp(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let target = self.get_operand_value(instruction, 0, state)?;
        state.registers.rip = target;
        Ok(())
    }
}