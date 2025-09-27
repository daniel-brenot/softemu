use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {

    pub fn execute_ucomisd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("UCOMISD instruction executed");
        Ok(())
    }

    pub fn execute_ucomiss(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("UCOMISS instruction executed");
        Ok(())
    }

    pub fn execute_ud0(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("UD0 instruction executed");
        Ok(())
    }

    pub fn execute_ud1(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("UD1 instruction executed");
        Ok(())
    }

    pub fn execute_ud2(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("UD2 instruction executed");
        Ok(())
    }

    pub fn execute_umonitor(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("UMONITOR instruction executed");
        Ok(())
    }

    pub fn execute_umov(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("UMOV instruction executed");
        Ok(())
    }

    pub fn execute_umwait(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("UMWAIT instruction executed");
        Ok(())
    }

    pub fn execute_unpckhpd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("UNPCKHPD instruction executed");
        Ok(())
    }

    pub fn execute_unpckhps(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("UNPCKHPS instruction executed");
        Ok(())
    }

    pub fn execute_unpcklpd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("UNPCKLPD instruction executed");
        Ok(())
    }

    pub fn execute_unpcklps(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("UNPCKLPS instruction executed");
        Ok(())
    }

    pub fn execute_undoc(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("UNDOC instruction executed");
        Ok(())
    }

    pub fn execute_uiret(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("UIRET instruction executed");
        Ok(())
    }
}
