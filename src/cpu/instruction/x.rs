use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    pub fn execute_xor(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid XOR instruction".to_string()));
        }

        // Get operand sizes
        let dst_size = self.get_operand_size(instruction, 0);
        let src_size = self.get_operand_size(instruction, 1);
        let result_size = dst_size.max(src_size);

        // Get operand values with proper size handling
        let dst = self.get_operand_value_with_size(instruction, 0, dst_size, state)?;
        let src = self.get_operand_value_with_size(instruction, 1, src_size, state)?;
        
        let result = dst ^ src;

        self.set_operand_value_with_size(instruction, 0, result, result_size, state)?;
        self.update_logical_flags_with_size(result, result_size, state);
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

    pub fn execute_xabort(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XABORT instruction executed");
        Ok(())
    }

    pub fn execute_xadd(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid XADD instruction".to_string()));
        }

        // Get operand sizes
        let dst_size = self.get_operand_size(instruction, 0);
        let src_size = self.get_operand_size(instruction, 1);
        let result_size = dst_size.max(src_size);

        // Get operand values with proper size handling
        let dst = self.get_operand_value_with_size(instruction, 0, dst_size, state)?;
        let src = self.get_operand_value_with_size(instruction, 1, src_size, state)?;
        
        let result = dst.wrapping_add(src);

        // XADD: exchange and add - store result in destination, store original destination in source
        self.set_operand_value(instruction, 0, result, state)?;
        self.set_operand_value(instruction, 1, dst, state)?;
        
        // Update arithmetic flags (XADD is addition, so is_subtraction = false)
        self.update_arithmetic_flags(result, src, dst, false, state);
        Ok(())
    }

    pub fn execute_xbegin(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XBEGIN instruction executed");
        Ok(())
    }

    pub fn execute_xbts(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XBTS instruction executed");
        Ok(())
    }

    pub fn execute_xcryptcbc(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XCRYPTCBC instruction executed");
        Ok(())
    }

    pub fn execute_xcryptcfb(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XCRYPTCFB instruction executed");
        Ok(())
    }

    pub fn execute_xcryptctr(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XCRYPTCTR instruction executed");
        Ok(())
    }

    pub fn execute_xcryptecb(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XCRYPTECB instruction executed");
        Ok(())
    }

    pub fn execute_xcryptofb(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XCRYPTOFB instruction executed");
        Ok(())
    }

    pub fn execute_xend(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XEND instruction executed");
        Ok(())
    }

    pub fn execute_xgetbv(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XGETBV instruction executed");
        Ok(())
    }

    pub fn execute_xorpd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XORPD instruction executed");
        Ok(())
    }

    pub fn execute_xorps(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XORPS instruction executed");
        Ok(())
    }

    pub fn execute_xrstor(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XRSTOR instruction executed");
        Ok(())
    }

    pub fn execute_xrstor64(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XRSTOR64 instruction executed");
        Ok(())
    }

    pub fn execute_xrstors(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XRSTORS instruction executed");
        Ok(())
    }

    pub fn execute_xrstors64(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XRSTORS64 instruction executed");
        Ok(())
    }

    pub fn execute_xsave(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XSAVE instruction executed");
        Ok(())
    }

    pub fn execute_xsave64(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XSAVE64 instruction executed");
        Ok(())
    }

    pub fn execute_xsavec(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XSAVEC instruction executed");
        Ok(())
    }

    pub fn execute_xsavec64(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XSAVEC64 instruction executed");
        Ok(())
    }

    pub fn execute_xsaveopt(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XSAVEOPT instruction executed");
        Ok(())
    }

    pub fn execute_xsaveopt64(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XSAVEOPT64 instruction executed");
        Ok(())
    }

    pub fn execute_xsaves(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XSAVES instruction executed");
        Ok(())
    }

    pub fn execute_xsaves64(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XSAVES64 instruction executed");
        Ok(())
    }

    pub fn execute_xsetbv(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XSETBV instruction executed");
        Ok(())
    }

    pub fn execute_xsha1(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XSHA1 instruction executed");
        Ok(())
    }

    pub fn execute_xsha256(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XSHA256 instruction executed");
        Ok(())
    }

    pub fn execute_xstore(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XSTORE instruction executed");
        Ok(())
    }

    pub fn execute_xtest(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XTEST instruction executed");
        Ok(())
    }

    pub fn execute_xsusldtrk(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XSUSLDTRK instruction executed");
        Ok(())
    }

    pub fn execute_xresldtrk(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XRESLDTRK instruction executed");
        Ok(())
    }

    pub fn execute_xsha512(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XSHA512 instruction executed");
        Ok(())
    }

    pub fn execute_xstore_alt(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XSTORE_ALT instruction executed");
        Ok(())
    }

    pub fn execute_xsha512_alt(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("XSHA512_ALT instruction executed");
        Ok(())
    }
}
