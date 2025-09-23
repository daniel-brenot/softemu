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

    fn execute_ret(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_iret(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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
        let flags = state.registers.get_flags();
        
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
}
