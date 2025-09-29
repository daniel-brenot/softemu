use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {

    pub fn execute_zero_bytes(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("ZERO_BYTES instruction executed");
        Ok(())
    }
}
