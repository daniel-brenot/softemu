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
            iced_x86::Mnemonic::INVALID => {
                Err(crate::EmulatorError::Cpu("Invalid instruction".to_string()))
            }
            iced_x86::Mnemonic::Aaa => {
                self.execute_aaa(instruction, state)
            }
            iced_x86::Mnemonic::Aad => {
                self.execute_aad(instruction, state)
            }
            iced_x86::Mnemonic::Aam => {
                self.execute_aam(instruction, state)
            }
            iced_x86::Mnemonic::Aas => {
                self.execute_aas(instruction, state)
            }
            iced_x86::Mnemonic::Adc => {
                self.execute_adc(instruction, state)
            }
            iced_x86::Mnemonic::Add => {
                self.execute_add(instruction, state)
            }
            iced_x86::Mnemonic::And => {
                self.execute_and(instruction, state)
            }
            iced_x86::Mnemonic::Arpl => {
                self.execute_arpl(instruction, state)
            }
            iced_x86::Mnemonic::Bsf => {
                self.execute_bsf(instruction, state)
            }
            iced_x86::Mnemonic::Bsr => {
                self.execute_bsr(instruction, state)
            }
            iced_x86::Mnemonic::Bswap => {
                self.execute_bswap(instruction, state)
            }
            iced_x86::Mnemonic::Bt => {
                self.execute_bt(instruction, state)
            }
            iced_x86::Mnemonic::Btc => {
                self.execute_btc(instruction, state)
            }
            iced_x86::Mnemonic::Btr => {
                self.execute_btr(instruction, state)
            }
            iced_x86::Mnemonic::Bts => {
                self.execute_bts(instruction, state)
            }
            iced_x86::Mnemonic::Call => {
                self.execute_call(instruction, state)
            }
            iced_x86::Mnemonic::Cbw => {
                self.execute_cbw(instruction, state)
            }
            iced_x86::Mnemonic::Cdq => {
                self.execute_cdq(instruction, state)
            }
            iced_x86::Mnemonic::Cdqe => {
                self.execute_cdqe(instruction, state)
            }
            iced_x86::Mnemonic::Clc => {
                self.execute_clc(instruction, state)
            }
            iced_x86::Mnemonic::Cld => {
                self.execute_cld(instruction, state)
            }
            iced_x86::Mnemonic::Cli => {
                self.execute_cli(instruction, state)
            }
            iced_x86::Mnemonic::Clts => {
                self.execute_clts(instruction, state)
            }
            iced_x86::Mnemonic::Cmc => {
                self.execute_cmc(instruction, state)
            }
            iced_x86::Mnemonic::Cmp => {
                self.execute_cmp(instruction, state)
            }
            iced_x86::Mnemonic::Cmpsb => {
                self.execute_cmpsb(instruction, state)
            }
            iced_x86::Mnemonic::Cmpsd => {
                self.execute_cmpsd(instruction, state)
            }
            iced_x86::Mnemonic::Cmpsq => {
                self.execute_cmpsq(instruction, state)
            }
            iced_x86::Mnemonic::Cmpsw => {
                self.execute_cmpsw(instruction, state)
            }
            iced_x86::Mnemonic::Cmpxchg => {
                self.execute_cmpxchg(instruction, state)
            }
            iced_x86::Mnemonic::Cpuid => {
                self.execute_cpuid(instruction, state)
            }
            iced_x86::Mnemonic::Cqo => {
                self.execute_cqo(instruction, state)
            }
            iced_x86::Mnemonic::Cwd => {
                self.execute_cwd(instruction, state)
            }
            iced_x86::Mnemonic::Cwde => {
                self.execute_cwde(instruction, state)
            }
            iced_x86::Mnemonic::Daa => {
                self.execute_daa(instruction, state)
            }
            iced_x86::Mnemonic::Das => {
                self.execute_das(instruction, state)
            }
            iced_x86::Mnemonic::Dec => {
                self.execute_dec(instruction, state)
            }
            iced_x86::Mnemonic::Div => {
                self.execute_div(instruction, state)
            }
            iced_x86::Mnemonic::Enter => {
                self.execute_enter(instruction, state)
            }
            iced_x86::Mnemonic::Hlt => {
                state.halt();
                Ok(())
            }
            iced_x86::Mnemonic::Idiv => {
                self.execute_idiv(instruction, state)
            }
            iced_x86::Mnemonic::Imul => {
                self.execute_imul(instruction, state)
            }
            iced_x86::Mnemonic::In => {
                self.execute_in(instruction, state)
            }
            iced_x86::Mnemonic::Inc => {
                self.execute_inc(instruction, state)
            }
            iced_x86::Mnemonic::Insb => {
                self.execute_insb(instruction, state)
            }
            iced_x86::Mnemonic::Insd => {
                self.execute_insd(instruction, state)
            }
            iced_x86::Mnemonic::Insw => {
                self.execute_insw(instruction, state)
            }
            iced_x86::Mnemonic::Int => {
                self.execute_int(instruction, state)
            }
            iced_x86::Mnemonic::Into => {
                self.execute_into(instruction, state)
            }
            iced_x86::Mnemonic::Invd => {
                self.execute_invd(instruction, state)
            }
            iced_x86::Mnemonic::Invlpg => {
                self.execute_invlpg(instruction, state)
            }
            iced_x86::Mnemonic::Iret => {
                self.execute_iret(instruction, state)
            }
            iced_x86::Mnemonic::Ja => {
                self.execute_ja(instruction, state)
            }
            iced_x86::Mnemonic::Jae => {
                self.execute_jae(instruction, state)
            }
            iced_x86::Mnemonic::Jb => {
                self.execute_jb(instruction, state)
            }
            iced_x86::Mnemonic::Jbe => {
                self.execute_jbe(instruction, state)
            }
            iced_x86::Mnemonic::Jcxz => {
                self.execute_jcxz(instruction, state)
            }
            iced_x86::Mnemonic::Je => {
                self.execute_je(instruction, state)
            }
            iced_x86::Mnemonic::Jecxz => {
                self.execute_jecxz(instruction, state)
            }
            iced_x86::Mnemonic::Jg => {
                self.execute_jg(instruction, state)
            }
            iced_x86::Mnemonic::Jge => {
                self.execute_jge(instruction, state)
            }
            iced_x86::Mnemonic::Jl => {
                self.execute_jl(instruction, state)
            }
            iced_x86::Mnemonic::Jle => {
                self.execute_jle(instruction, state)
            }
            iced_x86::Mnemonic::Jmp => {
                self.execute_jmp(instruction, state)
            }
            iced_x86::Mnemonic::Jne => {
                self.execute_jne(instruction, state)
            }
            iced_x86::Mnemonic::Jno => {
                self.execute_jno(instruction, state)
            }
            iced_x86::Mnemonic::Jns => {
                self.execute_jns(instruction, state)
            }
            iced_x86::Mnemonic::Jo => {
                self.execute_jo(instruction, state)
            }
            iced_x86::Mnemonic::Jrcxz => {
                self.execute_jrcxz(instruction, state)
            }
            iced_x86::Mnemonic::Js => {
                self.execute_js(instruction, state)
            }
            iced_x86::Mnemonic::Lahf => {
                self.execute_lahf(instruction, state)
            }
            iced_x86::Mnemonic::Lar => {
                self.execute_lar(instruction, state)
            }
            iced_x86::Mnemonic::Lds => {
                self.execute_lds(instruction, state)
            }
            iced_x86::Mnemonic::Lea => {
                self.execute_lea(instruction, state)
            }
            iced_x86::Mnemonic::Leave => {
                self.execute_leave(instruction, state)
            }
            iced_x86::Mnemonic::Les => {
                self.execute_les(instruction, state)
            }
            iced_x86::Mnemonic::Lfence => {
                self.execute_lfence(instruction, state)
            }
            iced_x86::Mnemonic::Lfs => {
                self.execute_lfs(instruction, state)
            }
            iced_x86::Mnemonic::Lgdt => {
                self.execute_lgdt(instruction, state)
            }
            iced_x86::Mnemonic::Lgs => {
                self.execute_lgs(instruction, state)
            }
            iced_x86::Mnemonic::Lidt => {
                self.execute_lidt(instruction, state)
            }
            iced_x86::Mnemonic::Lldt => {
                self.execute_lldt(instruction, state)
            }
            iced_x86::Mnemonic::Lmsw => {
                self.execute_lmsw(instruction, state)
            }
            iced_x86::Mnemonic::Lodsb => {
                self.execute_lodsb(instruction, state)
            }
            iced_x86::Mnemonic::Lodsd => {
                self.execute_lodsd(instruction, state)
            }
            iced_x86::Mnemonic::Lodsq => {
                self.execute_lodsq(instruction, state)
            }
            iced_x86::Mnemonic::Lodsw => {
                self.execute_lodsw(instruction, state)
            }
            iced_x86::Mnemonic::Loop => {
                self.execute_loop(instruction, state)
            }
            iced_x86::Mnemonic::Loope => {
                self.execute_loope(instruction, state)
            }
            iced_x86::Mnemonic::Loopne => {
                self.execute_loopne(instruction, state)
            }
            iced_x86::Mnemonic::Lsl => {
                self.execute_lsl(instruction, state)
            }
            iced_x86::Mnemonic::Lss => {
                self.execute_lss(instruction, state)
            }
            iced_x86::Mnemonic::Ltr => {
                self.execute_ltr(instruction, state)
            }
            iced_x86::Mnemonic::Mfence => {
                self.execute_mfence(instruction, state)
            }
            iced_x86::Mnemonic::Mov => {
                self.execute_mov(instruction, state)
            }
            iced_x86::Mnemonic::Movsb => {
                self.execute_movsb(instruction, state)
            }
            iced_x86::Mnemonic::Movsd => {
                self.execute_movsd(instruction, state)
            }
            iced_x86::Mnemonic::Movsq => {
                self.execute_movsq(instruction, state)
            }
            iced_x86::Mnemonic::Movsw => {
                self.execute_movsw(instruction, state)
            }
            iced_x86::Mnemonic::Movsx => {
                self.execute_movsx(instruction, state)
            }
            iced_x86::Mnemonic::Movzx => {
                self.execute_movzx(instruction, state)
            }
            iced_x86::Mnemonic::Mul => {
                self.execute_mul(instruction, state)
            }
            iced_x86::Mnemonic::Neg => {
                self.execute_neg(instruction, state)
            }
            iced_x86::Mnemonic::Nop => {
                // NOP instruction - do nothing
                Ok(())
            }
            iced_x86::Mnemonic::Not => {
                self.execute_not(instruction, state)
            }
            iced_x86::Mnemonic::Or => {
                self.execute_or(instruction, state)
            }
            iced_x86::Mnemonic::Out => {
                self.execute_out(instruction, state)
            }
            iced_x86::Mnemonic::Outsb => {
                self.execute_outsb(instruction, state)
            }
            iced_x86::Mnemonic::Outsd => {
                self.execute_outsd(instruction, state)
            }
            iced_x86::Mnemonic::Outsw => {
                self.execute_outsw(instruction, state)
            }
            iced_x86::Mnemonic::Pause => {
                self.execute_pause(instruction, state)
            }
            iced_x86::Mnemonic::Pop => {
                self.execute_pop(instruction, state)
            }
            iced_x86::Mnemonic::Popa => {
                self.execute_popa(instruction, state)
            }
            iced_x86::Mnemonic::Popcnt => {
                self.execute_popcnt(instruction, state)
            }
            iced_x86::Mnemonic::Popf => {
                self.execute_popf(instruction, state)
            }
            iced_x86::Mnemonic::Push => {
                self.execute_push(instruction, state)
            }
            iced_x86::Mnemonic::Pusha => {
                self.execute_pusha(instruction, state)
            }
            iced_x86::Mnemonic::Pushf => {
                self.execute_pushf(instruction, state)
            }
            iced_x86::Mnemonic::Rcl => {
                self.execute_rcl(instruction, state)
            }
            iced_x86::Mnemonic::Rcr => {
                self.execute_rcr(instruction, state)
            }
            iced_x86::Mnemonic::Rdmsr => {
                self.execute_rdmsr(instruction, state)
            }
            iced_x86::Mnemonic::Rdpmc => {
                self.execute_rdpmc(instruction, state)
            }
            iced_x86::Mnemonic::Rdrand => {
                self.execute_rdrand(instruction, state)
            }
            iced_x86::Mnemonic::Rdseed => {
                self.execute_rdseed(instruction, state)
            }
            iced_x86::Mnemonic::Rdtsc => {
                self.execute_rdtsc(instruction, state)
            }
            iced_x86::Mnemonic::Rdtscp => {
                self.execute_rdtscp(instruction, state)
            }
            iced_x86::Mnemonic::Ret => {
                self.execute_ret(instruction, state)
            }
            iced_x86::Mnemonic::Rol => {
                self.execute_rol(instruction, state)
            }
            iced_x86::Mnemonic::Ror => {
                self.execute_ror(instruction, state)
            }
            iced_x86::Mnemonic::Rsm => {
                self.execute_rsm(instruction, state)
            }
            iced_x86::Mnemonic::Sahf => {
                self.execute_sahf(instruction, state)
            }
            iced_x86::Mnemonic::Sal => {
                self.execute_shl(instruction, state)
            }
            iced_x86::Mnemonic::Salc => {
                self.execute_salc(instruction, state)
            }
            iced_x86::Mnemonic::Sar => {
                self.execute_sar(instruction, state)
            }
            iced_x86::Mnemonic::Sbb => {
                self.execute_sbb(instruction, state)
            }
            iced_x86::Mnemonic::Scasb => {
                self.execute_scasb(instruction, state)
            }
            iced_x86::Mnemonic::Scasd => {
                self.execute_scasd(instruction, state)
            }
            iced_x86::Mnemonic::Scasq => {
                self.execute_scasq(instruction, state)
            }
            iced_x86::Mnemonic::Scasw => {
                self.execute_scasw(instruction, state)
            }
            iced_x86::Mnemonic::Sfence => {
                self.execute_sfence(instruction, state)
            }
            iced_x86::Mnemonic::Sgdt => {
                self.execute_sgdt(instruction, state)
            }
            iced_x86::Mnemonic::Shl => {
                self.execute_shl(instruction, state)
            }
            iced_x86::Mnemonic::Shr => {
                self.execute_shr(instruction, state)
            }
            iced_x86::Mnemonic::Sidt => {
                self.execute_sidt(instruction, state)
            }
            iced_x86::Mnemonic::Sldt => {
                self.execute_sldt(instruction, state)
            }
            iced_x86::Mnemonic::Smsw => {
                self.execute_smsw(instruction, state)
            }
            iced_x86::Mnemonic::Stc => {
                self.execute_stc(instruction, state)
            }
            iced_x86::Mnemonic::Std => {
                self.execute_std(instruction, state)
            }
            iced_x86::Mnemonic::Sti => {
                self.execute_sti(instruction, state)
            }
            iced_x86::Mnemonic::Stosb => {
                self.execute_stosb(instruction, state)
            }
            iced_x86::Mnemonic::Stosd => {
                self.execute_stosd(instruction, state)
            }
            iced_x86::Mnemonic::Stosq => {
                self.execute_stosq(instruction, state)
            }
            iced_x86::Mnemonic::Stosw => {
                self.execute_stosw(instruction, state)
            }
            iced_x86::Mnemonic::Str => {
                self.execute_str(instruction, state)
            }
            iced_x86::Mnemonic::Sub => {
                self.execute_sub(instruction, state)
            }
            iced_x86::Mnemonic::Swapgs => {
                self.execute_swapgs(instruction, state)
            }
            iced_x86::Mnemonic::Syscall => {
                self.execute_syscall(instruction, state)
            }
            iced_x86::Mnemonic::Sysret => {
                self.execute_sysret(instruction, state)
            }
            iced_x86::Mnemonic::Test => {
                self.execute_test(instruction, state)
            }
            iced_x86::Mnemonic::Verr => {
                self.execute_verr(instruction, state)
            }
            iced_x86::Mnemonic::Verw => {
                self.execute_verw(instruction, state)
            }
            iced_x86::Mnemonic::Wbinvd => {
                self.execute_wbinvd(instruction, state)
            }
            iced_x86::Mnemonic::Wrmsr => {
                self.execute_wrmsr(instruction, state)
            }
            iced_x86::Mnemonic::Xchg => {
                self.execute_xchg(instruction, state)
            }
            iced_x86::Mnemonic::Xlatb => {
                self.execute_xlat(instruction, state)
            }
            iced_x86::Mnemonic::Xor => {
                self.execute_xor(instruction, state)
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

    // String instruction implementations
    fn execute_movsb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_movsw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_movsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_movsq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_cmpsb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_cmpsw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_cmpsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_cmpsq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_scasb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_scasw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_scasd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_scasq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_lodsb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_lodsw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_lodsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_lodsq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_stosb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_stosw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_stosd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_stosq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    // Additional control flow instructions
    fn execute_syscall(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_sysret(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    // I/O instructions
    fn execute_in(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid IN instruction".to_string()));
        }

        let _port = self.get_operand_value(instruction, 0, state)? as u16;
        // For now, return 0 for all I/O operations (placeholder)
        let value = 0u64;
        self.set_operand_value(instruction, 1, value, state)?;
        Ok(())
    }

    fn execute_out(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid OUT instruction".to_string()));
        }

        let port = self.get_operand_value(instruction, 0, state)? as u16;
        let value = self.get_operand_value(instruction, 1, state)?;
        // For now, ignore I/O operations (placeholder)
        log::debug!("OUT to port 0x{:x}: 0x{:x}", port, value);
        Ok(())
    }

    fn execute_insb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_insw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_insd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_outsb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Output byte from [RSI] to port DX
        let port = (state.registers.rdx & 0xFFFF) as u16;
        let src_addr = state.registers.rsi;
        
        let value = state.read_u8(src_addr)?;
        log::debug!("OUTSB to port 0x{:x}: 0x{:x}", port, value);
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(1);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(1);
        }
        Ok(())
    }

    fn execute_outsw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Output word from [RSI] to port DX
        let port = (state.registers.rdx & 0xFFFF) as u16;
        let src_addr = state.registers.rsi;
        
        let value = state.read_u16(src_addr)?;
        log::debug!("OUTSW to port 0x{:x}: 0x{:x}", port, value);
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(2);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(2);
        }
        Ok(())
    }

    fn execute_outsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Output doubleword from [RSI] to port DX
        let port = (state.registers.rdx & 0xFFFF) as u16;
        let src_addr = state.registers.rsi;
        
        let value = state.read_u32(src_addr)?;
        log::debug!("OUTSD to port 0x{:x}: 0x{:x}", port, value);
        
        // Update pointer based on direction flag
        if state.registers.get_flag(RFlags::DIRECTION) {
            state.registers.rsi = state.registers.rsi.wrapping_sub(4);
        } else {
            state.registers.rsi = state.registers.rsi.wrapping_add(4);
        }
        Ok(())
    }

    // System instructions
    fn execute_cli(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Clear interrupt flag
        state.registers.set_flag(RFlags::INTERRUPT, false);
        Ok(())
    }

    fn execute_sti(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Set interrupt flag
        state.registers.set_flag(RFlags::INTERRUPT, true);
        Ok(())
    }

    fn execute_lgdt(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LGDT instruction".to_string()));
        }
        // Load Global Descriptor Table (simplified - just log for now)
        log::debug!("LGDT instruction executed");
        Ok(())
    }

    fn execute_lidt(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LIDT instruction".to_string()));
        }
        // Load Interrupt Descriptor Table (simplified - just log for now)
        log::debug!("LIDT instruction executed");
        Ok(())
    }

    fn execute_ltr(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LTR instruction".to_string()));
        }
        // Load Task Register (simplified - just log for now)
        log::debug!("LTR instruction executed");
        Ok(())
    }

    fn execute_str(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid STR instruction".to_string()));
        }
        // Store Task Register (simplified - just log for now)
        log::debug!("STR instruction executed");
        Ok(())
    }

    fn execute_lmsw(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LMSW instruction".to_string()));
        }
        // Load Machine Status Word (simplified - just log for now)
        log::debug!("LMSW instruction executed");
        Ok(())
    }

    fn execute_smsw(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid SMSW instruction".to_string()));
        }
        // Store Machine Status Word (simplified - just log for now)
        log::debug!("SMSW instruction executed");
        Ok(())
    }

    // Register operations
    fn execute_pusha(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_popa(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_pushad(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_popad(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_lahf(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Load flags into AH register
        let flags = state.registers.get_flags();
        let ah_value = (flags.bits() & 0xFF) as u8;
        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFFFF00) | (ah_value as u64);
        Ok(())
    }

    fn execute_sahf(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Store AH register into flags
        let ah_value = (state.registers.rax >> 8) & 0xFF;
        let current_flags = state.registers.get_flags();
        let new_flags = RFlags::from_bits_truncate((current_flags.bits() & 0xFFFFFFFFFFFFFF00) | ah_value);
        state.registers.set_flags(new_flags);
        Ok(())
    }

    fn execute_pushf(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Push flags register
        state.registers.rsp -= 8;
        state.write_u64(state.registers.rsp, state.registers.rflags)?;
        Ok(())
    }

    fn execute_popf(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Pop flags register
        let flags = state.read_u64(state.registers.rsp)?;
        state.registers.rsp += 8;
        state.registers.rflags = flags;
        Ok(())
    }

    // Segment operations
    fn execute_lds(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LDS instruction".to_string()));
        }
        // Load DS segment and offset (simplified - just log for now)
        log::debug!("LDS instruction executed");
        Ok(())
    }

    fn execute_les(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LES instruction".to_string()));
        }
        // Load ES segment and offset (simplified - just log for now)
        log::debug!("LES instruction executed");
        Ok(())
    }

    fn execute_lfs(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LFS instruction".to_string()));
        }
        // Load FS segment and offset (simplified - just log for now)
        log::debug!("LFS instruction executed");
        Ok(())
    }

    fn execute_lgs(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LGS instruction".to_string()));
        }
        // Load GS segment and offset (simplified - just log for now)
        log::debug!("LGS instruction executed");
        Ok(())
    }

    fn execute_lss(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LSS instruction".to_string()));
        }
        // Load SS segment and offset (simplified - just log for now)
        log::debug!("LSS instruction executed");
        Ok(())
    }

    fn execute_movsx(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid MOVSX instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        // Sign extend based on source size (simplified - assume 32-bit to 64-bit)
        let result = src as i32 as i64 as u64;
        self.set_operand_value(instruction, 1, result, state)?;
        Ok(())
    }

    fn execute_movzx(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid MOVZX instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        // Zero extend (no change needed for 64-bit)
        self.set_operand_value(instruction, 1, src, state)?;
        Ok(())
    }

    // Bit manipulation instructions
    fn execute_bt(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BT instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let bit_index = self.get_operand_value(instruction, 1, state)?;
        let bit_pos = bit_index & 0x3F; // Mask to 6 bits for 64-bit
        
        let bit_value = (src >> bit_pos) & 1;
        state.registers.set_flag(RFlags::CARRY, bit_value != 0);
        Ok(())
    }

    fn execute_btc(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BTC instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let bit_index = self.get_operand_value(instruction, 1, state)?;
        let bit_pos = bit_index & 0x3F; // Mask to 6 bits for 64-bit
        
        let bit_value = (src >> bit_pos) & 1;
        state.registers.set_flag(RFlags::CARRY, bit_value != 0);
        
        // Toggle the bit
        let result = src ^ (1 << bit_pos);
        self.set_operand_value(instruction, 0, result, state)?;
        Ok(())
    }

    fn execute_btr(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BTR instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let bit_index = self.get_operand_value(instruction, 1, state)?;
        let bit_pos = bit_index & 0x3F; // Mask to 6 bits for 64-bit
        
        let bit_value = (src >> bit_pos) & 1;
        state.registers.set_flag(RFlags::CARRY, bit_value != 0);
        
        // Clear the bit
        let result = src & !(1 << bit_pos);
        self.set_operand_value(instruction, 0, result, state)?;
        Ok(())
    }

    fn execute_bts(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BTS instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let bit_index = self.get_operand_value(instruction, 1, state)?;
        let bit_pos = bit_index & 0x3F; // Mask to 6 bits for 64-bit
        
        let bit_value = (src >> bit_pos) & 1;
        state.registers.set_flag(RFlags::CARRY, bit_value != 0);
        
        // Set the bit
        let result = src | (1 << bit_pos);
        self.set_operand_value(instruction, 0, result, state)?;
        Ok(())
    }

    fn execute_bsf(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BSF instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        
        if src == 0 {
            state.registers.set_flag(RFlags::ZERO, true);
        } else {
            state.registers.set_flag(RFlags::ZERO, false);
            let bit_index = src.trailing_zeros() as u64;
            self.set_operand_value(instruction, 1, bit_index, state)?;
        }
        Ok(())
    }

    fn execute_bsr(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid BSR instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        
        if src == 0 {
            state.registers.set_flag(RFlags::ZERO, true);
        } else {
            state.registers.set_flag(RFlags::ZERO, false);
            let bit_index = 63 - src.leading_zeros() as u64;
            self.set_operand_value(instruction, 1, bit_index, state)?;
        }
        Ok(())
    }

    fn execute_popcnt(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    // Helper methods for flag updates
    fn update_shift_flags(&self, result: u64, _src: u64, count: u64, state: &mut CpuState) {
        // Update flags for shift operations
        state.registers.set_flag(RFlags::ZERO, result == 0);
        state.registers.set_flag(RFlags::SIGN, (result & 0x8000000000000000) != 0);
        state.registers.set_flag(RFlags::PARITY, (result.count_ones() & 1) == 0);
        
        // Carry flag is set to the last bit shifted out
        if count > 0 {
            let last_bit = (_src >> (count - 1)) & 1;
            state.registers.set_flag(RFlags::CARRY, last_bit != 0);
        }
        
        // Overflow flag is undefined for shift operations
        state.registers.set_flag(RFlags::OVERFLOW, false);
    }

    fn update_rotate_flags(&self, result: u64, _src: u64, count: u64, state: &mut CpuState) {
        // Update flags for rotate operations
        state.registers.set_flag(RFlags::ZERO, result == 0);
        state.registers.set_flag(RFlags::SIGN, (result & 0x8000000000000000) != 0);
        state.registers.set_flag(RFlags::PARITY, (result.count_ones() & 1) == 0);
        
        // Carry flag is set to the last bit rotated out
        if count > 0 {
            let last_bit = (_src >> (count - 1)) & 1;
            state.registers.set_flag(RFlags::CARRY, last_bit != 0);
        }
        
        // Overflow flag is undefined for rotate operations
        state.registers.set_flag(RFlags::OVERFLOW, false);
    }

    // Additional missing instruction implementations
    fn execute_xchg(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid XCHG instruction".to_string()));
        }

        let val1 = self.get_operand_value(instruction, 0, state)?;
        let val2 = self.get_operand_value(instruction, 1, state)?;
        
        self.set_operand_value(instruction, 0, val2, state)?;
        self.set_operand_value(instruction, 1, val1, state)?;
        Ok(())
    }

    fn execute_lea(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LEA instruction".to_string()));
        }

        // LEA loads the effective address of the source operand into the destination
        // For now, we'll use the source operand value as the address
        let src = self.get_operand_value(instruction, 0, state)?;
        self.set_operand_value(instruction, 1, src, state)?;
        Ok(())
    }

    fn execute_mul(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_imul(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_div(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid DIV instruction".to_string()));
        }

        let divisor = self.get_operand_value(instruction, 0, state)?;
        if divisor == 0 {
            return Err(crate::EmulatorError::Cpu("Division by zero".to_string()));
        }

        let dividend = (state.registers.rdx << 64) | state.registers.rax;
        let quotient = dividend / divisor;
        let remainder = dividend % divisor;
        
        state.registers.rax = quotient;
        state.registers.rdx = remainder;
        Ok(())
    }

    fn execute_idiv(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid IDIV instruction".to_string()));
        }

        let divisor = self.get_operand_value(instruction, 0, state)? as i64;
        if divisor == 0 {
            return Err(crate::EmulatorError::Cpu("Division by zero".to_string()));
        }

        let dividend = ((state.registers.rdx as i64) << 64) | (state.registers.rax as i64);
        let quotient = dividend / divisor;
        let remainder = dividend % divisor;
        
        state.registers.rax = quotient as u64;
        state.registers.rdx = remainder as u64;
        Ok(())
    }

    fn execute_neg(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid NEG instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let result = 0u64.wrapping_sub(src);
        
        self.set_operand_value(instruction, 0, result, state)?;
        self.update_arithmetic_flags(result, src, 0, true, state);
        Ok(())
    }

    fn execute_not(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid NOT instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let result = !src;
        
        self.set_operand_value(instruction, 0, result, state)?;
        Ok(())
    }

    fn execute_shl(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_shr(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_sar(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_rol(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_ror(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_rcl(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_rcr(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_stc(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Set carry flag
        state.registers.set_flag(RFlags::CARRY, true);
        Ok(())
    }

    fn execute_clc(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Clear carry flag
        state.registers.set_flag(RFlags::CARRY, false);
        Ok(())
    }

    fn execute_cmc(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Complement carry flag
        let current = state.registers.get_flag(RFlags::CARRY);
        state.registers.set_flag(RFlags::CARRY, !current);
        Ok(())
    }

    fn execute_std(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Set direction flag
        state.registers.set_flag(RFlags::DIRECTION, true);
        Ok(())
    }

    fn execute_cld(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Clear direction flag
        state.registers.set_flag(RFlags::DIRECTION, false);
        Ok(())
    }

    fn execute_xlat(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Table lookup: AL = [BX + AL]
        let offset = (state.registers.rbx & 0xFFFF) + (state.registers.rax & 0xFF);
        let value = state.read_u8(offset)?;
        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFFFF00) | (value as u64);
        Ok(())
    }

    fn execute_cpuid(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 0 {
            return Err(crate::EmulatorError::Cpu("Invalid CPUID instruction".to_string()));
        }

        // CPUID returns processor identification information
        // For now, return basic x86_64 information
        let eax = state.registers.rax & 0xFFFFFFFF;
        
        match eax {
            0 => {
                // Maximum input value for basic CPUID information
                state.registers.rax = 0x0000000D; // Maximum supported leaf
                state.registers.rbx = 0x68747541; // "Auth"
                state.registers.rcx = 0x444D4163; // "cAMD"
                state.registers.rdx = 0x69746E65; // "enti"
            }
            1 => {
                // Processor info and feature bits
                state.registers.rax = 0x00060F01; // Family 6, Model 15, Stepping 1
                state.registers.rbx = 0x00000000; // Brand index, CLFLUSH, etc.
                state.registers.rcx = 0x00000000; // Feature flags
                state.registers.rdx = 0x00000000; // Feature flags
            }
            _ => {
                // Unsupported leaf
                state.registers.rax = 0;
                state.registers.rbx = 0;
                state.registers.rcx = 0;
                state.registers.rdx = 0;
            }
        }
        Ok(())
    }

    fn execute_wait(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // WAIT instruction - wait for floating point operations
        // For now, just do nothing (no floating point unit)
        Ok(())
    }

    // Additional missing instruction implementations
    fn execute_aaa(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_aad(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_aam(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_aas(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_adc(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ADC instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let dst = self.get_operand_value(instruction, 1, state)?;
        let carry = if state.registers.get_flag(RFlags::CARRY) { 1 } else { 0 };
        let result = dst.wrapping_add(src).wrapping_add(carry);

        self.set_operand_value(instruction, 1, result, state)?;
        self.update_arithmetic_flags(result, src, dst, false, state);
        Ok(())
    }

    fn execute_arpl(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ARPL instruction".to_string()));
        }
        // Adjust RPL (simplified - just log for now)
        log::debug!("ARPL instruction executed");
        Ok(())
    }

    fn execute_bswap(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid BSWAP instruction".to_string()));
        }

        let src = self.get_operand_value(instruction, 0, state)?;
        let result = src.swap_bytes();
        self.set_operand_value(instruction, 0, result, state)?;
        Ok(())
    }

    fn execute_cbw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Convert Byte to Word (sign extend AL to AX)
        let al = (state.registers.rax & 0xFF) as i8;
        let ax = al as i16 as u16;
        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFF0000) | (ax as u64);
        Ok(())
    }

    fn execute_cdq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Convert Doubleword to Quadword (sign extend EAX to EDX:EAX)
        let eax = (state.registers.rax & 0xFFFFFFFF) as i32;
        let edx = (eax >> 31) as u32;
        state.registers.rdx = (state.registers.rdx & 0xFFFFFFFF00000000) | (edx as u64);
        Ok(())
    }

    fn execute_cdqe(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Convert Doubleword to Quadword (sign extend EAX to RAX)
        let eax = (state.registers.rax & 0xFFFFFFFF) as i32;
        state.registers.rax = eax as i64 as u64;
        Ok(())
    }

    fn execute_clts(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Clear Task-Switched Flag (simplified - just log for now)
        log::debug!("CLTS instruction executed");
        Ok(())
    }

    fn execute_cmpxchg(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_cqo(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Convert Quadword to Octaword (sign extend RAX to RDX:RAX)
        let rax = state.registers.rax as i64;
        state.registers.rdx = (rax >> 63) as u64;
        Ok(())
    }

    fn execute_cwd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Convert Word to Doubleword (sign extend AX to DX:AX)
        let ax = (state.registers.rax & 0xFFFF) as i16;
        let dx = (ax >> 15) as u16;
        state.registers.rdx = (state.registers.rdx & 0xFFFFFFFFFFFF0000) | (dx as u64);
        Ok(())
    }

    fn execute_cwde(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Convert Word to Extended Doubleword (sign extend AX to EAX)
        let ax = (state.registers.rax & 0xFFFF) as i16;
        let eax = ax as i32 as u32;
        state.registers.rax = (state.registers.rax & 0xFFFFFFFF00000000) | (eax as u64);
        Ok(())
    }

    fn execute_daa(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Decimal Adjust After Addition
        let al = (state.registers.rax & 0xFF) as u8;
        let mut result = al;
        let mut cf = false;

        if (al & 0x0F) > 9 || state.registers.get_flag(RFlags::AUXILIARY) {
            result += 6;
            state.registers.set_flag(RFlags::AUXILIARY, true);
        } else {
            state.registers.set_flag(RFlags::AUXILIARY, false);
        }

        if al > 0x99 || state.registers.get_flag(RFlags::CARRY) {
            result += 0x60;
            cf = true;
        }

        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFFFF00) | (result as u64);
        state.registers.set_flag(RFlags::CARRY, cf);
        self.update_logical_flags(result as u64, state);
        Ok(())
    }

    fn execute_das(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Decimal Adjust After Subtraction
        let al = (state.registers.rax & 0xFF) as u8;
        let mut result = al;
        let mut cf = false;

        if (al & 0x0F) > 9 || state.registers.get_flag(RFlags::AUXILIARY) {
            result -= 6;
            state.registers.set_flag(RFlags::AUXILIARY, true);
        } else {
            state.registers.set_flag(RFlags::AUXILIARY, false);
        }

        if al > 0x99 || state.registers.get_flag(RFlags::CARRY) {
            result -= 0x60;
            cf = true;
        }

        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFFFF00) | (result as u64);
        state.registers.set_flag(RFlags::CARRY, cf);
        self.update_logical_flags(result as u64, state);
        Ok(())
    }

    fn execute_enter(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid ENTER instruction".to_string()));
        }

        let frame_size = self.get_operand_value(instruction, 0, state)? as u32;
        let nesting_level = self.get_operand_value(instruction, 1, state)? as u32;

        // Push current frame pointer
        state.registers.rsp -= 8;
        state.write_u64(state.registers.rsp, state.registers.rbp)?;

        // Set new frame pointer
        state.registers.rbp = state.registers.rsp;

        // Allocate local variables
        state.registers.rsp -= frame_size as u64;

        // Handle nesting levels (simplified)
        for _ in 0..nesting_level.min(31) {
            state.registers.rsp -= 8;
            // In real implementation, would push previous frame pointers
        }
        Ok(())
    }

    fn execute_into(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Interrupt on Overflow
        if state.registers.get_flag(RFlags::OVERFLOW) {
            // Trigger interrupt 4
            self.execute_int(&Instruction::default(), state)?;
        }
        Ok(())
    }

    fn execute_invd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Invalidate Cache (simplified - just log for now)
        log::debug!("INVD instruction executed");
        Ok(())
    }

    fn execute_invlpg(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid INVLPG instruction".to_string()));
        }
        // Invalidate TLB Entry (simplified - just log for now)
        log::debug!("INVLPG instruction executed");
        Ok(())
    }

    fn execute_lar(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LAR instruction".to_string()));
        }
        // Load Access Rights (simplified - just log for now)
        log::debug!("LAR instruction executed");
        Ok(())
    }

    fn execute_leave(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Leave procedure (opposite of ENTER)
        state.registers.rsp = state.registers.rbp;
        state.registers.rbp = state.read_u64(state.registers.rsp)?;
        state.registers.rsp += 8;
        Ok(())
    }

    fn execute_lfence(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Load Fence (memory ordering)
        // For now, just do nothing
        Ok(())
    }

    fn execute_lldt(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid LLDT instruction".to_string()));
        }
        // Load Local Descriptor Table (simplified - just log for now)
        log::debug!("LLDT instruction executed");
        Ok(())
    }

    fn execute_lsl(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 2 {
            return Err(crate::EmulatorError::Cpu("Invalid LSL instruction".to_string()));
        }
        // Load Segment Limit (simplified - just log for now)
        log::debug!("LSL instruction executed");
        Ok(())
    }

    fn execute_mfence(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Memory Fence (memory ordering)
        // For now, just do nothing
        Ok(())
    }

    fn execute_pause(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Pause instruction (for spin loops)
        // For now, just do nothing
        Ok(())
    }

    fn execute_rdmsr(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 0 {
            return Err(crate::EmulatorError::Cpu("Invalid RDMSR instruction".to_string()));
        }

        let ecx = (state.registers.rcx & 0xFFFFFFFF) as u32;
        match ecx {
            0x1B => {
                // IA32_APIC_BASE
                state.registers.rax = 0xFEE00000;
                state.registers.rdx = 0;
            }
            _ => {
                // Unsupported MSR
                state.registers.rax = 0;
                state.registers.rdx = 0;
            }
        }
        Ok(())
    }

    fn execute_rdpmc(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 0 {
            return Err(crate::EmulatorError::Cpu("Invalid RDPMC instruction".to_string()));
        }

        let ecx = (state.registers.rcx & 0xFFFFFFFF) as u32;
        // Return dummy performance counter values
        state.registers.rax = ecx as u64;
        state.registers.rdx = 0;
        Ok(())
    }

    fn execute_rdrand(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid RDRAND instruction".to_string()));
        }

        // Generate a pseudo-random number
        let random_value = (state.registers.rax ^ state.registers.rcx ^ state.registers.rdx) as u64;
        self.set_operand_value(instruction, 0, random_value, state)?;
        state.registers.set_flag(RFlags::CARRY, true); // Indicate success
        Ok(())
    }

    fn execute_rdseed(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid RDSEED instruction".to_string()));
        }

        // Generate a pseudo-random number (simplified)
        let random_value = (state.registers.rax ^ state.registers.rcx ^ state.registers.rdx) as u64;
        self.set_operand_value(instruction, 0, random_value, state)?;
        state.registers.set_flag(RFlags::CARRY, true); // Indicate success
        Ok(())
    }

    fn execute_rdtsc(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 0 {
            return Err(crate::EmulatorError::Cpu("Invalid RDTSC instruction".to_string()));
        }

        // Return dummy timestamp counter values
        state.registers.rax = 0x12345678;
        state.registers.rdx = 0x9ABCDEF0;
        Ok(())
    }

    fn execute_rdtscp(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 0 {
            return Err(crate::EmulatorError::Cpu("Invalid RDTSCP instruction".to_string()));
        }

        // Return dummy timestamp counter values
        state.registers.rax = 0x12345678;
        state.registers.rdx = 0x9ABCDEF0;
        state.registers.rcx = 0; // Processor ID
        Ok(())
    }

    fn execute_rsm(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Resume from System Management Mode (simplified - just log for now)
        log::debug!("RSM instruction executed");
        Ok(())
    }

    fn execute_salc(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // Set AL on Carry
        let al_value = if state.registers.get_flag(RFlags::CARRY) { 0xFF } else { 0x00 };
        state.registers.rax = (state.registers.rax & 0xFFFFFFFFFFFFFF00) | (al_value as u64);
        Ok(())
    }

    fn execute_sbb(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
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

    fn execute_sfence(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Store Fence (memory ordering)
        // For now, just do nothing
        Ok(())
    }

    fn execute_sgdt(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid SGDT instruction".to_string()));
        }
        // Store Global Descriptor Table (simplified - just log for now)
        log::debug!("SGDT instruction executed");
        Ok(())
    }

    fn execute_sidt(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid SIDT instruction".to_string()));
        }
        // Store Interrupt Descriptor Table (simplified - just log for now)
        log::debug!("SIDT instruction executed");
        Ok(())
    }

    fn execute_sldt(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid SLDT instruction".to_string()));
        }
        // Store Local Descriptor Table (simplified - just log for now)
        log::debug!("SLDT instruction executed");
        Ok(())
    }

    fn execute_swapgs(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Swap GS Base (simplified - just log for now)
        log::debug!("SWAPGS instruction executed");
        Ok(())
    }

    fn execute_verr(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid VERR instruction".to_string()));
        }
        // Verify segment for read (simplified - just log for now)
        log::debug!("VERR instruction executed");
        Ok(())
    }

    fn execute_verw(&self, instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid VERW instruction".to_string()));
        }
        // Verify segment for write (simplified - just log for now)
        log::debug!("VERW instruction executed");
        Ok(())
    }

    fn execute_wbinvd(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // Write Back and Invalidate Cache (simplified - just log for now)
        log::debug!("WBINVD instruction executed");
        Ok(())
    }

    fn execute_wrmsr(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 0 {
            return Err(crate::EmulatorError::Cpu("Invalid WRMSR instruction".to_string()));
        }

        let ecx = (state.registers.rcx & 0xFFFFFFFF) as u32;
        let eax = (state.registers.rax & 0xFFFFFFFF) as u32;
        let edx = (state.registers.rdx & 0xFFFFFFFF) as u32;
        
        match ecx {
            0x1B => {
                // IA32_APIC_BASE
                log::debug!("WRMSR: IA32_APIC_BASE = 0x{:08x}{:08x}", edx, eax);
            }
            _ => {
                log::debug!("WRMSR: Unsupported MSR 0x{:08x} = 0x{:08x}{:08x}", ecx, edx, eax);
            }
        }
        Ok(())
    }
}
