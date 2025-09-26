use iced_x86::{Decoder, DecoderOptions, Instruction, OpKind};
use crate::cpu::state::CpuState;
use crate::cpu::registers::RFlags;
use crate::Result;
use iced_x86::Mnemonic;

mod a;
mod b;
mod c;
mod d;
mod e;
mod f;
mod g;
mod h;
mod i;
mod j;
mod k;
mod l;
mod m;
mod n;
mod o;
mod p;
mod q;
mod r;
mod s;
mod t;
mod u;
mod v;
mod w;
mod x;
mod y;
mod z;

/// Instruction decoder and executor
pub struct InstructionDecoder <'a> {
    decoder: Decoder<'a>,
}

impl InstructionDecoder<'_> {
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
            Mnemonic::INVALID => Err(crate::EmulatorError::Cpu("Invalid instruction".to_string())),
            // A
            Mnemonic::Aaa => self.execute_aaa(instruction, state),
            Mnemonic::Aad => self.execute_aad(instruction, state),
            Mnemonic::Aam => self.execute_aam(instruction, state),
            Mnemonic::Aas => self.execute_aas(instruction, state),
            Mnemonic::Adc => self.execute_adc(instruction, state),
            Mnemonic::Add => self.execute_add(instruction, state),
            Mnemonic::Addpd => self.execute_addpd(instruction, state),
            Mnemonic::Addps => self.execute_addps(instruction, state),
            Mnemonic::Addsd => self.execute_addsd(instruction, state),
            Mnemonic::Addss => self.execute_addss(instruction, state),
            Mnemonic::Addsubpd => self.execute_addsubpd(instruction, state),
            Mnemonic::Addsubps => self.execute_addsubps(instruction, state),
            Mnemonic::Adox => self.execute_adox(instruction, state),
            Mnemonic::Aesdec => self.execute_aesdec(instruction, state),
            Mnemonic::Aesdeclast => self.execute_aesdeclast(instruction, state),
            Mnemonic::Aesenc => self.execute_aesenc(instruction, state),
            Mnemonic::Aesenclast => self.execute_aesenclast(instruction, state),
            Mnemonic::Aesimc => self.execute_aesimc(instruction, state),
            Mnemonic::Aeskeygenassist => self.execute_aeskeygenassist(instruction, state),
            Mnemonic::And => self.execute_and(instruction, state),
            Mnemonic::Andn => self.execute_andn(instruction, state),
            Mnemonic::Andnpd => self.execute_andnpd(instruction, state),
            Mnemonic::Andnps => self.execute_andnps(instruction, state),
            Mnemonic::Andpd => self.execute_andpd(instruction, state),
            Mnemonic::Andps => self.execute_andps(instruction, state),
            Mnemonic::Arpl => self.execute_arpl(instruction, state),
            // B
            Mnemonic::Bextr => self.execute_bextr(instruction, state),
            Mnemonic::Blcfill => self.execute_blcfill(instruction, state),
            Mnemonic::Blci => self.execute_blci(instruction, state),
            Mnemonic::Blcic => self.execute_blcic(instruction, state),
            Mnemonic::Blcmsk => self.execute_blcmsk(instruction, state),
            Mnemonic::Blcs => self.execute_blcs(instruction, state),
            Mnemonic::Blendpd => self.execute_blendpd(instruction, state),
            Mnemonic::Blendps => self.execute_blendps(instruction, state),
            Mnemonic::Blendvpd => self.execute_blendvpd(instruction, state),
            Mnemonic::Blendvps => self.execute_blendvps(instruction, state),
            Mnemonic::Blsfill => self.execute_blsfill(instruction, state),
            Mnemonic::Blsi => self.execute_blsi(instruction, state),
            Mnemonic::Blsic => self.execute_blsic(instruction, state),
            Mnemonic::Blsmsk => self.execute_blsmask(instruction, state),
            Mnemonic::Blsr => self.execute_blsr(instruction, state),
            Mnemonic::Bndcl => self.execute_bndcl(instruction, state),
            Mnemonic::Bndcn => self.execute_bndcn(instruction, state),
            Mnemonic::Bndcu => self.execute_bndcu(instruction, state),
            Mnemonic::Bndldx => self.execute_bndldx(instruction, state),
            Mnemonic::Bndmk => self.execute_bndmk(instruction, state),
            Mnemonic::Bndmov => self.execute_bndmov(instruction, state),
            Mnemonic::Bndstx => self.execute_bndstx(instruction, state),
            Mnemonic::Bound => self.execute_bound(instruction, state),
            Mnemonic::Bsf => self.execute_bsf(instruction, state),
            Mnemonic::Bsr => self.execute_bsr(instruction, state),
            Mnemonic::Bswap => self.execute_bswap(instruction, state),
            Mnemonic::Bt => self.execute_bt(instruction, state),
            Mnemonic::Btc => self.execute_btc(instruction, state),
            Mnemonic::Btr => self.execute_btr(instruction, state),
            Mnemonic::Bts => self.execute_bts(instruction, state),
            Mnemonic::Bzhi => self.execute_bzhi(instruction, state),
            // C
            Mnemonic::Call => self.execute_call(instruction, state),
            Mnemonic::Cbw => self.execute_cbw(instruction, state),
            Mnemonic::Cdq => self.execute_cdq(instruction, state),
            Mnemonic::Cdqe => self.execute_cdqe(instruction, state),
            Mnemonic::Clac => self.execute_clac(instruction, state),
            Mnemonic::Clc => self.execute_clc(instruction, state),
            Mnemonic::Cld => self.execute_cld(instruction, state),
            Mnemonic::Cldemote => self.execute_cldemote(instruction, state),
            Mnemonic::Clflush => self.execute_clflush(instruction, state),
            Mnemonic::Clflushopt => self.execute_clflushopt(instruction, state),
            Mnemonic::Clgi => self.execute_clgi(instruction, state),
            Mnemonic::Cli => self.execute_cli(instruction, state),
            Mnemonic::Clrssbsy => self.execute_clrssbsy(instruction, state),
            Mnemonic::Clts => self.execute_clts(instruction, state),
            Mnemonic::Clui => self.execute_clui(instruction, state),
            Mnemonic::Clwb => self.execute_clwb(instruction, state),
            Mnemonic::Clzero => self.execute_clzero(instruction, state),
            Mnemonic::Cmc => self.execute_cmc(instruction, state),
            Mnemonic::Cmova => self.execute_cmova(instruction, state),
            Mnemonic::Cmovae => self.execute_cmovae(instruction, state),
            Mnemonic::Cmovb => self.execute_cmovb(instruction, state),
            Mnemonic::Cmovbe => self.execute_cmovbe(instruction, state),
            Mnemonic::Cmove => self.execute_cmove(instruction, state),
            Mnemonic::Cmovg => self.execute_cmovg(instruction, state),
            Mnemonic::Cmovge => self.execute_cmovge(instruction, state),
            Mnemonic::Cmovl => self.execute_cmovl(instruction, state),
            Mnemonic::Cmovle => self.execute_cmovle(instruction, state),
            Mnemonic::Cmovne => self.execute_cmovne(instruction, state),
            Mnemonic::Cmovno => self.execute_cmovno(instruction, state),
            Mnemonic::Cmovnp => self.execute_cmovnp(instruction, state),
            Mnemonic::Cmovns => self.execute_cmovns(instruction, state),
            Mnemonic::Cmovo => self.execute_cmovo(instruction, state),
            Mnemonic::Cmovp => self.execute_cmovp(instruction, state),
            Mnemonic::Cmovs => self.execute_cmovs(instruction, state),
            Mnemonic::Cmp => self.execute_cmp(instruction, state),
            Mnemonic::Cmpbexadd => self.execute_cmpbexadd(instruction, state),
            Mnemonic::Cmpbxadd => self.execute_cmpbxadd(instruction, state),
            Mnemonic::Cmplexadd => self.execute_cmplexadd(instruction, state),
            Mnemonic::Cmplxadd => self.execute_cmplxadd(instruction, state),
            Mnemonic::Cmpnbexadd => self.execute_cmpnbexadd(instruction, state),
            Mnemonic::Cmpnbxadd => self.execute_cmpnbxadd(instruction, state),
            Mnemonic::Cmpnlexadd => self.execute_cmpnlexadd(instruction, state),
            Mnemonic::Cmpnlxadd => self.execute_cmpnlxadd(instruction, state),
            Mnemonic::Cmpnoxadd => self.execute_cmpnoxadd(instruction, state),
            Mnemonic::Cmpnpxadd => self.execute_cmpnpxadd(instruction, state),
            Mnemonic::Cmpnsxadd => self.execute_cmpnsxadd(instruction, state),
            Mnemonic::Cmpnzxadd => self.execute_cmpnzxadd(instruction, state),
            Mnemonic::Cmpoxadd => self.execute_cmpoxadd(instruction, state),
            Mnemonic::Cmppd => self.execute_cmppd(instruction, state),
            Mnemonic::Cmppxadd => self.execute_cmppxadd(instruction, state),
            Mnemonic::Cmpps => self.execute_cmpps(instruction, state),
            Mnemonic::Cmpsb => self.execute_cmpsb(instruction, state),
            Mnemonic::Cmpsd => self.execute_cmpsd(instruction, state),
            Mnemonic::Cmpsq => self.execute_cmpsq(instruction, state),
            Mnemonic::Cmpsxadd => self.execute_cmpsxadd(instruction, state),
            Mnemonic::Cmpss => self.execute_cmpss(instruction, state),
            Mnemonic::Cmpsw => self.execute_cmpsw(instruction, state),
            Mnemonic::Cmpxchg => self.execute_cmpxchg(instruction, state),
            Mnemonic::Cmpzxadd => self.execute_cmpzxadd(instruction, state),
            Mnemonic::Comisd => self.execute_comisd(instruction, state),
            Mnemonic::Comiss => self.execute_comiss(instruction, state),
            Mnemonic::Cpuid => self.execute_cpuid(instruction, state),
            Mnemonic::Cqo => self.execute_cqo(instruction, state),
            Mnemonic::Cwd => self.execute_cwd(instruction, state),
            Mnemonic::Cwde => self.execute_cwde(instruction, state),
            // D
            Mnemonic::Daa => self.execute_daa(instruction, state),
            Mnemonic::Das => self.execute_das(instruction, state),
            Mnemonic::Dec => self.execute_dec(instruction, state),
            Mnemonic::Div => self.execute_div(instruction, state),
            // E
            Mnemonic::Enter => self.execute_enter(instruction, state),
            // H
            Mnemonic::Hlt => self.execute_hlt(instruction, state),
            // I
            Mnemonic::Idiv => self.execute_idiv(instruction, state),
            Mnemonic::Imul => self.execute_imul(instruction, state),
            Mnemonic::In => self.execute_in(instruction, state),
            Mnemonic::Inc => self.execute_inc(instruction, state),
            Mnemonic::Insb => self.execute_insb(instruction, state),
            Mnemonic::Insd => self.execute_insd(instruction, state),
            Mnemonic::Insw => self.execute_insw(instruction, state),
            Mnemonic::Int => self.execute_int(instruction, state),
            Mnemonic::Into => self.execute_into(instruction, state),
            Mnemonic::Invd => self.execute_invd(instruction, state),
            Mnemonic::Invlpg => self.execute_invlpg(instruction, state),
            Mnemonic::Iret => self.execute_iret(instruction, state),
            // J
            Mnemonic::Ja => self.execute_ja(instruction, state),
            Mnemonic::Jae => self.execute_jae(instruction, state),
            Mnemonic::Jb => self.execute_jb(instruction, state),
            Mnemonic::Jbe => self.execute_jbe(instruction, state),
            Mnemonic::Jcxz => self.execute_jcxz(instruction, state),
            Mnemonic::Je => self.execute_je(instruction, state),
            Mnemonic::Jecxz => self.execute_jecxz(instruction, state),
            Mnemonic::Jg => self.execute_jg(instruction, state),
            Mnemonic::Jge => self.execute_jge(instruction, state),
            Mnemonic::Jl => self.execute_jl(instruction, state),
            Mnemonic::Jle => self.execute_jle(instruction, state),
            Mnemonic::Jmp => self.execute_jmp(instruction, state),
            Mnemonic::Jne => self.execute_jne(instruction, state),
            Mnemonic::Jno => self.execute_jno(instruction, state),
            Mnemonic::Jns => self.execute_jns(instruction, state),
            Mnemonic::Jo => self.execute_jo(instruction, state),
            Mnemonic::Jrcxz => self.execute_jrcxz(instruction, state),
            Mnemonic::Js => self.execute_js(instruction, state),
            // L
            Mnemonic::Lahf => self.execute_lahf(instruction, state),
            Mnemonic::Lar => self.execute_lar(instruction, state),
            Mnemonic::Lds => self.execute_lds(instruction, state),
            Mnemonic::Lea => self.execute_lea(instruction, state),
            Mnemonic::Leave => self.execute_leave(instruction, state),
            Mnemonic::Les => self.execute_les(instruction, state),
            Mnemonic::Lfence => self.execute_lfence(instruction, state),
            Mnemonic::Lfs => self.execute_lfs(instruction, state),
            Mnemonic::Lgdt => self.execute_lgdt(instruction, state),
            Mnemonic::Lgs => self.execute_lgs(instruction, state),
            Mnemonic::Lidt => self.execute_lidt(instruction, state),
            Mnemonic::Lldt => self.execute_lldt(instruction, state),
            Mnemonic::Lmsw => self.execute_lmsw(instruction, state),
            Mnemonic::Lodsb => self.execute_lodsb(instruction, state),
            Mnemonic::Lodsd => self.execute_lodsd(instruction, state),
            Mnemonic::Lodsq => self.execute_lodsq(instruction, state),
            Mnemonic::Lodsw => self.execute_lodsw(instruction, state),
            Mnemonic::Loop => self.execute_loop(instruction, state),
            Mnemonic::Loope => self.execute_loope(instruction, state),
            Mnemonic::Loopne => self.execute_loopne(instruction, state),
            Mnemonic::Lsl => self.execute_lsl(instruction, state),
            Mnemonic::Lss => self.execute_lss(instruction, state),
            Mnemonic::Ltr => self.execute_ltr(instruction, state),
            // M
            Mnemonic::Mfence => self.execute_mfence(instruction, state),
            Mnemonic::Mov => self.execute_mov(instruction, state),
            Mnemonic::Movsb => self.execute_movsb(instruction, state),
            Mnemonic::Movsd => self.execute_movsd(instruction, state),
            Mnemonic::Movsq => self.execute_movsq(instruction, state),
            Mnemonic::Movsw => self.execute_movsw(instruction, state),
            Mnemonic::Movsx => self.execute_movsx(instruction, state),
            Mnemonic::Movzx => self.execute_movzx(instruction, state),
            Mnemonic::Mul => self.execute_mul(instruction, state),
            Mnemonic::Neg => self.execute_neg(instruction, state),
            Mnemonic::Nop => self.execute_nop(instruction, state),
            Mnemonic::Not => self.execute_not(instruction, state),
            // O
            Mnemonic::Or => self.execute_or(instruction, state),
            Mnemonic::Out => self.execute_out(instruction, state),
            Mnemonic::Outsb => self.execute_outsb(instruction, state),
            Mnemonic::Outsd => self.execute_outsd(instruction, state),
            Mnemonic::Outsw => self.execute_outsw(instruction, state),
            // P
            Mnemonic::Pause => self.execute_pause(instruction, state),
            Mnemonic::Pop => self.execute_pop(instruction, state),
            Mnemonic::Popa => self.execute_popa(instruction, state),
            Mnemonic::Popcnt => self.execute_popcnt(instruction, state),
            Mnemonic::Popf => self.execute_popf(instruction, state),
            Mnemonic::Push => self.execute_push(instruction, state),
            Mnemonic::Pusha => self.execute_pusha(instruction, state),
            Mnemonic::Pushf => self.execute_pushf(instruction, state),
            // R
            Mnemonic::Rcl => self.execute_rcl(instruction, state),
            Mnemonic::Rcr => self.execute_rcr(instruction, state),
            Mnemonic::Rdmsr => self.execute_rdmsr(instruction, state),
            Mnemonic::Rdpmc => self.execute_rdpmc(instruction, state),
            Mnemonic::Rdrand => self.execute_rdrand(instruction, state),
            Mnemonic::Rdseed => self.execute_rdseed(instruction, state),
            Mnemonic::Rdtsc => self.execute_rdtsc(instruction, state),
            Mnemonic::Rdtscp => self.execute_rdtscp(instruction, state),
            Mnemonic::Ret => self.execute_ret(instruction, state),
            Mnemonic::Rol => self.execute_rol(instruction, state),
            Mnemonic::Ror => self.execute_ror(instruction, state),
            Mnemonic::Rsm => self.execute_rsm(instruction, state),
            // S
            Mnemonic::Sahf => self.execute_sahf(instruction, state),
            Mnemonic::Sal => self.execute_shl(instruction, state),
            Mnemonic::Salc => self.execute_salc(instruction, state),
            Mnemonic::Sar => self.execute_sar(instruction, state),
            Mnemonic::Sbb => self.execute_sbb(instruction, state),
            Mnemonic::Scasb => self.execute_scasb(instruction, state),
            Mnemonic::Scasd => self.execute_scasd(instruction, state),
            Mnemonic::Scasq => self.execute_scasq(instruction, state),
            Mnemonic::Scasw => self.execute_scasw(instruction, state),
            Mnemonic::Sfence => self.execute_sfence(instruction, state),
            Mnemonic::Sgdt => self.execute_sgdt(instruction, state),
            Mnemonic::Shl => self.execute_shl(instruction, state),
            Mnemonic::Shr => self.execute_shr(instruction, state),
            Mnemonic::Sidt => self.execute_sidt(instruction, state),
            Mnemonic::Sldt => self.execute_sldt(instruction, state),
            Mnemonic::Smsw => self.execute_smsw(instruction, state),
            Mnemonic::Stc => self.execute_stc(instruction, state),
            Mnemonic::Std => self.execute_std(instruction, state),
            Mnemonic::Sti => self.execute_sti(instruction, state),
            Mnemonic::Stosb => self.execute_stosb(instruction, state),
            Mnemonic::Stosd => self.execute_stosd(instruction, state),
            Mnemonic::Stosq => self.execute_stosq(instruction, state),
            Mnemonic::Stosw => self.execute_stosw(instruction, state),
            Mnemonic::Str => self.execute_str(instruction, state),
            Mnemonic::Sub => self.execute_sub(instruction, state),
            Mnemonic::Swapgs => self.execute_swapgs(instruction, state),
            Mnemonic::Syscall => self.execute_syscall(instruction, state),
            Mnemonic::Sysret => self.execute_sysret(instruction, state),
            // T
            Mnemonic::Test => self.execute_test(instruction, state),
            // V
            Mnemonic::Verr => self.execute_verr(instruction, state),
            Mnemonic::Verw => self.execute_verw(instruction, state),
            // W
            Mnemonic::Wbinvd => self.execute_wbinvd(instruction, state),
            Mnemonic::Wrmsr => self.execute_wrmsr(instruction, state),
            // X
            Mnemonic::Xchg => self.execute_xchg(instruction, state),
            Mnemonic::Xlatb => self.execute_xlat(instruction, state),
            Mnemonic::Xor => self.execute_xor(instruction, state),
            _ => {
                // Unimplemented instruction
                log::warn!("Unimplemented instruction: {:?}", instruction.mnemonic());
                Ok(())
            }
        }
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
            // if let Ok(reg) = instruction.try_op_register(op_index) addr = self.get_register_value(reg, state);,
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

    fn update_logical_flags(&self, result: u64, state: &mut CpuState) {
        state.registers.set_flag(RFlags::ZERO, result == 0);
        state.registers.set_flag(RFlags::SIGN, (result & 0x8000000000000000) != 0);
        state.registers.set_flag(RFlags::CARRY, false);
        state.registers.set_flag(RFlags::OVERFLOW, false);
        
        let low_byte = (result & 0xFF) as u8;
        let parity = low_byte.count_ones() % 2 == 0;
        state.registers.set_flag(RFlags::PARITY, parity);
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

}
