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

        let dst = self.get_operand_value(instruction, 0, state)?;  // Destination (first operand)
        let src = self.get_operand_value(instruction, 1, state)?;  // Source (second operand)
        let carry = if state.registers.get_flag(RFlags::CARRY) { 1 } else { 0 };
        let result = dst.wrapping_add(src).wrapping_add(carry);

        self.set_operand_value(instruction, 0, result, state)?;  // Set result to destination
        self.update_arithmetic_flags(result, src, dst, false, state);
        Ok(())
    }

    pub fn execute_adcx(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ADCX instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let carry_in = if state.registers.get_flag(RFlags::CARRY) { 1 } else { 0 };
        
        // Perform addition with carry using overflowing_add
        let (sum1, overflow1) = dst.overflowing_add(src);
        let (result, overflow2) = sum1.overflowing_add(carry_in);
        
        // Set the result
        self.set_operand_value(instruction, 1, result, state)?;
        
        // Update flags - ADCX only affects the carry flag, not other arithmetic flags
        let carry_out = overflow1 || overflow2;
        state.registers.set_flag(RFlags::CARRY, carry_out);
        
        // Clear other flags that ADCX doesn't affect
        state.registers.set_flag(RFlags::ZERO, false);
        state.registers.set_flag(RFlags::SIGN, false);
        state.registers.set_flag(RFlags::OVERFLOW, false);
        state.registers.set_flag(RFlags::PARITY, false);
        
        Ok(())
    }

    pub fn execute_add(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ADD instruction".to_string()));
        }

        let dst = self.get_operand_value(instruction, 0, state)?;  // Destination (first operand)
        let src = self.get_operand_value(instruction, 1, state)?;  // Source (second operand)
        let result = dst.wrapping_add(src);
        self.set_operand_value(instruction, 0, result, state)?;  // Set result to destination
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

        let dst = self.get_operand_value(instruction, 0, state)?;  // Destination (first operand)
        let src = self.get_operand_value(instruction, 1, state)?;  // Source (second operand)
        let result = dst & src;

        self.set_operand_value(instruction, 0, result, state)?;  // Set result to destination
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

    pub fn execute_arpl(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ARPL instruction".to_string()));
        }

        // Get the operands
        let dst = self.get_operand_value(instruction, 0, state)?;
        let src = self.get_operand_value(instruction, 1, state)?;

        // ARPL works with 16-bit operands, so mask to 16 bits
        let dst_16 = (dst & 0xFFFF) as u16;
        let src_16 = (src & 0xFFFF) as u16;

        // Extract RPL fields (bits 0-1)
        let dst_rpl = (dst_16 & 0x03) as u8;
        let src_rpl = (src_16 & 0x03) as u8;

        // ARPL adjusts the RPL field of the destination to the higher of the two RPL values
        let new_rpl = dst_rpl.max(src_rpl);
        let result_16 = (dst_16 & 0xFFFC) | (new_rpl as u16);

        // Set the result (preserve upper bits of the register)
        let result = (dst & 0xFFFF0000) | (result_16 as u64);
        
        self.set_operand_value(instruction, 0, result, state)?;

        // Set the zero flag: ZF = 1 if the RPL field was changed, 0 if unchanged
        let rpl_changed = dst_rpl != new_rpl;
        state.registers.set_flag(RFlags::ZERO, !rpl_changed);

        Ok(())
    }

    pub fn execute_altinst(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("ALTINST instruction executed");
        Ok(())
    }

    pub fn execute_aesencwide128kl(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("AESENCWIDE128KL instruction executed");
        Ok(())
    }

    pub fn execute_aesdecwide128kl(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("AESDECWIDE128KL instruction executed");
        Ok(())
    }

    pub fn execute_aesencwide256kl(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("AESENCWIDE256KL instruction executed");
        Ok(())
    }

    pub fn execute_aesdecwide256kl(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("AESDECWIDE256KL instruction executed");
        Ok(())
    }

    pub fn execute_aesenc128kl(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("AESENC128KL instruction executed");
        Ok(())
    }

    pub fn execute_aesdec128kl(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("AESDEC128KL instruction executed");
        Ok(())
    }

    pub fn execute_aesenc256kl(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("AESENC256KL instruction executed");
        Ok(())
    }

    pub fn execute_aesdec256kl(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("AESDEC256KL instruction executed");
        Ok(())
    }

    pub fn execute_aadd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("AADD instruction executed");
        Ok(())
    }

    pub fn execute_aand(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("AAND instruction executed");
        Ok(())
    }

    pub fn execute_aor(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("AOR instruction executed");
        Ok(())
    }

    pub fn execute_axor(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("AXOR instruction executed");
        Ok(())
    }
}
