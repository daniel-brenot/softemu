use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {

    pub fn execute_aaa(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // ASCII Adjust After Addition
        let al = (state.registers.rax & 0xFF) as u8;
        if (al & 0x0F) > 9 || state.registers.get_flag(RFlags::AUXILIARY) {
            state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFFFF00) | ((al + 6) as u64);
            state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFF00FF) | (((state.registers.rax >> 8) + 1) << 8);
            state.registers.set_flag(RFlags::AUXILIARY, true);
            state.registers.set_flag(RFlags::CARRY, true);
        } else {
            state.registers.set_flag(RFlags::AUXILIARY, false);
            state.registers.set_flag(RFlags::CARRY, false);
        }
        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFFFF0F) | ((state.registers.rax & 0x0F) as u64);
        Ok(())
    }

    pub fn execute_aad(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // ASCII Adjust Before Division
        let imm = if instruction.op_count() == 1 {
            self.get_operand_value(instruction, 0, state)? as u8
        } else {
            10
        };
        let al = (state.registers.rax & 0xFF) as u8;
        let ah = ((state.registers.rax >> 8) & 0xFF) as u8;
        let result = al + (ah * imm);
        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFF0000) | (result as u64);
        self.update_logical_flags(result as u64, state);
        Ok(())
    }

    pub fn execute_aam(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // ASCII Adjust After Multiplication
        let imm = if instruction.op_count() == 1 {
            self.get_operand_value(instruction, 0, state)? as u8
        } else {
            10
        };
        let al = (state.registers.rax & 0xFF) as u8;
        let ah = al / imm;
        let new_al = al % imm;
        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFF0000) | ((ah as u64) << 8) | (new_al as u64);
        self.update_logical_flags(new_al as u64, state);
        Ok(())
    }

    pub fn execute_aas(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // ASCII Adjust After Subtraction
        let al = (state.registers.rax & 0xFF) as u8;
        if (al & 0x0F) > 9 || state.registers.get_flag(RFlags::AUXILIARY) {
            state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFFFF00) | ((al - 6) as u64);
            state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFF00FF) | (((state.registers.rax >> 8) - 1) << 8);
            state.registers.set_flag(RFlags::AUXILIARY, true);
            state.registers.set_flag(RFlags::CARRY, true);
        } else {
            state.registers.set_flag(RFlags::AUXILIARY, false);
            state.registers.set_flag(RFlags::CARRY, false);
        }
        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFFFF0F) | ((state.registers.rax & 0x0F) as u64);
        Ok(())
    }

    pub fn execute_adc(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ADC instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let carry = if state.registers.get_flag(RFlags::CARRY) { 1 } else { 0 };
        let result = dst.wrapping_add(src).wrapping_add(carry);

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_arithmetic_flags(result, src, dst, false, state);
        Ok(())
    }

    pub fn execute_add(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ADD instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst.wrapping_add(src);

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_arithmetic_flags(result, src, dst, false, state);
        Ok(())
    }

    pub fn execute_addpd(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ADDPD instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst.wrapping_add(src);

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_arithmetic_flags(result, src, dst, false, state);
        Ok(())
    }

    pub fn execute_addps(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ADDPS instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst.wrapping_add(src);

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_arithmetic_flags(result, src, dst, false, state);
        Ok(())
    }

    pub fn execute_addsd(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ADDSD instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst.wrapping_add(src);

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_arithmetic_flags(result, src, dst, false, state);
        Ok(())
    }

    pub fn execute_addss(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ADDSS instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst.wrapping_add(src);

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_arithmetic_flags(result, src, dst, false, state);
        Ok(())
    }

    pub fn execute_addsubpd(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ADDSUBPD instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst.wrapping_add(src);

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_arithmetic_flags(result, src, dst, false, state);
        Ok(())
    }

    pub fn execute_addsubps(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ADDSUBPS instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst.wrapping_add(src);

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_arithmetic_flags(result, src, dst, false, state);
        Ok(())
    }

    pub fn execute_adox(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ADOX instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst.wrapping_add(src);

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_arithmetic_flags(result, src, dst, false, state);
        Ok(())
    }

    pub fn execute_aesdec(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid AESDEC instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst.wrapping_add(src);

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_arithmetic_flags(result, src, dst, false, state);
        Ok(())
    }

    pub fn execute_aesdeclast(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid AESDECLAST instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst.wrapping_add(src);

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_arithmetic_flags(result, src, dst, false, state);
        Ok(())
    }

    pub fn execute_aesenc(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid AESENC instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst.wrapping_add(src);

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_arithmetic_flags(result, src, dst, false, state);
        Ok(())
    }

    pub fn execute_aesenclast(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid AESENCLAST instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst.wrapping_add(src);

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_arithmetic_flags(result, src, dst, false, state);
        Ok(())
    }

    pub fn execute_aesimc(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid AESIMC instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst.wrapping_add(src);

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_arithmetic_flags(result, src, dst, false, state);
        Ok(())
    }

    pub fn execute_aeskeygenassist(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid AESKEYGENASSIST instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst.wrapping_add(src);

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_arithmetic_flags(result, src, dst, false, state);
        Ok(())
    }

    pub fn execute_and(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid AND instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_andn(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ANDN instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_andnpd(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ANDNPD instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_andnps(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ANDNPS instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_andpd(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ANDPD instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_andps(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ANDPS instruction".to_string()));
        }
        
        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    pub fn execute_arpl(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ARPL instruction".to_string()));
        }
        // Adjust RPL (simplified - just log for now)
        log::debug!("ARPL instruction executed");
        Ok(())
    }
}