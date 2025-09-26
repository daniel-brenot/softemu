use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    pub fn execute_call(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let target = self.get_operand_value(instruction, 0, state)?;
        let return_addr = state.registers.rip + instruction.len() as u64;
        
        // Push return address
        state.registers.rsp -= 8;
        state.write_u64(state.registers.rsp, return_addr)?;
        
        // Jump to target
        state.registers.rip = target;
        Ok(())
    }

    pub fn execute_cmp(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid CMP instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst.wrapping_sub(src);

        // Update flags (CMP doesn't store result)
        self.update_arithmetic_flags(result, src, dst, true, state);
        Ok(())
    }

    pub fn execute_cmpsb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Compare byte at [RSI] with byte at [RDI]
        let src_addr = state.registers.rsi;
        let dst_addr = state.registers.rdi;
        
        let src_value = state.read_u8(src_addr)? as u64;
        let dst_value = state.read_u8(dst_addr)? as u64;
        let result = dst_value.wrapping_sub(src_value);
        
        // Update flags
        self.update_arithmetic_flags(result, src_value, dst_value, true, state);
        
        // Update pointers based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(1);
            state.registers.rdi = state.registers.rdi.wrapping_sub(1);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(1);
            state.registers.rdi = state.registers.rdi.wrapping_add(1);
        }
        Ok(())
    }

    pub fn execute_cmpsw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Compare word at [RSI] with word at [RDI]
        let src_addr = state.registers.rsi;
        let dst_addr = state.registers.rdi;
        
        let src_value = state.read_u16(src_addr)? as u64;
        let dst_value = state.read_u16(dst_addr)? as u64;
        let result = dst_value.wrapping_sub(src_value);
        
        // Update flags
        self.update_arithmetic_flags(result, src_value, dst_value, true, state);
        
        // Update pointers based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(2);
            state.registers.rdi = state.registers.rdi.wrapping_sub(2);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(2);
            state.registers.rdi = state.registers.rdi.wrapping_add(2);
        }
        Ok(())
    }

    pub fn execute_cmpsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Compare doubleword at [RSI] with doubleword at [RDI]
        let src_addr = state.registers.rsi;
        let dst_addr = state.registers.rdi;
        
        let src_value = state.read_u32(src_addr)? as u64;
        let dst_value = state.read_u32(dst_addr)? as u64;
        let result = dst_value.wrapping_sub(src_value);
        
        // Update flags
        self.update_arithmetic_flags(result, src_value, dst_value, true, state);
        
        // Update pointers based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(4);
            state.registers.rdi = state.registers.rdi.wrapping_sub(4);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(4);
            state.registers.rdi = state.registers.rdi.wrapping_add(4);
        }
        Ok(())
    }

    pub fn execute_cmpsq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Compare quadword at [RSI] with quadword at [RDI]
        let src_addr = state.registers.rsi;
        let dst_addr = state.registers.rdi;
        
        let src_value = state.read_u64(src_addr)?;
        let dst_value = state.read_u64(dst_addr)?;
        let result = dst_value.wrapping_sub(src_value);
        
        // Update flags
        self.update_arithmetic_flags(result, src_value, dst_value, true, state);
        
        // Update pointers based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(8);
            state.registers.rdi = state.registers.rdi.wrapping_sub(8);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(8);
            state.registers.rdi = state.registers.rdi.wrapping_add(8);
        }
        Ok(())
    }

    pub fn execute_cli(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Clear interrupt flag
        state.registers.set_flag(RFlags::INTERRUPT, false);
        Ok(())
    }

    pub fn execute_cmpxchg(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid CMPXCHG instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let accumulator = state.registers.rax;

        if accumulator == dst {
            self.set_operand_value(instruction, 1, src, state)?;
            state.registers.set_flag(RFlags::ZERO, true);
        } else {
            state.registers.rax = dst;
            state.registers.set_flag(RFlags::ZERO, false);
        }
        Ok(())
    }

    pub fn execute_cqo(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Convert Quadword to Octaword (sign extend RAX to RDX:RAX)
        if (state.registers.rax & 0x8000000000000000) != 0 {
            state.registers.rdx = 0xFFFFFFFFFFFFFFFF;
        } else {
            state.registers.rdx = 0;
        }
        Ok(())
    }

    pub fn execute_clc(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Clear carry flag
        state.registers.set_flag(RFlags::CARRY, false);
        Ok(())
    }

    pub fn execute_cmc(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Complement carry flag
        let current = state.registers.get_flag(RFlags::CARRY);
        state.registers.set_flag(RFlags::CARRY, !current);
        Ok(())
    }

    pub fn execute_cld(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Clear direction flag
        state.registers.set_flag(RFlags::DIRECTION, false);
        Ok(())
    }

    pub fn execute_cpuid(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 0 {
            return Err(crate::EmulatorError::Cpu("Invalid CPUID instruction".to_string()));
        }

        // Simplified CPUID implementation
        let eax = state.registers.rax as u32;
        match eax {
            0 => {
                // Maximum input value for basic CPUID information
                state.registers.rax = 0x00000001;
                state.registers.rbx = 0x68747541; // "Auth"
                state.registers.rcx = 0x444D4163; // "cAMD"
                state.registers.rdx = 0x69746E65; // "enti"
            }
            1 => {
                // Processor info and feature flags
                state.registers.rax = 0x00000F61; // Family, Model, Stepping
                state.registers.rbx = 0x00000000;
                state.registers.rcx = 0x00000000; // Feature flags
                state.registers.rdx = 0x00000000; // Feature flags
            }
            _ => {
                // Unsupported CPUID leaf
                state.registers.rax = 0;
                state.registers.rbx = 0;
                state.registers.rcx = 0;
                state.registers.rdx = 0;
            }
        }
        Ok(())
    }

    pub fn execute_cbw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Convert Byte to Word (sign extend AL to AX)
        let al = (state.registers.rax & 0xFF) as i8;
        let ax = al as i16;
        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFF0000) | (ax as u16 as u64);
        Ok(())
    }

    pub fn execute_cdq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Convert Doubleword to Quadword (sign extend EAX to EDX:EAX)
        let eax = (state.registers.rax & 0xFFFFFFFF) as i32;
        state.registers.rdx = (eax >> 31) as u64;
        Ok(())
    }

    pub fn execute_cdqe(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Convert Doubleword to Quadword (sign extend EAX to RAX)
        let eax = (state.registers.rax & 0xFFFFFFFF) as i32;
        state.registers.rax = eax as u64;
        Ok(())
    }

    pub fn execute_clts(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Clear Task-Switched Flag (simplified - just log for now)
        log::debug!("CLTS instruction executed");
        Ok(())
    }

    pub fn execute_cwd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Convert Word to Doubleword (sign extend AX to DX:AX)
        let ax = (state.registers.rax & 0xFFFF) as i16;
        let dx = (ax >> 15) as u16;
        state.registers.rdx = (state.registers.rdx & 0xFFFFFFFFFFFF0000) | (dx as u64);
        Ok(())
    }

    pub fn execute_cwde(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Convert Word to Extended Doubleword (sign extend AX to EAX)
        let ax = (state.registers.rax & 0xFFFF) as i16;
        state.registers.rax = (state.registers.rax & 0xFFFFFFFF00000000) | (ax as u32 as u64);
        Ok(())
    }

    // Additional C instruction implementations
    pub fn execute_clac(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Clear AC flag in EFLAGS
        log::debug!("CLAC instruction executed");
        Ok(())
    }

    pub fn execute_cldemote(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Cache line demote
        log::debug!("CLDEMOTE instruction executed");
        Ok(())
    }

    pub fn execute_clflush(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Flush cache line
        log::debug!("CLFLUSH instruction executed");
        Ok(())
    }

    pub fn execute_clflushopt(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Flush cache line (optimized)
        log::debug!("CLFLUSHOPT instruction executed");
        Ok(())
    }

    pub fn execute_clgi(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Clear global interrupt flag
        log::debug!("CLGI instruction executed");
        Ok(())
    }

    pub fn execute_clrssbsy(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Clear busy flag in a supervisor shadow stack token
        log::debug!("CLRSSBSY instruction executed");
        Ok(())
    }

    pub fn execute_clui(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Clear user interrupt flag
        log::debug!("CLUI instruction executed");
        Ok(())
    }

    pub fn execute_clwb(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Cache line write back
        log::debug!("CLWB instruction executed");
        Ok(())
    }

    pub fn execute_clzero(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Cache line zero
        log::debug!("CLZERO instruction executed");
        Ok(())
    }

    // CMOV instructions (conditional moves)
    pub fn execute_cmova(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid CMOVA instruction".to_string()));
        }
        if !state.registers.get_flag(RFlags::CARRY) && !state.registers.get_flag(RFlags::ZERO) {
            let src = self.get_operand_value(instruction, 0, state)?;
            self.set_operand_value(instruction, 1, src, state)?;
        }
        Ok(())
    }

    pub fn execute_cmovae(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid CMOVAE instruction".to_string()));
        }
        if !state.registers.get_flag(RFlags::CARRY) {
            let src = self.get_operand_value(instruction, 0, state)?;
            self.set_operand_value(instruction, 1, src, state)?;
        }
        Ok(())
    }

    pub fn execute_cmovb(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid CMOVB instruction".to_string()));
        }
        if state.registers.get_flag(RFlags::CARRY) {
            let src = self.get_operand_value(instruction, 0, state)?;
            self.set_operand_value(instruction, 1, src, state)?;
        }
        Ok(())
    }

    pub fn execute_cmovbe(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid CMOVBE instruction".to_string()));
        }
        if state.registers.get_flag(RFlags::CARRY) || state.registers.get_flag(RFlags::ZERO) {
            let src = self.get_operand_value(instruction, 0, state)?;
            self.set_operand_value(instruction, 1, src, state)?;
        }
        Ok(())
    }

    pub fn execute_cmove(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid CMOVE instruction".to_string()));
        }
        if state.registers.get_flag(RFlags::ZERO) {
            let src = self.get_operand_value(instruction, 0, state)?;
            self.set_operand_value(instruction, 1, src, state)?;
        }
        Ok(())
    }

    pub fn execute_cmovg(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid CMOVG instruction".to_string()));
        }
        if !state.registers.get_flag(RFlags::ZERO) && 
           (state.registers.get_flag(RFlags::SIGN) == state.registers.get_flag(RFlags::OVERFLOW)) {
            let src = self.get_operand_value(instruction, 0, state)?;
            self.set_operand_value(instruction, 1, src, state)?;
        }
        Ok(())
    }

    pub fn execute_cmovge(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid CMOVGE instruction".to_string()));
        }
        if state.registers.get_flag(RFlags::SIGN) == state.registers.get_flag(RFlags::OVERFLOW) {
            let src = self.get_operand_value(instruction, 0, state)?;
            self.set_operand_value(instruction, 1, src, state)?;
        }
        Ok(())
    }

    pub fn execute_cmovl(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid CMOVL instruction".to_string()));
        }
        if state.registers.get_flag(RFlags::SIGN) != state.registers.get_flag(RFlags::OVERFLOW) {
            let src = self.get_operand_value(instruction, 0, state)?;
            self.set_operand_value(instruction, 1, src, state)?;
        }
        Ok(())
    }

    pub fn execute_cmovle(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid CMOVLE instruction".to_string()));
        }
        if state.registers.get_flag(RFlags::ZERO) || 
           (state.registers.get_flag(RFlags::SIGN) != state.registers.get_flag(RFlags::OVERFLOW)) {
            let src = self.get_operand_value(instruction, 0, state)?;
            self.set_operand_value(instruction, 1, src, state)?;
        }
        Ok(())
    }

    pub fn execute_cmovne(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid CMOVNE instruction".to_string()));
        }
        if !state.registers.get_flag(RFlags::ZERO) {
            let src = self.get_operand_value(instruction, 0, state)?;
            self.set_operand_value(instruction, 1, src, state)?;
        }
        Ok(())
    }

    pub fn execute_cmovno(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid CMOVNO instruction".to_string()));
        }
        if !state.registers.get_flag(RFlags::OVERFLOW) {
            let src = self.get_operand_value(instruction, 0, state)?;
            self.set_operand_value(instruction, 1, src, state)?;
        }
        Ok(())
    }

    pub fn execute_cmovnp(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid CMOVNP instruction".to_string()));
        }
        if !state.registers.get_flag(RFlags::PARITY) {
            let src = self.get_operand_value(instruction, 0, state)?;
            self.set_operand_value(instruction, 1, src, state)?;
        }
        Ok(())
    }

    pub fn execute_cmovns(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid CMOVNS instruction".to_string()));
        }
        if !state.registers.get_flag(RFlags::SIGN) {
            let src = self.get_operand_value(instruction, 0, state)?;
            self.set_operand_value(instruction, 1, src, state)?;
        }
        Ok(())
    }

    pub fn execute_cmovo(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid CMOVO instruction".to_string()));
        }
        if state.registers.get_flag(RFlags::OVERFLOW) {
            let src = self.get_operand_value(instruction, 0, state)?;
            self.set_operand_value(instruction, 1, src, state)?;
        }
        Ok(())
    }

    pub fn execute_cmovp(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid CMOVP instruction".to_string()));
        }
        if state.registers.get_flag(RFlags::PARITY) {
            let src = self.get_operand_value(instruction, 0, state)?;
            self.set_operand_value(instruction, 1, src, state)?;
        }
        Ok(())
    }

    pub fn execute_cmovs(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid CMOVS instruction".to_string()));
        }
        if state.registers.get_flag(RFlags::SIGN) {
            let src = self.get_operand_value(instruction, 0, state)?;
            self.set_operand_value(instruction, 1, src, state)?;
        }
        Ok(())
    }

    // CMPXADD instructions (compare and exchange add)
    pub fn execute_cmpbexadd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("CMPBEXADD instruction executed");
        Ok(())
    }

    pub fn execute_cmpbxadd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("CMPBXADD instruction executed");
        Ok(())
    }

    pub fn execute_cmplexadd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("CMPLEXADD instruction executed");
        Ok(())
    }

    pub fn execute_cmplxadd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("CMPLXADD instruction executed");
        Ok(())
    }

    pub fn execute_cmpnbexadd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("CMPNBEXADD instruction executed");
        Ok(())
    }

    pub fn execute_cmpnbxadd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("CMPNBXADD instruction executed");
        Ok(())
    }

    pub fn execute_cmpnlexadd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("CMPNLEXADD instruction executed");
        Ok(())
    }

    pub fn execute_cmpnlxadd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("CMPNLXADD instruction executed");
        Ok(())
    }

    pub fn execute_cmpnoxadd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("CMPNOXADD instruction executed");
        Ok(())
    }

    pub fn execute_cmpnpxadd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("CMPNPXADD instruction executed");
        Ok(())
    }

    pub fn execute_cmpnsxadd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("CMPNSXADD instruction executed");
        Ok(())
    }

    pub fn execute_cmpnzxadd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("CMPNZXADD instruction executed");
        Ok(())
    }

    pub fn execute_cmpoxadd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("CMPOXADD instruction executed");
        Ok(())
    }

    pub fn execute_cmppxadd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("CMPPXADD instruction executed");
        Ok(())
    }

    pub fn execute_cmpsxadd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("CMPSXADD instruction executed");
        Ok(())
    }

    pub fn execute_cmpzxadd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("CMPZXADD instruction executed");
        Ok(())
    }

    // SIMD compare instructions
    pub fn execute_cmppd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("CMPPD instruction executed");
        Ok(())
    }

    pub fn execute_cmpps(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("CMPPS instruction executed");
        Ok(())
    }

    pub fn execute_cmpss(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("CMPSS instruction executed");
        Ok(())
    }

    // Compare scalar double/single precision
    pub fn execute_comisd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("COMISD instruction executed");
        Ok(())
    }

    pub fn execute_comiss(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("COMISS instruction executed");
        Ok(())
    }
}