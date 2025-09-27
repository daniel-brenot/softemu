use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    pub fn execute_loop(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LOOP instruction".to_string()));
        }

        // Decrement RCX (counter register)
        state.registers.rcx = state.registers.rcx.wrapping_sub(1);
        
        // If RCX != 0, jump to target
        if state.registers.rcx != 0 {
            // LOOP uses relative addressing - get the immediate value and add to current RIP
            let operand = instruction.try_op_kind(0).unwrap();
            let target = match operand {
                iced_x86::OpKind::Immediate8 => {
                    let offset = instruction.immediate8() as i8 as i64;
                    (state.registers.rip as i64 + offset) as u64
                },
                iced_x86::OpKind::Immediate16 => (state.registers.rip as i64 + instruction.immediate16() as i16 as i64) as u64,
                iced_x86::OpKind::Immediate32 => (state.registers.rip as i64 + instruction.immediate32() as i32 as i64) as u64,
                iced_x86::OpKind::NearBranch16 => instruction.near_branch16() as u64,
                iced_x86::OpKind::NearBranch32 => instruction.near_branch32() as u64,
                iced_x86::OpKind::NearBranch64 => {
                    let offset = instruction.near_branch64() as i64;
                    (state.registers.rip as i64 + offset) as u64
                },
                _ => return Err(crate::EmulatorError::Cpu("Invalid LOOP operand".to_string())),
            };
            state.registers.rip = target;
        } else {
            // If RCX == 0, advance RIP by instruction length
            state.registers.rip += instruction.len() as u64;
        }
        Ok(())
    }

    pub fn execute_loope(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LOOPE instruction".to_string()));
        }

        // Decrement RCX (counter register)
        state.registers.rcx = state.registers.rcx.wrapping_sub(1);
        
        // If RCX != 0 AND zero flag is set, jump to target
        if state.registers.rcx != 0 && state.registers.get_flag(RFlags::ZERO) {
            // LOOPE uses relative addressing - get the immediate value and add to current RIP
            let operand = instruction.try_op_kind(0).unwrap();
            let target = match operand {
                iced_x86::OpKind::Immediate8 => (state.registers.rip as i64 + instruction.immediate8() as i8 as i64) as u64,
                iced_x86::OpKind::Immediate16 => (state.registers.rip as i64 + instruction.immediate16() as i16 as i64) as u64,
                iced_x86::OpKind::Immediate32 => (state.registers.rip as i64 + instruction.immediate32() as i32 as i64) as u64,
                iced_x86::OpKind::NearBranch16 => instruction.near_branch16() as u64,
                iced_x86::OpKind::NearBranch32 => instruction.near_branch32() as u64,
                iced_x86::OpKind::NearBranch64 => {
                    let offset = instruction.near_branch64() as i64;
                    (state.registers.rip as i64 + offset) as u64
                },
                _ => return Err(crate::EmulatorError::Cpu("Invalid LOOPE operand".to_string())),
            };
            state.registers.rip = target;
        } else {
            // If RCX == 0 or zero flag is clear, advance RIP by instruction length
            state.registers.rip += instruction.len() as u64;
        }
        Ok(())
    }

    pub fn execute_loopne(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LOOPNE instruction".to_string()));
        }

        // Decrement RCX (counter register)
        state.registers.rcx = state.registers.rcx.wrapping_sub(1);
        
        // If RCX != 0 AND zero flag is clear, jump to target
        if state.registers.rcx != 0 && !state.registers.get_flag(RFlags::ZERO) {
            // LOOPNE uses relative addressing - get the immediate value and add to current RIP
            let operand = instruction.try_op_kind(0).unwrap();
            let target = match operand {
                iced_x86::OpKind::Immediate8 => (state.registers.rip as i64 + instruction.immediate8() as i8 as i64) as u64,
                iced_x86::OpKind::Immediate16 => (state.registers.rip as i64 + instruction.immediate16() as i16 as i64) as u64,
                iced_x86::OpKind::Immediate32 => (state.registers.rip as i64 + instruction.immediate32() as i32 as i64) as u64,
                iced_x86::OpKind::NearBranch16 => instruction.near_branch16() as u64,
                iced_x86::OpKind::NearBranch32 => instruction.near_branch32() as u64,
                iced_x86::OpKind::NearBranch64 => {
                    let offset = instruction.near_branch64() as i64;
                    (state.registers.rip as i64 + offset) as u64
                },
                _ => return Err(crate::EmulatorError::Cpu("Invalid LOOPNE operand".to_string())),
            };
            state.registers.rip = target;
        } else {
            // If RCX == 0 or zero flag is set, advance RIP by instruction length
            state.registers.rip += instruction.len() as u64;
        }
        Ok(())
    }

    pub fn execute_lodsb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Load byte from [RSI] into AL
        let src_addr = state.registers.rsi;
        
        let value = state.read_u8(src_addr)?;
        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFFFF00) | (value as u64);
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(1);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(1);
        }
        Ok(())
    }

    pub fn execute_lodsw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Load word from [RSI] into AX
        let src_addr = state.registers.rsi;
        
        let value = state.read_u16(src_addr)?;
        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFF0000) | (value as u64);
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(2);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(2);
        }
        Ok(())
    }

    pub fn execute_lodsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Load doubleword from [RSI] into EAX
        let src_addr = state.registers.rsi;
        
        let value = state.read_u32(src_addr)?;
        state.registers.rax = (state.registers.rax & 0xFFFFFFFF00000000) | (value as u64);
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(4);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(4);
        }
        Ok(())
    }

    pub fn execute_lodsq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Load quadword from [RSI] into RAX
        let src_addr = state.registers.rsi;
        
        let value = state.read_u64(src_addr)?;
        state.registers.rax = value;
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(8);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(8);
        }
        Ok(())
    }

    pub fn execute_lgdt(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LGDT instruction".to_string()));
        }
        // Load Global Descriptor Table (simplified - just log for now)
        log::debug!("LGDT instruction executed");
        Ok(())
    }

    pub fn execute_lidt(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LIDT instruction".to_string()));
        }
        // Load Interrupt Descriptor Table (simplified - just log for now)
        log::debug!("LIDT instruction executed");
        Ok(())
    }

    pub fn execute_ltr(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LTR instruction".to_string()));
        }
        // Load Task Register (simplified - just log for now)
        log::debug!("LTR instruction executed");
        Ok(())
    }

    pub fn execute_lmsw(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LMSW instruction".to_string()));
        }
        // Load Machine Status Word (simplified - just log for now)
        log::debug!("LMSW instruction executed");
        Ok(())
    }

    pub fn execute_lahf(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Load flags into AH register
        // LAHF loads the lower 8 bits of the flags register into AH
        // The flags are: SF(7), ZF(6), AF(4), PF(2), CF(0)
        let mut ah_value = 0u8;
        if state.registers.get_flag(RFlags::SIGN) { ah_value |= 0x80; }      // SF
        if state.registers.get_flag(RFlags::ZERO) { ah_value |= 0x40; }     // ZF
        if state.registers.get_flag(RFlags::AUXILIARY) { ah_value |= 0x10; } // AF
        if state.registers.get_flag(RFlags::PARITY) { ah_value |= 0x04; }   // PF
        if state.registers.get_flag(RFlags::CARRY) { ah_value |= 0x01; }    // CF
        
        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFF00FF) | ((ah_value as u64) << 8);
        Ok(())
    }

    pub fn execute_lds(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LDS instruction".to_string()));
        }
        // Load DS segment and offset (simplified - just log for now)
        log::debug!("LDS instruction executed");
        Ok(())
    }

    pub fn execute_les(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LES instruction".to_string()));
        }
        // Load ES segment and offset (simplified - just log for now)
        log::debug!("LES instruction executed");
        Ok(())
    }

    pub fn execute_lfs(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LFS instruction".to_string()));
        }
        // Load FS segment and offset (simplified - just log for now)
        log::debug!("LFS instruction executed");
        Ok(())
    }

    pub fn execute_lgs(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LGS instruction".to_string()));
        }
        // Load GS segment and offset (simplified - just log for now)
        log::debug!("LGS instruction executed");
        Ok(())
    }

    pub fn execute_lss(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LSS instruction".to_string()));
        }
        // Load SS segment and offset (simplified - just log for now)
        log::debug!("LSS instruction executed");
        Ok(())
    }

    pub fn execute_lea(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LEA instruction".to_string()));
        }

        // LEA loads the effective address of the source operand into the destination
        // We need to calculate the effective address without actually accessing memory
        let effective_addr = self.calculate_memory_address(instruction, 1, state)?;
        self.set_operand_value(instruction, 0, effective_addr, state)?;
        Ok(())
    }

    pub fn execute_leave(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Leave procedure (opposite of ENTER)
        state.registers.rsp = state.registers.rbp;
        state.registers.rbp = state.read_u64(state.registers.rsp)?;
        state.registers.rsp += 8;
        Ok(())
    }

    pub fn execute_lfence(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Load Fence (memory ordering)
        // For now, just do nothing
        Ok(())
    }

    pub fn execute_lldt(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LLDT instruction".to_string()));
        }
        // Load Local Descriptor Table (simplified - just log for now)
        log::debug!("LLDT instruction executed");
        Ok(())
    }

    pub fn execute_lar(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LAR instruction".to_string()));
        }
        // Load Access Rights (simplified - just log for now)
        log::debug!("LAR instruction executed");
        Ok(())
    }

    pub fn execute_lsl(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LSL instruction".to_string()));
        }
        // Load Segment Limit (simplified - just log for now)
        log::debug!("LSL instruction executed");
        Ok(())
    }

    pub fn execute_lddqu(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("LDDQU instruction executed");
        Ok(())
    }

    pub fn execute_ldmxcsr(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("LDMXCSR instruction executed");
        Ok(())
    }

    pub fn execute_llwpcb(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("LLWPCB instruction executed");
        Ok(())
    }

    pub fn execute_loadall(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("LOADALL instruction executed");
        Ok(())
    }

    pub fn execute_lwpins(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("LWPINS instruction executed");
        Ok(())
    }

    pub fn execute_lwpval(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("LWPVAL instruction executed");
        Ok(())
    }

    pub fn execute_lzcnt(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("LZCNT instruction executed");
        Ok(())
    }

    pub fn execute_ldtilecfg(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("LDTILECFG instruction executed");
        Ok(())
    }

    pub fn execute_loadiwkey(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("LOADIWKEY instruction executed");
        Ok(())
    }

    pub fn execute_lkgs(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        log::debug!("LKGS instruction executed");
        Ok(())
    }
}
