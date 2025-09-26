use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    pub fn execute_int(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let vector = self.get_operand_value(instruction, 0, state)? as u8;
        state.trigger_interrupt(vector);
        Ok(())
    }

    pub fn execute_iret(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Pop RIP, CS, RFLAGS
        let rip = state.read_u64(state.registers.rsp)?;
        state.registers.rsp += 8;
        let cs = state.read_u16(state.registers.rsp)?;
        state.registers.rsp += 2;
        let rflags = state.read_u64(state.registers.rsp)?;
        state.registers.rsp += 8;

        state.registers.rip = rip;
        state.registers.cs = cs;
        state.registers.rflags = rflags;
        state.clear_interrupt();
        Ok(())
    }

    pub fn execute_inc(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid INC instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let result = src.wrapping_add(1);
        self.set_operand_value(instruction, 0, result, state)?;
        
        // Update flags (INC doesn't affect carry flag)
        state.registers.set_flag(RFlags::ZERO, result == 0);
        state.registers.set_flag(RFlags::SIGN, (result & 0x8000000000000000) != 0);
        state.registers.set_flag(RFlags::OVERFLOW, result == 0x8000000000000000);
        
        let low_byte = (result & 0xFF) as u8;
        let parity = low_byte.count_ones() % 2 == 0;
        state.registers.set_flag(RFlags::PARITY, parity);
        Ok(())
    }

    pub fn execute_in(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid IN instruction".to_string()));
        }

        let _port = self.get_operand_value(instruction, 0, state)? as u16;
        // For now, return 0 for all I/O operations (placeholder)
        let value = 0u64;
        self.set_operand_value(instruction, 1, value, state)?;
        Ok(())
    }

    pub fn execute_insb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Input byte from port DX to [RDI]
        let _port = (state.registers.rdx & 0xFFFF) as u16;
        let dst_addr = state.registers.rdi;
        
        // For now, write 0 (placeholder)
        state.write_u8(dst_addr, 0)?;
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rdi = state.registers.rdi.wrapping_sub(1);
        } else {
            state.registers.rdi = state.registers.rdi.wrapping_add(1);
        }
        Ok(())
    }

    pub fn execute_insw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Input word from port DX to [RDI]
        let _port = (state.registers.rdx & 0xFFFF) as u16;
        let dst_addr = state.registers.rdi;
        
        // For now, write 0 (placeholder)
        state.write_u16(dst_addr, 0)?;
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rdi = state.registers.rdi.wrapping_sub(2);
        } else {
            state.registers.rdi = state.registers.rdi.wrapping_add(2);
        }
        Ok(())
    }

    pub fn execute_insd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Input doubleword from port DX to [RDI]
        let _port = (state.registers.rdx & 0xFFFF) as u16;
        let dst_addr = state.registers.rdi;
        
        // For now, write 0 (placeholder)
        state.write_u32(dst_addr, 0)?;
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rdi = state.registers.rdi.wrapping_sub(4);
        } else {
            state.registers.rdi = state.registers.rdi.wrapping_add(4);
        }
        Ok(())
    }

    pub fn execute_imul(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid IMUL instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)? as i64;
        let rax = state.registers.rax as i64;
        let result = rax.wrapping_mul(src);
        
        // Store result in RAX:RDX (64-bit result in RAX, overflow in RDX)
        state.registers.rax = result as u64;
        state.registers.rdx = (result >> 63) as u64; // Sign extend
        
        // Update flags
        state.registers.set_flag(RFlags::CARRY, state.registers.rdx != 0);
        state.registers.set_flag(RFlags::OVERFLOW, state.registers.rdx != 0);
        Ok(())
    }

    pub fn execute_into(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Interrupt on Overflow
        if state.registers.get_flag(RFlags::OVERFLOW) {
            // Trigger interrupt 4
            self.execute_int(&Instruction::default(), state)?;
        }
        Ok(())
    }

    pub fn execute_invd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Invalidate Cache (simplified - just log for now)
        log::debug!("INVD instruction executed");
        Ok(())
    }

    pub fn execute_invlpg(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid INVLPG instruction".to_string()));
        }
        // Invalidate TLB Entry (simplified - just log for now)
        log::debug!("INVLPG instruction executed");
        Ok(())
    }
}