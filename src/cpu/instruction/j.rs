use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    fn get_jump_target(&self, instruction: &Instruction, state: &CpuState) -> Result<u64> {
        let operand = instruction.try_op_kind(0).unwrap();
        match operand {
            iced_x86::OpKind::Immediate8 => {
                // Relative jump: target = current_rip + instruction_length + offset
                let offset = instruction.immediate8() as i8 as i64;
                Ok((state.registers.rip + instruction.len() as u64).wrapping_add(offset as u64))
            }
            iced_x86::OpKind::Immediate16 => {
                // Relative jump: target = current_rip + instruction_length + offset
                let offset = instruction.immediate16() as i16 as i64;
                Ok((state.registers.rip + instruction.len() as u64).wrapping_add(offset as u64))
            }
            iced_x86::OpKind::Immediate32 => {
                // Relative jump: target = current_rip + instruction_length + offset
                let offset = instruction.immediate32() as i32 as i64;
                Ok((state.registers.rip + instruction.len() as u64).wrapping_add(offset as u64))
            }
            iced_x86::OpKind::Immediate64 => {
                // Absolute jump
                Ok(instruction.immediate64())
            }
            iced_x86::OpKind::Register => {
                // Indirect jump to register value
                let reg = instruction.op_register(0);
                Ok(self.get_register_value(reg, state))
            }
            iced_x86::OpKind::Memory => {
                // Indirect jump to memory value
                let addr = self.calculate_memory_address(instruction, 0, state)?;
                state.read_u64(addr)
            }
            iced_x86::OpKind::NearBranch16 => {
                // Relative jump: target = current_rip + instruction_length + offset
                let offset = instruction.near_branch16() as i16 as i64;
                Ok((state.registers.rip + instruction.len() as u64).wrapping_add(offset as u64))
            }
            iced_x86::OpKind::NearBranch32 => {
                // Relative jump: target = current_rip + instruction_length + offset
                let offset = instruction.near_branch32() as i32 as i64;
                Ok((state.registers.rip + instruction.len() as u64).wrapping_add(offset as u64))
            }
            iced_x86::OpKind::NearBranch64 => {
                // Relative jump: target = current_rip + instruction_length + offset
                let offset = instruction.near_branch64() as i64;
                Ok((state.registers.rip + instruction.len() as u64).wrapping_add(offset as u64))
            }
            _ => Err(crate::EmulatorError::Cpu("Unsupported jump operand kind".to_string())),
        }
    }
    pub fn execute_jmp(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let target = self.get_jump_target(instruction, state)?;
        state.registers.rip = target;
        Ok(())
    }

    pub fn execute_je(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if state.registers.get_flag(RFlags::ZERO) {
            let target = self.get_jump_target(instruction, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jne(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if !state.registers.get_flag(RFlags::ZERO) {
            let target = self.get_jump_target(instruction, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jl(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let sign = state.registers.get_flag(RFlags::SIGN);
        let overflow = state.registers.get_flag(RFlags::OVERFLOW);
        if sign != overflow {
            let target = self.get_jump_target(instruction, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jle(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let zero = state.registers.get_flag(RFlags::ZERO);
        let sign = state.registers.get_flag(RFlags::SIGN);
        let overflow = state.registers.get_flag(RFlags::OVERFLOW);
        if zero || (sign != overflow) {
            let target = self.get_jump_target(instruction, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jg(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let zero = state.registers.get_flag(RFlags::ZERO);
        let sign = state.registers.get_flag(RFlags::SIGN);
        let overflow = state.registers.get_flag(RFlags::OVERFLOW);
        if !zero && (sign == overflow) {
            let target = self.get_jump_target(instruction, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jge(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let sign = state.registers.get_flag(RFlags::SIGN);
        let overflow = state.registers.get_flag(RFlags::OVERFLOW);
        if sign == overflow {
            let target = self.get_jump_target(instruction, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jb(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if state.registers.get_flag(RFlags::CARRY) {
            let target = self.get_jump_target(instruction, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jbe(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let carry = state.registers.get_flag(RFlags::CARRY);
        let zero = state.registers.get_flag(RFlags::ZERO);
        if carry || zero {
            let target = self.get_jump_target(instruction, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_ja(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let carry = state.registers.get_flag(RFlags::CARRY);
        let zero = state.registers.get_flag(RFlags::ZERO);
        if !carry && !zero {
            let target = self.get_jump_target(instruction, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jae(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if !state.registers.get_flag(RFlags::CARRY) {
            let target = self.get_jump_target(instruction, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_js(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if state.registers.get_flag(RFlags::SIGN) {
            let target = self.get_jump_target(instruction, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jns(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if !state.registers.get_flag(RFlags::SIGN) {
            let target = self.get_jump_target(instruction, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jo(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if state.registers.get_flag(RFlags::OVERFLOW) {
            let target = self.get_jump_target(instruction, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jno(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if !state.registers.get_flag(RFlags::OVERFLOW) {
            let target = self.get_jump_target(instruction, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jcxz(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid JCXZ instruction".to_string()));
        }

        // Jump if CX (16-bit part of RCX) is zero
        if (state.registers.rcx & 0xFFFF) == 0 {
            let target = self.get_jump_target(instruction, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jecxz(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid JECXZ instruction".to_string()));
        }

        // Jump if ECX (32-bit part of RCX) is zero
        if (state.registers.rcx & 0xFFFFFFFF) == 0 {
            let target = self.get_jump_target(instruction, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jrcxz(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid JRCXZ instruction".to_string()));
        }

        // Jump if RCX (64-bit) is zero
        if state.registers.rcx == 0 {
            let target = self.get_jump_target(instruction, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jmpe(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("JMPE instruction executed");
        Ok(())
    }

    pub fn execute_jnp(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if !state.registers.get_flag(RFlags::PARITY) {
            let target = self.get_jump_target(instruction, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jp(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if state.registers.get_flag(RFlags::PARITY) {
            let target = self.get_jump_target(instruction, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    pub fn execute_jknzd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("JKNZD instruction executed");
        Ok(())
    }

    pub fn execute_jkzd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("JKZD instruction executed");
        Ok(())
    }
}
