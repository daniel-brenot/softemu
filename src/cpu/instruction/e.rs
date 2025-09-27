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

    // Missing E instructions
    pub fn execute_emms(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("EMMS instruction executed");
        Ok(())
    }

    pub fn execute_encls(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("ENCLS instruction executed");
        Ok(())
    }

    pub fn execute_enclu(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("ENCLU instruction executed");
        Ok(())
    }

    pub fn execute_enclv(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("ENCLV instruction executed");
        Ok(())
    }

    pub fn execute_endbr32(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("ENDBR32 instruction executed");
        Ok(())
    }

    pub fn execute_endbr64(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("ENDBR64 instruction executed");
        Ok(())
    }

    pub fn execute_enqcmd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("ENQCMD instruction executed");
        Ok(())
    }

    pub fn execute_enqcmds(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("ENQCMDS instruction executed");
        Ok(())
    }

    pub fn execute_extractps(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("EXTRACTPS instruction executed");
        Ok(())
    }

    pub fn execute_extrq(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("EXTRQ instruction executed");
        Ok(())
    }

    pub fn execute_encodekey128(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("ENCODEKEY128 instruction executed");
        Ok(())
    }

    pub fn execute_encodekey256(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("ENCODEKEY256 instruction executed");
        Ok(())
    }

    pub fn execute_eretu(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("ERETU instruction executed");
        Ok(())
    }

    pub fn execute_erets(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("ERETS instruction executed");
        Ok(())
    }
}
