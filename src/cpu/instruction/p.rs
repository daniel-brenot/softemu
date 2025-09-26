use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    pub fn execute_push(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let value = self.get_operand_value(instruction, 0, state)?;
        state.registers.rsp -= 8;
        state.write_u64(state.registers.rsp, value)?;
        Ok(())
    }

    pub fn execute_pop(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let value = state.read_u64(state.registers.rsp)?;
        state.registers.rsp += 8;
        self.set_operand_value(instruction, 0, value, state)?;
        Ok(())
    }

    pub fn execute_pusha(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Push all 16-bit registers (AX, CX, DX, BX, SP, BP, SI, DI)
        let ax = (state.registers.rax & 0xFFFF) as u16;
        let cx = (state.registers.rcx & 0xFFFF) as u16;
        let dx = (state.registers.rdx & 0xFFFF) as u16;
        let bx = (state.registers.rbx & 0xFFFF) as u16;
        let sp = (state.registers.rsp & 0xFFFF) as u16;
        let bp = (state.registers.rbp & 0xFFFF) as u16;
        let si = (state.registers.rsi & 0xFFFF) as u16;
        let di = (state.registers.rdi & 0xFFFF) as u16;

        // Push in reverse order
        state.registers.rsp -= 2; state.write_u16(state.registers.rsp, di)?;
        state.registers.rsp -= 2; state.write_u16(state.registers.rsp, si)?;
        state.registers.rsp -= 2; state.write_u16(state.registers.rsp, bp)?;
        state.registers.rsp -= 2; state.write_u16(state.registers.rsp, sp)?;
        state.registers.rsp -= 2; state.write_u16(state.registers.rsp, bx)?;
        state.registers.rsp -= 2; state.write_u16(state.registers.rsp, dx)?;
        state.registers.rsp -= 2; state.write_u16(state.registers.rsp, cx)?;
        state.registers.rsp -= 2; state.write_u16(state.registers.rsp, ax)?;
        Ok(())
    }

    pub fn execute_popa(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Pop all 16-bit registers (AX, CX, DX, BX, SP, BP, SI, DI)
        let ax = state.read_u16(state.registers.rsp)?; state.registers.rsp += 2;
        let cx = state.read_u16(state.registers.rsp)?; state.registers.rsp += 2;
        let dx = state.read_u16(state.registers.rsp)?; state.registers.rsp += 2;
        let bx = state.read_u16(state.registers.rsp)?; state.registers.rsp += 2;
        let sp = state.read_u16(state.registers.rsp)?; state.registers.rsp += 2;
        let bp = state.read_u16(state.registers.rsp)?; state.registers.rsp += 2;
        let si = state.read_u16(state.registers.rsp)?; state.registers.rsp += 2;
        let di = state.read_u16(state.registers.rsp)?; state.registers.rsp += 2;

        // Update registers (preserve upper bits)
        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFF0000) | (ax as u64);
        state.registers.rcx = (state.registers.rcx & 0xFFFFFFFFFFFF0000) | (cx as u64);
        state.registers.rdx = (state.registers.rdx & 0xFFFFFFFFFFFF0000) | (dx as u64);
        state.registers.rbx = (state.registers.rbx & 0xFFFFFFFFFFFF0000) | (bx as u64);
        state.registers.rsp = (state.registers.rsp & 0xFFFFFFFFFFFF0000) | (sp as u64);
        state.registers.rbp = (state.registers.rbp & 0xFFFFFFFFFFFF0000) | (bp as u64);
        state.registers.rsi = (state.registers.rsi & 0xFFFFFFFFFFFF0000) | (si as u64);
        state.registers.rdi = (state.registers.rdi & 0xFFFFFFFFFFFF0000) | (di as u64);
        Ok(())
    }

    pub fn execute_pushad(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Push all 32-bit registers (EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI)
        let eax = (state.registers.rax & 0xFFFFFFFF) as u32;
        let ecx = (state.registers.rcx & 0xFFFFFFFF) as u32;
        let edx = (state.registers.rdx & 0xFFFFFFFF) as u32;
        let ebx = (state.registers.rbx & 0xFFFFFFFF) as u32;
        let esp = (state.registers.rsp & 0xFFFFFFFF) as u32;
        let ebp = (state.registers.rbp & 0xFFFFFFFF) as u32;
        let esi = (state.registers.rsi & 0xFFFFFFFF) as u32;
        let edi = (state.registers.rdi & 0xFFFFFFFF) as u32;

        // Push in reverse order
        state.registers.rsp -= 4; state.write_u32(state.registers.rsp, edi)?;
        state.registers.rsp -= 4; state.write_u32(state.registers.rsp, esi)?;
        state.registers.rsp -= 4; state.write_u32(state.registers.rsp, ebp)?;
        state.registers.rsp -= 4; state.write_u32(state.registers.rsp, esp)?;
        state.registers.rsp -= 4; state.write_u32(state.registers.rsp, ebx)?;
        state.registers.rsp -= 4; state.write_u32(state.registers.rsp, edx)?;
        state.registers.rsp -= 4; state.write_u32(state.registers.rsp, ecx)?;
        state.registers.rsp -= 4; state.write_u32(state.registers.rsp, eax)?;
        Ok(())
    }

    pub fn execute_popad(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Pop all 32-bit registers (EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI)
        let eax = state.read_u32(state.registers.rsp)?; state.registers.rsp += 4;
        let ecx = state.read_u32(state.registers.rsp)?; state.registers.rsp += 4;
        let edx = state.read_u32(state.registers.rsp)?; state.registers.rsp += 4;
        let ebx = state.read_u32(state.registers.rsp)?; state.registers.rsp += 4;
        let esp = state.read_u32(state.registers.rsp)?; state.registers.rsp += 4;
        let ebp = state.read_u32(state.registers.rsp)?; state.registers.rsp += 4;
        let esi = state.read_u32(state.registers.rsp)?; state.registers.rsp += 4;
        let edi = state.read_u32(state.registers.rsp)?; state.registers.rsp += 4;

        // Update registers (preserve upper bits)
        state.registers.rax = (state.registers.rax & 0xFFFFFFFF00000000) | (eax as u64);
        state.registers.rcx = (state.registers.rcx & 0xFFFFFFFF00000000) | (ecx as u64);
        state.registers.rdx = (state.registers.rdx & 0xFFFFFFFF00000000) | (edx as u64);
        state.registers.rbx = (state.registers.rbx & 0xFFFFFFFF00000000) | (ebx as u64);
        state.registers.rsp = (state.registers.rsp & 0xFFFFFFFF00000000) | (esp as u64);
        state.registers.rbp = (state.registers.rbp & 0xFFFFFFFF00000000) | (ebp as u64);
        state.registers.rsi = (state.registers.rsi & 0xFFFFFFFF00000000) | (esi as u64);
        state.registers.rdi = (state.registers.rdi & 0xFFFFFFFF00000000) | (edi as u64);
        Ok(())
    }

    pub fn execute_pushf(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Push flags register
        state.registers.rsp -= 8;
        state.write_u64(state.registers.rsp, state.registers.rflags)?;
        Ok(())
    }

    pub fn execute_popf(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Pop flags register
        let flags = state.read_u64(state.registers.rsp)?;
        state.registers.rsp += 8;
        state.registers.rflags = flags;
        Ok(())
    }

    pub fn execute_popcnt(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid POPCNT instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let count = src.count_ones() as u64;
        
        // Update flags
        state.registers.set_flag(RFlags::ZERO, count == 0);
        state.registers.set_flag(RFlags::CARRY, false);
        state.registers.set_flag(RFlags::OVERFLOW, false);
        state.registers.set_flag(RFlags::SIGN, false);
        
        self.set_operand_value(instruction, 1, count, state)?;
        Ok(())
    }

    pub fn execute_pause(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Pause instruction (for spin loops)
        // For now, just do nothing
        Ok(())
    }
}