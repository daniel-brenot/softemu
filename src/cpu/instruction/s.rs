use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    pub fn execute_sub(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid SUB instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst.wrapping_sub(src);

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_arithmetic_flags(result, src, dst, true, state);
        Ok(())
    }

    pub fn execute_scasb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Compare AL with byte at [RDI]
        let dst_addr = state.registers.rdi;
        
        let src_value = (state.registers.rax & 0xFF) as u64;
        let dst_value = state.read_u8(dst_addr)? as u64;
        let result = src_value.wrapping_sub(dst_value);
        
        // Update flags
        self.update_arithmetic_flags(result, dst_value, src_value, true, state);
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rdi = state.registers.rdi.wrapping_sub(1);
        } else {
            state.registers.rdi = state.registers.rdi.wrapping_add(1);
        }
        Ok(())
    }

    pub fn execute_scasw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Compare AX with word at [RDI]
        let dst_addr = state.registers.rdi;
        
        let src_value = (state.registers.rax & 0xFFFF) as u64;
        let dst_value = state.read_u16(dst_addr)? as u64;
        let result = src_value.wrapping_sub(dst_value);
        
        // Update flags
        self.update_arithmetic_flags(result, dst_value, src_value, true, state);
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rdi = state.registers.rdi.wrapping_sub(2);
        } else {
            state.registers.rdi = state.registers.rdi.wrapping_add(2);
        }
        Ok(())
    }

    pub fn execute_scasd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Compare EAX with doubleword at [RDI]
        let dst_addr = state.registers.rdi;
        
        let src_value = (state.registers.rax & 0xFFFFFFFF) as u64;
        let dst_value = state.read_u32(dst_addr)? as u64;
        let result = src_value.wrapping_sub(dst_value);
        
        // Update flags
        self.update_arithmetic_flags(result, dst_value, src_value, true, state);
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rdi = state.registers.rdi.wrapping_sub(4);
        } else {
            state.registers.rdi = state.registers.rdi.wrapping_add(4);
        }
        Ok(())
    }

    pub fn execute_scasq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Compare RAX with quadword at [RDI]
        let dst_addr = state.registers.rdi;
        
        let src_value = state.registers.rax;
        let dst_value = state.read_u64(dst_addr)?;
        let result = src_value.wrapping_sub(dst_value);
        
        // Update flags
        self.update_arithmetic_flags(result, dst_value, src_value, true, state);
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rdi = state.registers.rdi.wrapping_sub(8);
        } else {
            state.registers.rdi = state.registers.rdi.wrapping_add(8);
        }
        Ok(())
    }

    pub fn execute_stosb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Store AL to byte at [RDI]
        let dst_addr = state.registers.rdi;
        
        let value = (state.registers.rax & 0xFF) as u8;
        state.write_u8(dst_addr, value)?;
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rdi = state.registers.rdi.wrapping_sub(1);
        } else {
            state.registers.rdi = state.registers.rdi.wrapping_add(1);
        }
        Ok(())
    }

    pub fn execute_stosw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Store AX to word at [RDI]
        let dst_addr = state.registers.rdi;
        
        let value = (state.registers.rax & 0xFFFF) as u16;
        state.write_u16(dst_addr, value)?;
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rdi = state.registers.rdi.wrapping_sub(2);
        } else {
            state.registers.rdi = state.registers.rdi.wrapping_add(2);
        }
        Ok(())
    }

    pub fn execute_stosd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Store EAX to doubleword at [RDI]
        let dst_addr = state.registers.rdi;
        
        let value = (state.registers.rax & 0xFFFFFFFF) as u32;
        state.write_u32(dst_addr, value)?;
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rdi = state.registers.rdi.wrapping_sub(4);
        } else {
            state.registers.rdi = state.registers.rdi.wrapping_add(4);
        }
        Ok(())
    }

    pub fn execute_stosq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Store RAX to quadword at [RDI]
        let dst_addr = state.registers.rdi;
        
        let value = state.registers.rax;
        state.write_u64(dst_addr, value)?;
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rdi = state.registers.rdi.wrapping_sub(8);
        } else {
            state.registers.rdi = state.registers.rdi.wrapping_add(8);
        }
        Ok(())
    }

    pub fn execute_syscall(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Save return address and flags
        state.registers.rsp -= 8;
        state.write_u64(state.registers.rsp, state.registers.rip)?;
        state.registers.rsp -= 8;
        state.write_u64(state.registers.rsp, state.registers.rflags)?;
        
        // Load new RIP from LSTAR MSR (simplified)
        state.registers.rip = state.registers.msr_lstar;
        
        // Clear direction flag and interrupt flag
        state.registers.set_flag(RFlags::DIRECTION, false);
        state.registers.set_flag(RFlags::INTERRUPT, false);
        
        // Set privilege level to 0 (kernel mode)
        state.set_privilege_level(0);
        Ok(())
    }

    pub fn execute_sysret(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Restore flags and return address
        let rflags = state.read_u64(state.registers.rsp)?;
        state.registers.rsp += 8;
        let rip = state.read_u64(state.registers.rsp)?;
        state.registers.rsp += 8;
        
        state.registers.rflags = rflags;
        state.registers.rip = rip;
        
        // Set privilege level to 3 (user mode)
        state.set_privilege_level(3);
        Ok(())
    }

    pub fn execute_sti(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Set interrupt flag
        state.registers.set_flag(RFlags::INTERRUPT, true);
        Ok(())
    }

    pub fn execute_str(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid STR instruction".to_string()));
        }
        // Store Task Register (simplified - just log for now)
        log::debug!("STR instruction executed");
        Ok(())
    }

    pub fn execute_smsw(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid SMSW instruction".to_string()));
        }
        // Store Machine Status Word (simplified - just log for now)
        log::debug!("SMSW instruction executed");
        Ok(())
    }

    pub fn execute_sahf(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Store AH register into flags
        let ah_value = (state.registers.rax >> 8) & 0xFF;
        let current_flags = state.registers.get_flags();
        let new_flags = RFlags::from_bits_truncate((current_flags.bits() & 0xFFFFFFFFFFFFFF00) | ah_value);
        state.registers.set_flags(new_flags);
        Ok(())
    }

    pub fn execute_shl(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid SHL instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let count = self.get_operand_value(instruction, 1, state)? & 0x3F; // Mask to 6 bits
        
        let result = src << count;
        self.set_operand_value(instruction, 0, result, state)?;
        self.update_shift_flags(result, src, count, state);
        Ok(())
    }

    pub fn execute_shr(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid SHR instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let count = self.get_operand_value(instruction, 1, state)? & 0x3F; // Mask to 6 bits
        
        let result = src >> count;
        self.set_operand_value(instruction, 0, result, state)?;
        self.update_shift_flags(result, src, count, state);
        Ok(())
    }

    pub fn execute_sar(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid SAR instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)? as i64;
        let count = self.get_operand_value(instruction, 1, state)? & 0x3F; // Mask to 6 bits
        
        let result = (src >> count) as u64;
        self.set_operand_value(instruction, 0, result, state)?;
        self.update_shift_flags(result, src as u64, count, state);
        Ok(())
    }

    pub fn execute_rol(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ROL instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let count = self.get_operand_value(instruction, 1, state)? & 0x3F; // Mask to 6 bits
        
        let result = src.rotate_left(count as u32);
        self.set_operand_value(instruction, 0, result, state)?;
        self.update_rotate_flags(result, src, count, state);
        Ok(())
    }

    pub fn execute_ror(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ROR instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let count = self.get_operand_value(instruction, 1, state)? & 0x3F; // Mask to 6 bits
        
        let result = src.rotate_right(count as u32);
        self.set_operand_value(instruction, 0, result, state)?;
        self.update_rotate_flags(result, src, count, state);
        Ok(())
    }

    pub fn execute_rcl(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid RCL instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let count = self.get_operand_value(instruction, 1, state)? & 0x3F; // Mask to 6 bits
        
        let carry = if state.registers.get_flag(RFlags::CARRY) { 1 } else { 0 };
        let result = (src << count) | (carry << (count - 1));
        
        self.set_operand_value(instruction, 0, result, state)?;
        self.update_rotate_flags(result, src, count, state);
        Ok(())
    }

    pub fn execute_rcr(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid RCR instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let count = self.get_operand_value(instruction, 1, state)? & 0x3F; // Mask to 6 bits
        
        let carry = if state.registers.get_flag(RFlags::CARRY) { 1 } else { 0 };
        let result = (src >> count) | (carry << (63 - count));
        
        self.set_operand_value(instruction, 0, result, state)?;
        self.update_rotate_flags(result, src, count, state);
        Ok(())
    }

    pub fn execute_stc(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Set carry flag
        state.registers.set_flag(RFlags::CARRY, true);
        Ok(())
    }

    pub fn execute_std(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Set direction flag
        state.registers.set_flag(RFlags::DIRECTION, true);
        Ok(())
    }

    pub fn execute_salc(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Set AL on Carry
        let al_value = if state.registers.get_flag(RFlags::CARRY) { 0xFF } else { 0x00 };
        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFFFF00) | (al_value as u64);
        Ok(())
    }

    pub fn execute_sbb(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid SBB instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let borrow = if state.registers.get_flag(RFlags::CARRY) { 1 } else { 0 };
        let result = dst.wrapping_sub(src).wrapping_sub(borrow);

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_arithmetic_flags(result, src, dst, true, state);
        Ok(())
    }

    pub fn execute_sfence(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Store Fence (memory ordering)
        // For now, just do nothing
        Ok(())
    }

    pub fn execute_sgdt(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid SGDT instruction".to_string()));
        }
        // Store Global Descriptor Table (simplified - just log for now)
        log::debug!("SGDT instruction executed");
        Ok(())
    }

    pub fn execute_sidt(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid SIDT instruction".to_string()));
        }
        // Store Interrupt Descriptor Table (simplified - just log for now)
        log::debug!("SIDT instruction executed");
        Ok(())
    }

    pub fn execute_sldt(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid SLDT instruction".to_string()));
        }
        // Store Local Descriptor Table (simplified - just log for now)
        log::debug!("SLDT instruction executed");
        Ok(())
    }

    pub fn execute_swapgs(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Swap GS Base (simplified - just log for now)
        log::debug!("SWAPGS instruction executed");
        Ok(())
    }
}