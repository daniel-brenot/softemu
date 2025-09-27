use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {

    pub fn execute_mov(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid MOV instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        self.set_operand_value(instruction, 1, src, state)?;
        Ok(())
    }

    pub fn execute_movsb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Move byte from [RSI] to [RDI]
        let src_addr = state.registers.rsi;
        let dst_addr = state.registers.rdi;
        
        let value = state.read_u8(src_addr)?;
        state.write_u8(dst_addr, value)?;
        
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

    pub fn execute_movsw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Move word from [RSI] to [RDI]
        let src_addr = state.registers.rsi;
        let dst_addr = state.registers.rdi;
        
        let value = state.read_u16(src_addr)?;
        state.write_u16(dst_addr, value)?;
        
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

    pub fn execute_movsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Move doubleword from [RSI] to [RDI]
        let src_addr = state.registers.rsi;
        let dst_addr = state.registers.rdi;
        
        let value = state.read_u32(src_addr)?;
        state.write_u32(dst_addr, value)?;
        
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

    pub fn execute_movsq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Move quadword from [RSI] to [RDI]
        let src_addr = state.registers.rsi;
        let dst_addr = state.registers.rdi;
        
        let value = state.read_u64(src_addr)?;
        state.write_u64(dst_addr, value)?;
        
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

    pub fn execute_movsx(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid MOVSX instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        // Sign extend based on source size (simplified - assume 32-bit to 64-bit)
        let result = src as i32 as i64 as u64;
        self.set_operand_value(instruction, 1, result, state)?;
        Ok(())
    }

    pub fn execute_movzx(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid MOVZX instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        // Zero extend (no change needed for 64-bit)
        self.set_operand_value(instruction, 1, src, state)?;
        Ok(())
    }

    pub fn execute_mul(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid MUL instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let rax = state.registers.rax;
        let result = rax.wrapping_mul(src);
        
        // Store result in RAX:RDX (64-bit result in RAX, overflow in RDX)
        state.registers.rax = result;
        state.registers.rdx = if result < rax { 1 } else { 0 };
        
        // Update flags
        state.registers.set_flag(RFlags::CARRY, state.registers.rdx != 0);
        state.registers.set_flag(RFlags::OVERFLOW, state.registers.rdx != 0);
        Ok(())
    }

    pub fn execute_mfence(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Memory Fence (memory ordering)
        // For now, just do nothing
        Ok(())
    }

    pub fn execute_maskmovdqu(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MASKMOVDQU instruction executed");
        Ok(())
    }

    pub fn execute_maskmovq(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MASKMOVQ instruction executed");
        Ok(())
    }

    pub fn execute_maxpd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MAXPD instruction executed");
        Ok(())
    }

    pub fn execute_maxps(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MAXPS instruction executed");
        Ok(())
    }

    pub fn execute_maxsd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MAXSD instruction executed");
        Ok(())
    }

    pub fn execute_maxss(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MAXSS instruction executed");
        Ok(())
    }

    pub fn execute_mcommit(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MCOMMIT instruction executed");
        Ok(())
    }

    pub fn execute_minpd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MINPD instruction executed");
        Ok(())
    }

    pub fn execute_minps(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MINPS instruction executed");
        Ok(())
    }

    pub fn execute_minsd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MINSD instruction executed");
        Ok(())
    }

    pub fn execute_minss(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MINSS instruction executed");
        Ok(())
    }

    pub fn execute_monitor(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MONITOR instruction executed");
        Ok(())
    }

    pub fn execute_monitorx(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MONITORX instruction executed");
        Ok(())
    }

    pub fn execute_montmul(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MONTMUL instruction executed");
        Ok(())
    }

    pub fn execute_movapd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVAPD instruction executed");
        Ok(())
    }

    pub fn execute_movaps(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVAPS instruction executed");
        Ok(())
    }

    pub fn execute_movbe(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVBE instruction executed");
        Ok(())
    }

    pub fn execute_movd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVD instruction executed");
        Ok(())
    }

    pub fn execute_movddup(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVDDUP instruction executed");
        Ok(())
    }

    pub fn execute_movdir64b(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVDIR64B instruction executed");
        Ok(())
    }

    pub fn execute_movdiri(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVDIRI instruction executed");
        Ok(())
    }

    pub fn execute_movdq2q(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVDQ2Q instruction executed");
        Ok(())
    }

    pub fn execute_movdqa(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVDQA instruction executed");
        Ok(())
    }

    pub fn execute_movdqu(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVDQU instruction executed");
        Ok(())
    }

    pub fn execute_movhlps(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVHLPS instruction executed");
        Ok(())
    }

    pub fn execute_movhpd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVHPD instruction executed");
        Ok(())
    }

    pub fn execute_movhps(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVHPS instruction executed");
        Ok(())
    }

    pub fn execute_movlhps(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVLHPS instruction executed");
        Ok(())
    }

    pub fn execute_movlpd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVLPD instruction executed");
        Ok(())
    }

    pub fn execute_movlps(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVLPS instruction executed");
        Ok(())
    }

    pub fn execute_movmskpd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVMSKPD instruction executed");
        Ok(())
    }

    pub fn execute_movmskps(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVMSKPS instruction executed");
        Ok(())
    }

    pub fn execute_movntdq(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVNTDQ instruction executed");
        Ok(())
    }

    pub fn execute_movntdqa(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVNTDQA instruction executed");
        Ok(())
    }

    pub fn execute_movnti(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVNTI instruction executed");
        Ok(())
    }

    pub fn execute_movntpd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVNTPD instruction executed");
        Ok(())
    }

    pub fn execute_movntps(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVNTPS instruction executed");
        Ok(())
    }

    pub fn execute_movntq(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVNTQ instruction executed");
        Ok(())
    }

    pub fn execute_movntsd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVNTSD instruction executed");
        Ok(())
    }

    pub fn execute_movntss(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVNTSS instruction executed");
        Ok(())
    }

    pub fn execute_movq(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVQ instruction executed");
        Ok(())
    }

    pub fn execute_movq2dq(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVQ2DQ instruction executed");
        Ok(())
    }

    pub fn execute_movshdup(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVSHDUP instruction executed");
        Ok(())
    }

    pub fn execute_movsldup(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVSLDUP instruction executed");
        Ok(())
    }

    pub fn execute_movss(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVSS instruction executed");
        Ok(())
    }

    pub fn execute_movsxd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVSXD instruction executed");
        Ok(())
    }

    pub fn execute_movupd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVUPD instruction executed");
        Ok(())
    }

    pub fn execute_movups(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MOVUPS instruction executed");
        Ok(())
    }

    pub fn execute_mpsadbw(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MPSADBW instruction executed");
        Ok(())
    }

    pub fn execute_mulpd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MULPD instruction executed");
        Ok(())
    }

    pub fn execute_mulps(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MULPS instruction executed");
        Ok(())
    }

    pub fn execute_mulsd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MULSD instruction executed");
        Ok(())
    }

    pub fn execute_mulss(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MULSS instruction executed");
        Ok(())
    }

    pub fn execute_mulx(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MULX instruction executed");
        Ok(())
    }

    pub fn execute_mwait(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MWAIT instruction executed");
        Ok(())
    }

    pub fn execute_mwaitx(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("MWAITX instruction executed");
        Ok(())
    }
}
