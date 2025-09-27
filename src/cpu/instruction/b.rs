use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {

    pub fn execute_bextr(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 3 {
            return Err(crate::EmulatorError::Cpu("Invalid BEXTR instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let bit_index = self.get_operand_value(instruction, 2, state)?;
        let bit_pos = bit_index & 0x3F; // Mask to 6 bits for 64-bit
        
        let bit_value = (src >> bit_pos) & 1;
        Ok(())
    }

    pub fn execute_blcfill(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BLCFILL instruction".to_string()));
        }
        
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_blci(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BLCI instruction".to_string()));
        }
        
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_blcic(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {

        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BLICIC instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_blcmsk(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {

        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BLCMSK instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_blcs(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {

        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BLCS instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_blendpd(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {

        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BLENDPD instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_blendps(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {

        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BLENDPS instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_blendvpd(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {

        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BLENDVPD instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_blendvps(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {

        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BLENDVPPS instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_blsfill(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {

        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BLSFILL instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_blsi(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {

        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BLSI instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_blsic(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {

        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BLsic instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_blsmask(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {

        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BLSMASK instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_blsr(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {

        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BLSR instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_bndcl(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {

        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BNDCL instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_bndcn(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {

        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BNDCN instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_bndcu(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {

        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BNDCU instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_bt(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BT instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let bit_index = self.get_operand_value(instruction, 1, state)?;
        let bit_pos = bit_index & 0x3F; // Mask to 6 bits for 64-bit
        
        let bit_value = (src >> bit_pos) & 1;
        state.registers.set_flag(RFlags::CARRY, bit_value != 0);
        Ok(())
    }

    pub fn execute_btc(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BTC instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let bit_index = self.get_operand_value(instruction, 1, state)?;
        let bit_pos = bit_index & 0x3F; // Mask to 6 bits for 64-bit
        
        let bit_value = (src >> bit_pos) & 1;
        state.registers.set_flag(RFlags::CARRY, bit_value != 0);
        
        // Toggle the bit
        let result = src ^ (1 << bit_pos);
        self.set_operand_value(instruction, 0, result, state)?;
        Ok(())
    }

    pub fn execute_btr(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BTR instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let bit_index = self.get_operand_value(instruction, 1, state)?;
        let bit_pos = bit_index & 0x3F; // Mask to 6 bits for 64-bit
        
        let bit_value = (src >> bit_pos) & 1;
        state.registers.set_flag(RFlags::CARRY, bit_value != 0);
        
        // Clear the bit
        let result = src & !(1 << bit_pos);
        self.set_operand_value(instruction, 0, result, state)?;
        Ok(())
    }

    pub fn execute_bts(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BTS instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let bit_index = self.get_operand_value(instruction, 1, state)?;
        let bit_pos = bit_index & 0x3F; // Mask to 6 bits for 64-bit
        
        let bit_value = (src >> bit_pos) & 1;
        state.registers.set_flag(RFlags::CARRY, bit_value != 0);
        
        // Set the bit
        let result = src | (1 << bit_pos);
        self.set_operand_value(instruction, 0, result, state)?;
        Ok(())
    }

    pub fn execute_bsf(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BSF instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        
        if src == 0 {
            state.registers.set_flag(RFlags::ZERO, true);
        } else {
            state.registers.set_flag(RFlags::ZERO, false);
            let bit_index = src.trailing_zeros() as u64;
            self.set_operand_value(instruction, 1, bit_index, state)?;
        }
        Ok(())
    }

    pub fn execute_bsr(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BSR instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        
        if src == 0 {
            state.registers.set_flag(RFlags::ZERO, true);
        } else {
            state.registers.set_flag(RFlags::ZERO, false);
            let bit_index = 63 - src.leading_zeros() as u64;
            self.set_operand_value(instruction, 1, bit_index, state)?;
        }
        Ok(())
    }

    pub fn execute_bndldx(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // BNDLDX - Load Extended Bounds Using Address Translation
        // Simplified implementation - just log for now
        log::debug!("BNDLDX instruction executed");
        Ok(())
    }

    pub fn execute_bndmk(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // BNDMK - Make Bounds
        // Simplified implementation - just log for now
        log::debug!("BNDMK instruction executed");
        Ok(())
    }

    pub fn execute_bndmov(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // BNDMOV - Move Bounds
        // Simplified implementation - just log for now
        log::debug!("BNDMOV instruction executed");
        Ok(())
    }

    pub fn execute_bndstx(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // BNDSTX - Store Extended Bounds Using Address Translation
        // Simplified implementation - just log for now
        log::debug!("BNDSTX instruction executed");
        Ok(())
    }

    pub fn execute_bound(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // BOUND - Check Array Index Against Bounds
        // Simplified implementation - just log for now
        log::debug!("BOUND instruction executed");
        Ok(())
    }

    pub fn execute_bswap(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid BSWAP instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let result = src.swap_bytes();
        self.set_operand_value(instruction, 0, result, state)?;
        Ok(())
    }

    pub fn execute_bzhi(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // BZHI - Zero High Bits Starting with Specified Bit Position
        // Simplified implementation - just log for now
        log::debug!("BZHI instruction executed");
        Ok(())
    }

    pub fn execute_bb0_reset(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("BB0_RESET instruction executed");
        Ok(())
    }

    pub fn execute_bb1_reset(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("BB1_RESET instruction executed");
        Ok(())
    }
}
