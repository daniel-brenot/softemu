use iced_x86::{Decoder, DecoderOptions, Instruction, OpKind};
use crate::cpu::state::CpuState;
use crate::cpu::registers::RFlags;
use crate::Result;

/// Instruction decoder and executor
pub struct InstructionDecoder<'a> {
    decoder: Decoder<'a>,
}

impl<'a> InstructionDecoder<'a> {
    pub fn new() -> Self {
        Self {
            decoder: Decoder::new(64, &[], DecoderOptions::NONE),
        }
    }

    pub fn decode_instruction(&mut self, code: &[u8]) -> Instruction {
        // Create a new decoder for each instruction
        let mut decoder = Decoder::new(64, code, DecoderOptions::NONE);
        decoder.decode()
    }

    pub fn execute_instruction(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        match instruction.mnemonic() {
            iced_x86::Mnemonic::Nop => {
                // NOP instruction - do nothing
                Ok(())
            }
            iced_x86::Mnemonic::Mov => {
                self.execute_mov(instruction, state)
            }
            iced_x86::Mnemonic::Add => {
                self.execute_add(instruction, state)
            }
            iced_x86::Mnemonic::Sub => {
                self.execute_sub(instruction, state)
            }
            iced_x86::Mnemonic::Jmp => {
                self.execute_jmp(instruction, state)
            }
            iced_x86::Mnemonic::Call => {
                self.execute_call(instruction, state)
            }
            iced_x86::Mnemonic::Ret => {
                self.execute_ret(instruction, state)
            }
            iced_x86::Mnemonic::Push => {
                self.execute_push(instruction, state)
            }
            iced_x86::Mnemonic::Pop => {
                self.execute_pop(instruction, state)
            }
            iced_x86::Mnemonic::Hlt => {
                state.halt();
                Ok(())
            }
            iced_x86::Mnemonic::Int => {
                self.execute_int(instruction, state)
            }
            iced_x86::Mnemonic::Iret => {
                self.execute_iret(instruction, state)
            }
            // Additional arithmetic instructions
            iced_x86::Mnemonic::Cmp => {
                self.execute_cmp(instruction, state)
            }
            iced_x86::Mnemonic::Inc => {
                self.execute_inc(instruction, state)
            }
            iced_x86::Mnemonic::Dec => {
                self.execute_dec(instruction, state)
            }
            // Logical instructions
            iced_x86::Mnemonic::And => {
                self.execute_and(instruction, state)
            }
            iced_x86::Mnemonic::Or => {
                self.execute_or(instruction, state)
            }
            iced_x86::Mnemonic::Xor => {
                self.execute_xor(instruction, state)
            }
            iced_x86::Mnemonic::Test => {
                self.execute_test(instruction, state)
            }
            // Conditional jumps
            iced_x86::Mnemonic::Je => {
                self.execute_je(instruction, state)
            }
            iced_x86::Mnemonic::Jne => {
                self.execute_jne(instruction, state)
            }
            iced_x86::Mnemonic::Jl => {
                self.execute_jl(instruction, state)
            }
            iced_x86::Mnemonic::Jle => {
                self.execute_jle(instruction, state)
            }
            iced_x86::Mnemonic::Jg => {
                self.execute_jg(instruction, state)
            }
            iced_x86::Mnemonic::Jge => {
                self.execute_jge(instruction, state)
            }
            iced_x86::Mnemonic::Jb => {
                self.execute_jb(instruction, state)
            }
            iced_x86::Mnemonic::Jbe => {
                self.execute_jbe(instruction, state)
            }
            iced_x86::Mnemonic::Ja => {
                self.execute_ja(instruction, state)
            }
            iced_x86::Mnemonic::Jae => {
                self.execute_jae(instruction, state)
            }
            iced_x86::Mnemonic::Js => {
                self.execute_js(instruction, state)
            }
            iced_x86::Mnemonic::Jns => {
                self.execute_jns(instruction, state)
            }
            iced_x86::Mnemonic::Jo => {
                self.execute_jo(instruction, state)
            }
            iced_x86::Mnemonic::Jno => {
                self.execute_jno(instruction, state)
            }
            // Loop instructions
            iced_x86::Mnemonic::Loop => {
                self.execute_loop(instruction, state)
            }
            iced_x86::Mnemonic::Loope => {
                self.execute_loope(instruction, state)
            }
            iced_x86::Mnemonic::Loopne => {
                self.execute_loopne(instruction, state)
            }
            iced_x86::Mnemonic::Jcxz => {
                self.execute_jcxz(instruction, state)
            }
            iced_x86::Mnemonic::Jecxz => {
                self.execute_jecxz(instruction, state)
            }
            iced_x86::Mnemonic::Jrcxz => {
                self.execute_jrcxz(instruction, state)
            }
            _ => {
                // Unimplemented instruction
                log::warn!("Unimplemented instruction: {:?}", instruction.mnemonic());
                Ok(())
            }
        }
    }

