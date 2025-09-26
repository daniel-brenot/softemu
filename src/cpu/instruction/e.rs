use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    pub fn execute_enter(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ENTER instruction".to_string()));
        }

        let frame_size = self.get_operand_value(instruction, 0, state)? as u32;
        let nesting_level = self.get_operand_value(instruction, 1, state)? as u32;

        // Push current frame pointer
        state.registers.rsp -= 8;
        state.write_u64(state.registers.rsp, state.registers.rbp)?;

        // Set new frame pointer
        state.registers.rbp = state.registers.rsp;

        // Allocate local variables
        state.registers.rsp -= frame_size as u64;

        // Handle nesting levels (simplified)
        for _ in 0..nesting_level.min(31) {
            state.registers.rsp -= 8;
            // In real implementation, would push previous frame pointers
        }
        Ok(())
    }
}