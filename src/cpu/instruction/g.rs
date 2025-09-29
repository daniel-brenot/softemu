use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    // G instructions
    pub fn execute_getsec(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("GETSEC instruction executed");
        Ok(())
    }

    pub fn execute_gf2p8affineinvqb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("GF2P8AFFINEINVQB instruction executed");
        Ok(())
    }

    pub fn execute_gf2p8affineqb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("GF2P8AFFINEQB instruction executed");
        Ok(())
    }

    pub fn execute_gf2p8mulb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("GF2P8MULB instruction executed");
        Ok(())
    }

    pub fn execute_getsecq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("GETSECQ instruction executed");
        Ok(())
    }
}
