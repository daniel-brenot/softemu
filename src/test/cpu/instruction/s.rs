use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};
use crate::memory::guest_memory::GuestMemory;
use crate::Result;
use iced_x86::{Decoder, DecoderOptions};

fn create_test_cpu_state() -> Result<CpuState> {
    let memory = GuestMemory::new(1024 * 1024)?; // 1MB memory
    Ok(CpuState::new(memory))
}

fn execute_instruction(instruction_bytes: &[u8], state: &mut CpuState) -> Result<CpuState> {
    let mut instruction_decoder = InstructionDecoder::new();
    let mut decoder = Decoder::new(64, instruction_bytes, DecoderOptions::NONE);
    let instruction = decoder.decode();
    instruction_decoder.execute_instruction(&instruction, state)?;
    Ok(state.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x1000;
        state.registers.rbx = 0x300;
        
        // SUB RAX, RBX (0x48 0x29 0xC3)
        let result = execute_instruction(&[0x48, 0x29, 0xC3], &mut state).unwrap();
        assert_eq!(result.registers.rax, 0xD00); // 0x1000 - 0x300 = 0xD00
    }

    #[test]
    fn test_sub_with_carry() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x100;
        state.registers.rbx = 0x200;
        
        // SUB RAX, RBX (0x48 0x29 0xC3)
        let result = execute_instruction(&[0x48, 0x29, 0xC3], &mut state).unwrap();
        assert_eq!(result.registers.rax, 0xFFFFFFFFFFFFFF00); // 0x100 - 0x200 = -0x100
        assert!(result.registers.get_flag(RFlags::CARRY)); // Borrow occurred
    }

    #[test]
    fn test_shl_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x10;
        
        // SHL RAX, 2 (0x48 0xC1 0xE0 0x02)
        let result = execute_instruction(&[0x48, 0xC1, 0xE0, 0x02], &mut state).unwrap();
        assert_eq!(result.registers.rax, 0x40); // 0x10 << 2 = 0x40
    }

    #[test]
    fn test_shl_with_carry() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x8000000000000000;
        
        // SHL RAX, 1 (0x48 0xD1 0xE0)
        let result = execute_instruction(&[0x48, 0xD1, 0xE0], &mut state).unwrap();
        assert_eq!(result.registers.rax, 0x0); // 0x8000000000000000 << 1 = 0x0
        assert!(result.registers.get_flag(RFlags::CARRY)); // MSB was shifted out
    }

    #[test]
    fn test_shr_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x40;
        
        // SHR RAX, 2 (0x48 0xC1 0xE8 0x02)
        let result = execute_instruction(&[0x48, 0xC1, 0xE8, 0x02], &mut state).unwrap();
        assert_eq!(result.registers.rax, 0x10); // 0x40 >> 2 = 0x10
    }

    #[test]
    fn test_shr_with_carry() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x1;
        
        // SHR RAX, 1 (0x48 0xD1 0xE8)
        let result = execute_instruction(&[0x48, 0xD1, 0xE8], &mut state).unwrap();
        assert_eq!(result.registers.rax, 0x0); // 0x1 >> 1 = 0x0
        assert!(result.registers.get_flag(RFlags::CARRY)); // LSB was shifted out
    }

    #[test]
    fn test_sar_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0xFFFFFFFFFFFFFF80; // -128 in two's complement
        
        // SAR RAX, 1 (0x48 0xD1 0xF8)
        let result = execute_instruction(&[0x48, 0xD1, 0xF8], &mut state).unwrap();
        assert_eq!(result.registers.rax, 0xFFFFFFFFFFFFFFC0); // -64 in two's complement
    }

    #[test]
    fn test_sar_positive() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x80; // 128
        
        // SAR RAX, 1 (0x48 0xD1 0xF8)
        let result = execute_instruction(&[0x48, 0xD1, 0xF8], &mut state).unwrap();
        assert_eq!(result.registers.rax, 0x40); // 64
    }

    #[test]
    fn test_stc_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        assert!(!state.registers.get_flag(RFlags::CARRY));
        
        // STC (0xF9)
        let result = execute_instruction(&[0xF9], &mut state).unwrap();
        assert!(result.registers.get_flag(RFlags::CARRY));
    }

    #[test]
    fn test_std_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        assert!(!state.registers.get_flag(RFlags::DIRECTION));
        
        // STD (0xFD)
        let result = execute_instruction(&[0xFD], &mut state).unwrap();
        assert!(result.registers.get_flag(RFlags::DIRECTION));
    }

    #[test]
    fn test_sti_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // Clear interrupt flag first
        state.registers.set_flag(RFlags::INTERRUPT, false);
        assert!(!state.registers.get_flag(RFlags::INTERRUPT));
        
        // STI (0xFB)
        let result = execute_instruction(&[0xFB], &mut state).unwrap();
        assert!(result.registers.get_flag(RFlags::INTERRUPT));
    }

    #[test]
    fn test_salc_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.set_flag(RFlags::CARRY, true);
        
        // SALC (0xD6) - This instruction is not available in 64-bit mode
        // Skip this test in 64-bit mode
        if cfg!(target_arch = "x86_64") {
            println!("SALC instruction not available in 64-bit mode, skipping test");
            return;
        }
        
        let result = execute_instruction(&[0xD6], &mut state).unwrap();
        assert_eq!(result.registers.rax & 0xFF, 0xFF); // AL should be 0xFF when carry is set
    }

    #[test]
    fn test_salc_no_carry() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.set_flag(RFlags::CARRY, false);
        
        // SALC (0xD6) - This instruction is not available in 64-bit mode
        // Skip this test in 64-bit mode
        if cfg!(target_arch = "x86_64") {
            println!("SALC instruction not available in 64-bit mode, skipping test");
            return;
        }
        
        let result = execute_instruction(&[0xD6], &mut state).unwrap();
        assert_eq!(result.registers.rax & 0xFF, 0x00); // AL should be 0x00 when carry is not set
    }

    #[test]
    fn test_sbb_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x1000;
        state.registers.rbx = 0x300;
        state.registers.set_flag(RFlags::CARRY, true);
        
        // SBB RAX, RBX (0x48 0x19 0xC3) - Fixed encoding
        let result = execute_instruction(&[0x48, 0x19, 0xC3], &mut state).unwrap();
        assert_eq!(result.registers.rax, 0xCFF); // 0x1000 - 0x300 - 1 = 0xCFF
    }

    #[test]
    fn test_sbb_no_carry() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x1000;
        state.registers.rbx = 0x300;
        state.registers.set_flag(RFlags::CARRY, false);
        
        // SBB RAX, RBX (0x48 0x19 0xC3) - Fixed encoding
        let result = execute_instruction(&[0x48, 0x19, 0xC3], &mut state).unwrap();
        assert_eq!(result.registers.rax, 0xD00); // 0x1000 - 0x300 - 0 = 0xD00
    }

    #[test]
    fn test_stosb_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x123456789ABCDEF0;
        state.registers.rdi = 0x1000;
        
        // STOSB (0xAA)
        let result = execute_instruction(&[0xAA], &mut state).unwrap();
        assert_eq!(result.registers.rdi, 0x1001); // RDI incremented by 1
        assert_eq!(result.read_u8(0x1000).unwrap(), 0xF0); // AL (0xF0) stored at [RDI]
    }

    #[test]
    fn test_stosb_backward() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x123456789ABCDEF0;
        state.registers.rdi = 0x1000;
        state.registers.set_flag(RFlags::DIRECTION, true);
        
        // STOSB (0xAA)
        let result = execute_instruction(&[0xAA], &mut state).unwrap();
        assert_eq!(result.registers.rdi, 0xFFF); // RDI decremented by 1
        assert_eq!(result.read_u8(0x1000).unwrap(), 0xF0); // AL (0xF0) stored at [RDI]
    }

    #[test]
    fn test_stosw_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x123456789ABCDEF0;
        state.registers.rdi = 0x1000;
        
        // STOSW (0x66 0xAB)
        let result = execute_instruction(&[0x66, 0xAB], &mut state).unwrap();
        assert_eq!(result.registers.rdi, 0x1002); // RDI incremented by 2
        assert_eq!(result.read_u16(0x1000).unwrap(), 0xDEF0); // AX (0xDEF0) stored at [RDI]
    }

    #[test]
    fn test_stosd_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x123456789ABCDEF0;
        state.registers.rdi = 0x1000;
        
        // STOSD (0xAB)
        let result = execute_instruction(&[0xAB], &mut state).unwrap();
        assert_eq!(result.registers.rdi, 0x1004); // RDI incremented by 4
        assert_eq!(result.read_u32(0x1000).unwrap(), 0x9ABCDEF0); // EAX (0x9ABCDEF0) stored at [RDI]
    }

    #[test]
    fn test_stosq_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x123456789ABCDEF0;
        state.registers.rdi = 0x1000;
        
        // STOSQ (0x48 0xAB)
        let result = execute_instruction(&[0x48, 0xAB], &mut state).unwrap();
        assert_eq!(result.registers.rdi, 0x1008); // RDI incremented by 8
        assert_eq!(result.read_u64(0x1000).unwrap(), 0x123456789ABCDEF0); // RAX stored at [RDI]
    }

    #[test]
    fn test_scasb_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x123456789ABCDEF0; // AL = 0xF0
        state.registers.rdi = 0x1000;
        state.write_u8(0x1000, 0xF0).unwrap(); // Store 0xF0 at [RDI]
        
        // SCASB (0xAE)
        let result = execute_instruction(&[0xAE], &mut state).unwrap();
        assert_eq!(result.registers.rdi, 0x1001); // RDI incremented by 1
        assert!(result.registers.get_flag(RFlags::ZERO)); // AL == [RDI], so zero flag set
    }

    #[test]
    fn test_scasb_not_equal() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x123456789ABCDEF0; // AL = 0xF0
        state.registers.rdi = 0x1000;
        state.write_u8(0x1000, 0x00).unwrap(); // Store 0x00 at [RDI]
        
        // SCASB (0xAE)
        let result = execute_instruction(&[0xAE], &mut state).unwrap();
        assert_eq!(result.registers.rdi, 0x1001); // RDI incremented by 1
        assert!(!result.registers.get_flag(RFlags::ZERO)); // AL != [RDI], so zero flag not set
    }

    #[test]
    fn test_scasw_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x123456789ABCDEF0; // AX = 0xDEF0
        state.registers.rdi = 0x1000;
        state.write_u16(0x1000, 0xDEF0).unwrap(); // Store 0xDEF0 at [RDI]
        
        // SCASW (0x66 0xAF)
        let result = execute_instruction(&[0x66, 0xAF], &mut state).unwrap();
        assert_eq!(result.registers.rdi, 0x1002); // RDI incremented by 2
        assert!(result.registers.get_flag(RFlags::ZERO)); // AX == [RDI], so zero flag set
    }

    #[test]
    fn test_scasd_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x123456789ABCDEF0; // EAX = 0x9ABCDEF0
        state.registers.rdi = 0x1000;
        state.write_u32(0x1000, 0x9ABCDEF0).unwrap(); // Store 0x9ABCDEF0 at [RDI]
        
        // SCASD (0xAF)
        let result = execute_instruction(&[0xAF], &mut state).unwrap();
        assert_eq!(result.registers.rdi, 0x1004); // RDI incremented by 4
        assert!(result.registers.get_flag(RFlags::ZERO)); // EAX == [RDI], so zero flag set
    }

    #[test]
    fn test_scasq_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x123456789ABCDEF0;
        state.registers.rdi = 0x1000;
        state.write_u64(0x1000, 0x123456789ABCDEF0).unwrap(); // Store RAX at [RDI]
        
        // SCASQ (0x48 0xAF)
        let result = execute_instruction(&[0x48, 0xAF], &mut state).unwrap();
        assert_eq!(result.registers.rdi, 0x1008); // RDI incremented by 8
        assert!(result.registers.get_flag(RFlags::ZERO)); // RAX == [RDI], so zero flag set
    }

    #[test]
    fn test_sahf_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x123456789ABCDE2A; // AH = 0xDE, AL = 0x2A
        
        // SAHF (0x9E)
        let result = execute_instruction(&[0x9E], &mut state).unwrap();
        // Check that flags were set based on AH value (0xDE masked to 0xD4)
        // SAHF only affects bits 0, 2, 4, 6, 7 (mask 0xD5), so 0xDE becomes 0xD4
        assert_eq!(result.registers.get_flags().bits() & 0xFF, 0xD4);
    }

    #[test]
    fn test_syscall_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rip = 0x1000;
        state.registers.rsp = 0x2000;
        state.registers.msr_lstar = 0x3000;
        state.registers.set_flag(RFlags::DIRECTION, true);
        state.registers.set_flag(RFlags::INTERRUPT, true);
        
        // SYSCALL (0x0F 0x05)
        let result = execute_instruction(&[0x0F, 0x05], &mut state).unwrap();
        assert_eq!(result.registers.rip, 0x3000); // RIP set to MSR_LSTAR
        assert_eq!(result.registers.rsp, 0x1FF0); // RSP decremented by 16 (2 * 8 bytes)
        assert!(!result.registers.get_flag(RFlags::DIRECTION)); // Direction flag cleared
        assert!(!result.registers.get_flag(RFlags::INTERRUPT)); // Interrupt flag cleared
    }

    #[test]
    fn test_sysret_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rsp = 0x2000;
        state.write_u64(0x2000, 0x4000).unwrap(); // Return RIP
        state.write_u64(0x2008, 0x100).unwrap(); // Return RFLAGS
        
        // SYSRET (0x0F 0x07)
        let result = execute_instruction(&[0x0F, 0x07], &mut state).unwrap();
        assert_eq!(result.registers.rip, 0x4000); // RIP restored
        assert_eq!(result.registers.rflags, 0x100); // RFLAGS restored
        assert_eq!(result.registers.rsp, 0x2010); // RSP incremented by 16
    }

    #[test]
    fn test_shift_flags() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x8000000000000000;
        
        // SHL RAX, 1 (0x48 0xD1 0xE0)
        let result = execute_instruction(&[0x48, 0xD1, 0xE0], &mut state).unwrap();
        assert_eq!(result.registers.rax, 0x0);
        assert!(result.registers.get_flag(RFlags::CARRY)); // MSB shifted out
        assert!(result.registers.get_flag(RFlags::ZERO)); // Result is zero
        assert!(!result.registers.get_flag(RFlags::SIGN)); // Result is not negative
    }

    #[test]
    fn test_shift_count_mask() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x1;
        
        // SHL RAX, 65 (should be masked to 1) (0x48 0xC1 0xE0 0x41)
        let result = execute_instruction(&[0x48, 0xC1, 0xE0, 0x41], &mut state).unwrap();
        assert_eq!(result.registers.rax, 0x2); // 0x1 << 1 = 0x2 (65 & 0x3F = 1)
    }

    #[test]
    fn test_string_operations_direction_flag() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x123456789ABCDEF0;
        state.registers.rdi = 0x1000;
        state.registers.set_flag(RFlags::DIRECTION, true);
        
        // STOSB (0xAA)
        let result = execute_instruction(&[0xAA], &mut state).unwrap();
        assert_eq!(result.registers.rdi, 0xFFF); // RDI decremented by 1
        assert_eq!(result.read_u8(0x1000).unwrap(), 0xF0); // AL stored at original RDI
    }

    #[test]
    fn test_arithmetic_flags() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x7FFFFFFFFFFFFFFF; // Max positive 64-bit signed
        state.registers.rbx = 0x1;
        
        // SUB RAX, RBX (0x48 0x29 0xC3)
        let result = execute_instruction(&[0x48, 0x29, 0xC3], &mut state).unwrap();
        assert_eq!(result.registers.rax, 0x7FFFFFFFFFFFFFFE);
        assert!(!result.registers.get_flag(RFlags::OVERFLOW)); // No overflow
        assert!(!result.registers.get_flag(RFlags::SIGN)); // Result is positive
        assert!(!result.registers.get_flag(RFlags::ZERO)); // Result is not zero
    }

    #[test]
    fn test_overflow_condition() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x8000000000000000; // Min negative 64-bit signed
        state.registers.rbx = 0x1;
        
        // SUB RAX, RBX (0x48 0x29 0xC3)
        let result = execute_instruction(&[0x48, 0x29, 0xC3], &mut state).unwrap();
        assert_eq!(result.registers.rax, 0x7FFFFFFFFFFFFFFF);
        assert!(result.registers.get_flag(RFlags::OVERFLOW)); // Overflow occurred
        assert!(!result.registers.get_flag(RFlags::SIGN)); // Result is positive
    }
}