    fn execute_mov(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid MOV instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        self.set_operand_value(instruction, 1, src, state)?;
        Ok(())
    }

    fn execute_add(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_sub(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_jmp(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let target = self.get_operand_value(instruction, 0, state)?;
        state.registers.rip = target;
        Ok(())
    }

    fn execute_call(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let target = self.get_operand_value(instruction, 0, state)?;
        let return_addr = state.registers.rip + instruction.len() as u64;
        
        // Push return address
        state.registers.rsp -= 8;
        state.write_u64(state.registers.rsp, return_addr)?;
        
        // Jump to target
        state.registers.rip = target;
        Ok(())
    }

    fn execute_ret(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Pop return address
        let return_addr = state.read_u64(state.registers.rsp)?;
        state.registers.rsp += 8;
        
        // Jump to return address
        state.registers.rip = return_addr;
        Ok(())
    }

    fn execute_push(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let value = self.get_operand_value(instruction, 0, state)?;
        state.registers.rsp -= 8;
        state.write_u64(state.registers.rsp, value)?;
        Ok(())
    }

    fn execute_pop(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let value = state.read_u64(state.registers.rsp)?;
        state.registers.rsp += 8;
        self.set_operand_value(instruction, 0, value, state)?;
        Ok(())
    }

    fn execute_int(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let vector = self.get_operand_value(instruction, 0, state)? as u8;
        state.trigger_interrupt(vector);
        Ok(())
    }

    fn execute_iret(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn get_operand_value(&self, instruction: &Instruction, op_index: u32, state: &CpuState) -> Result<u64> {
        let operand = instruction.try_op_kind(op_index).unwrap();
        match operand {
            OpKind::Register => {
                let reg = instruction.op_register(op_index);
                Ok(self.get_register_value(reg, state))
            }
            OpKind::Immediate8 => Ok(instruction.immediate8() as u64),
            OpKind::Immediate16 => Ok(instruction.immediate16() as u64),
            OpKind::Immediate32 => Ok(instruction.immediate32() as u64),
            OpKind::Immediate64 => Ok(instruction.immediate64()),
            OpKind::Memory => {
                let addr = self.calculate_memory_address(instruction, op_index, state)?;
                state.read_u64(addr)
            }
            _ => Err(crate::EmulatorError::Cpu("Unsupported operand kind".to_string())),
        }
    }

    fn set_operand_value(&self, instruction: &Instruction, op_index: u32, value: u64, state: &mut CpuState) -> Result<()> {
        let operand = instruction.try_op_kind(op_index).unwrap();
        match operand {
            OpKind::Register => {
                let reg = instruction.op_register(op_index);
                self.set_register_value(reg, value, state);
                Ok(())
            }
            OpKind::Memory => {
                let addr = self.calculate_memory_address(instruction, op_index, state)?;
                state.write_u64(addr, value)
            }
            _ => Err(crate::EmulatorError::Cpu("Cannot set value to this operand".to_string())),
        }
    }

    fn get_register_value(&self, reg: iced_x86::Register, state: &CpuState) -> u64 {
        match reg {
            iced_x86::Register::RAX => state.registers.rax,
            iced_x86::Register::RBX => state.registers.rbx,
            iced_x86::Register::RCX => state.registers.rcx,
            iced_x86::Register::RDX => state.registers.rdx,
            iced_x86::Register::RSI => state.registers.rsi,
            iced_x86::Register::RDI => state.registers.rdi,
            iced_x86::Register::RBP => state.registers.rbp,
            iced_x86::Register::RSP => state.registers.rsp,
            iced_x86::Register::R8 => state.registers.r8,
            iced_x86::Register::R9 => state.registers.r9,
            iced_x86::Register::R10 => state.registers.r10,
            iced_x86::Register::R11 => state.registers.r11,
            iced_x86::Register::R12 => state.registers.r12,
            iced_x86::Register::R13 => state.registers.r13,
            iced_x86::Register::R14 => state.registers.r14,
            iced_x86::Register::R15 => state.registers.r15,
            iced_x86::Register::RIP => state.registers.rip,
            _ => 0,
        }
    }

    fn set_register_value(&self, reg: iced_x86::Register, value: u64, state: &mut CpuState) {
        match reg {
            iced_x86::Register::RAX => state.registers.rax = value,
            iced_x86::Register::RBX => state.registers.rbx = value,
            iced_x86::Register::RCX => state.registers.rcx = value,
            iced_x86::Register::RDX => state.registers.rdx = value,
            iced_x86::Register::RSI => state.registers.rsi = value,
            iced_x86::Register::RDI => state.registers.rdi = value,
            iced_x86::Register::RBP => state.registers.rbp = value,
            iced_x86::Register::RSP => state.registers.rsp = value,
            iced_x86::Register::R8 => state.registers.r8 = value,
            iced_x86::Register::R9 => state.registers.r9 = value,
            iced_x86::Register::R10 => state.registers.r10 = value,
            iced_x86::Register::R11 => state.registers.r11 = value,
            iced_x86::Register::R12 => state.registers.r12 = value,
            iced_x86::Register::R13 => state.registers.r13 = value,
            iced_x86::Register::R14 => state.registers.r14 = value,
            iced_x86::Register::R15 => state.registers.r15 = value,
            iced_x86::Register::RIP => state.registers.rip = value,
            _ => {}
        }
    }

    fn calculate_memory_address(&self, instruction: &Instruction, op_index: u32, state: &CpuState) -> Result<u64> {
        // Simplified memory address calculation
        // In a real implementation, this would properly handle all addressing modes
        let mut addr = 0u64;

        // For now, just use a simple base address
        // This is a placeholder implementation
        if instruction.op_count() > op_index {
            // Try to get a register value as base
            if let Ok(reg) = instruction.try_op_register(op_index) {
                addr = self.get_register_value(reg, state);
            }
        }

        Ok(addr)
    }

    fn update_arithmetic_flags(&self, result: u64, src: u64, dst: u64, is_subtraction: bool, state: &mut CpuState) {
        let _flags = state.registers.get_flags();
        
        // Zero flag
        state.registers.set_flag(RFlags::ZERO, result == 0);
        
        // Sign flag (MSB)
        state.registers.set_flag(RFlags::SIGN, (result & 0x8000000000000000) != 0);
        
        // Carry flag
        let carry = if is_subtraction {
            dst < src
        } else {
            result < dst
        };
        state.registers.set_flag(RFlags::CARRY, carry);
        
        // Overflow flag
        let overflow = if is_subtraction {
            ((dst ^ src) & (dst ^ result)) & 0x8000000000000000 != 0
        } else {
            ((dst ^ src) & (dst ^ result)) & 0x8000000000000000 != 0
        };
        state.registers.set_flag(RFlags::OVERFLOW, overflow);
        
        // Parity flag (even number of set bits in low byte)
        let low_byte = (result & 0xFF) as u8;
        let parity = low_byte.count_ones() % 2 == 0;
        state.registers.set_flag(RFlags::PARITY, parity);
    }

    // Additional instruction implementations
    fn execute_cmp(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_inc(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_dec(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid DEC instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let result = src.wrapping_sub(1);
        self.set_operand_value(instruction, 0, result, state)?;
        
        // Update flags (DEC doesn't affect carry flag)
        state.registers.set_flag(RFlags::ZERO, result == 0);
        state.registers.set_flag(RFlags::SIGN, (result & 0x8000000000000000) != 0);
        state.registers.set_flag(RFlags::OVERFLOW, result == 0x7FFFFFFFFFFFFFFF);
        
        let low_byte = (result & 0xFF) as u8;
        let parity = low_byte.count_ones() % 2 == 0;
        state.registers.set_flag(RFlags::PARITY, parity);
        Ok(())
    }

    fn execute_and(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_or(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid OR instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst | src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    fn execute_xor(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid XOR instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst ^ src;

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_logical_flags(result, state);
        Ok(())
    }

    fn execute_test(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid TEST instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let result = dst & src;

        // Update flags (TEST doesn't store result)
        self.update_logical_flags(result, state);
        Ok(())
    }

    fn update_logical_flags(&self, result: u64, state: &mut CpuState) {
        state.registers.set_flag(RFlags::ZERO, result == 0);
        state.registers.set_flag(RFlags::SIGN, (result & 0x8000000000000000) != 0);
        state.registers.set_flag(RFlags::CARRY, false);
        state.registers.set_flag(RFlags::OVERFLOW, false);
        
        let low_byte = (result & 0xFF) as u8;
        let parity = low_byte.count_ones() % 2 == 0;
        state.registers.set_flag(RFlags::PARITY, parity);
    }

    // Conditional jump implementations
    fn execute_je(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if state.registers.get_flag(RFlags::ZERO) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    fn execute_jne(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if !state.registers.get_flag(RFlags::ZERO) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    fn execute_jl(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let sign = state.registers.get_flag(RFlags::SIGN);
        let overflow = state.registers.get_flag(RFlags::OVERFLOW);
        if sign != overflow {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    fn execute_jle(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let zero = state.registers.get_flag(RFlags::ZERO);
        let sign = state.registers.get_flag(RFlags::SIGN);
        let overflow = state.registers.get_flag(RFlags::OVERFLOW);
        if zero || (sign != overflow) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    fn execute_jg(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let zero = state.registers.get_flag(RFlags::ZERO);
        let sign = state.registers.get_flag(RFlags::SIGN);
        let overflow = state.registers.get_flag(RFlags::OVERFLOW);
        if !zero && (sign == overflow) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    fn execute_jge(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let sign = state.registers.get_flag(RFlags::SIGN);
        let overflow = state.registers.get_flag(RFlags::OVERFLOW);
        if sign == overflow {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    fn execute_jb(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if state.registers.get_flag(RFlags::CARRY) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    fn execute_jbe(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let carry = state.registers.get_flag(RFlags::CARRY);
        let zero = state.registers.get_flag(RFlags::ZERO);
        if carry || zero {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    fn execute_ja(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        let carry = state.registers.get_flag(RFlags::CARRY);
        let zero = state.registers.get_flag(RFlags::ZERO);
        if !carry && !zero {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    fn execute_jae(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if !state.registers.get_flag(RFlags::CARRY) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    fn execute_js(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if state.registers.get_flag(RFlags::SIGN) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    fn execute_jns(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if !state.registers.get_flag(RFlags::SIGN) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    fn execute_jo(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if state.registers.get_flag(RFlags::OVERFLOW) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    fn execute_jno(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if !state.registers.get_flag(RFlags::OVERFLOW) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    // Loop instruction implementations
    fn execute_loop(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LOOP instruction".to_string()));
        }

        // Decrement RCX (counter register)
        state.registers.rcx = state.registers.rcx.wrapping_sub(1);
        
        // If RCX != 0, jump to target
        if state.registers.rcx != 0 {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    fn execute_loope(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LOOPE instruction".to_string()));
        }

        // Decrement RCX (counter register)
        state.registers.rcx = state.registers.rcx.wrapping_sub(1);
        
        // If RCX != 0 AND zero flag is set, jump to target
        if state.registers.rcx != 0 && state.registers.get_flag(RFlags::ZERO) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    fn execute_loopne(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LOOPNE instruction".to_string()));
        }

        // Decrement RCX (counter register)
        state.registers.rcx = state.registers.rcx.wrapping_sub(1);
        
        // If RCX != 0 AND zero flag is clear, jump to target
        if state.registers.rcx != 0 && !state.registers.get_flag(RFlags::ZERO) {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    fn execute_jcxz(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid JCXZ instruction".to_string()));
        }

        // Jump if CX (16-bit part of RCX) is zero
        if (state.registers.rcx & 0xFFFF) == 0 {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    fn execute_jecxz(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid JECXZ instruction".to_string()));
        }

        // Jump if ECX (32-bit part of RCX) is zero
        if (state.registers.rcx & 0xFFFFFFFF) == 0 {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }

    fn execute_jrcxz(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid JRCXZ instruction".to_string()));
        }

        // Jump if RCX (64-bit) is zero
        if state.registers.rcx == 0 {
            let target = self.get_operand_value(instruction, 0, state)?;
            state.registers.rip = target;
        }
        Ok(())
    }
}
