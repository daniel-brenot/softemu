use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};
use crate::memory::guest_memory::GuestMemory;
use crate::Result;

fn create_test_cpu_state() -> Result<CpuState> {
    let memory = GuestMemory::new(1024 * 1024)?; // 1MB of memory
    let mut state = CpuState::new(memory);
    // Initialize stack pointer to a safe location
    state.registers.rsp = 0x10000;
    Ok(state)
}

fn execute_instruction(instruction_bytes: &[u8], state: &mut CpuState) -> Result<CpuState> {
    let mut decoder = InstructionDecoder::new();
    let instruction = decoder.decode_instruction(instruction_bytes);
    decoder.execute_instruction(&instruction, state)?;
    Ok(state.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_instruction_register() {
        // PUSH RAX (0x50)
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x123456789ABCDEF0;
        let initial_sp = state.registers.rsp;
        
        let result = execute_instruction(&[0x50], &mut state).unwrap();
        
        // Stack pointer should decrease by 8
        assert_eq!(result.registers.rsp, initial_sp - 8);
        // Value should be pushed to stack
        assert_eq!(result.read_u64(result.registers.rsp).unwrap(), 0x123456789ABCDEF0);
    }

    #[test]
    fn test_push_instruction_immediate() {
        // PUSH imm32 (0x68 0x00 0x10 0x00 0x00)
        let mut state = create_test_cpu_state().unwrap();
        let initial_sp = state.registers.rsp;
        
        let result = execute_instruction(&[0x68, 0x00, 0x10, 0x00, 0x00], &mut state).unwrap();
        
        // Stack pointer should decrease by 8
        assert_eq!(result.registers.rsp, initial_sp - 8);
        // Immediate value should be pushed to stack
        assert_eq!(result.read_u64(result.registers.rsp).unwrap(), 0x1000);
    }

    #[test]
    fn test_pop_instruction_register() {
        // POP RBX (0x5B)
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rsp = 0x10000;
        state.write_u64(0x10000, 0xFEDCBA9876543210).unwrap();
        
        let result = execute_instruction(&[0x5B], &mut state).unwrap();
        
        // Stack pointer should increase by 8
        assert_eq!(result.registers.rsp, 0x10008);
        // Value should be popped into RBX
        assert_eq!(result.registers.rbx, 0xFEDCBA9876543210);
    }

    #[test]
    fn test_push_pop_combination() {
        // Test PUSH followed by POP
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x123456789ABCDEF0;
        let initial_sp = state.registers.rsp;
        
        // PUSH RAX
        let mut result1 = execute_instruction(&[0x50], &mut state).unwrap();
        assert_eq!(result1.registers.rsp, initial_sp - 8);
        
        // POP RBX
        let result2 = execute_instruction(&[0x5B], &mut result1).unwrap();
        assert_eq!(result2.registers.rsp, initial_sp);
        assert_eq!(result2.registers.rbx, 0x123456789ABCDEF0);
    }

    #[test]
    fn test_pusha_instruction() {
        // PUSHA (0x60) - push all 16-bit registers
        // Note: PUSHA/POPA are not available in 64-bit mode
        // This test is skipped for 64-bit mode
        if cfg!(target_arch = "x86_64") {
            return;
        }
        
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x123456789ABCDEF0;
        state.registers.rcx = 0xFEDCBA9876543210;
        state.registers.rdx = 0x1111111111111111;
        state.registers.rbx = 0x2222222222222222;
        state.registers.rsp = 0x10000;
        state.registers.rbp = 0x3333333333333333;
        state.registers.rsi = 0x4444444444444444;
        state.registers.rdi = 0x5555555555555555;
        
        let result = execute_instruction(&[0x60], &mut state).unwrap();
        
        // Stack pointer should decrease by 16 (8 registers * 2 bytes each)
        assert_eq!(result.registers.rsp, 0x10000 - 16);
        
        // Check that registers were pushed in correct order (DI, SI, BP, SP, BX, DX, CX, AX)
        let mut sp = result.registers.rsp;
        assert_eq!(result.read_u16(sp).unwrap(), 0x5555); // DI
        sp += 2;
        assert_eq!(result.read_u16(sp).unwrap(), 0x4444); // SI
        sp += 2;
        assert_eq!(result.read_u16(sp).unwrap(), 0x3333); // BP
        sp += 2;
        assert_eq!(result.read_u16(sp).unwrap(), 0x1000); // SP (original value)
        sp += 2;
        assert_eq!(result.read_u16(sp).unwrap(), 0x2222); // BX
        sp += 2;
        assert_eq!(result.read_u16(sp).unwrap(), 0x1111); // DX
        sp += 2;
        assert_eq!(result.read_u16(sp).unwrap(), 0x3210); // CX
        sp += 2;
        assert_eq!(result.read_u16(sp).unwrap(), 0xDEF0); // AX
    }

    #[test]
    fn test_popa_instruction() {
        // POPA (0x61) - pop all 16-bit registers
        // Note: PUSHA/POPA are not available in 64-bit mode
        // This test is skipped for 64-bit mode
        if cfg!(target_arch = "x86_64") {
            return;
        }
        
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rsp = 0x10000;
        
        // Push test values in correct order
        state.write_u16(0x10000, 0x5555); // DI
        state.write_u16(0x10002, 0x4444); // SI
        state.write_u16(0x10004, 0x3333); // BP
        state.write_u16(0x10006, 0x2222); // SP (will be discarded)
        state.write_u16(0x10008, 0x1111); // BX
        state.write_u16(0x1000A, 0x0000); // DX
        state.write_u16(0x1000C, 0xFFFF); // CX
        state.write_u16(0x1000E, 0xAAAA); // AX
        
        let result = execute_instruction(&[0x61], &mut state).unwrap();
        
        // Stack pointer should increase by 16
        assert_eq!(result.registers.rsp, 0x10010);
        
        // Check that registers were popped correctly
        assert_eq!(result.registers.rax & 0xFFFF, 0xAAAA);
        assert_eq!(result.registers.rcx & 0xFFFF, 0xFFFF);
        assert_eq!(result.registers.rdx & 0xFFFF, 0x0000);
        assert_eq!(result.registers.rbx & 0xFFFF, 0x1111);
        assert_eq!(result.registers.rbp & 0xFFFF, 0x3333);
        assert_eq!(result.registers.rsi & 0xFFFF, 0x4444);
        assert_eq!(result.registers.rdi & 0xFFFF, 0x5555);
    }

    #[test]
    fn test_pushad_instruction() {
        // PUSHAD (0x60) - push all 32-bit registers
        // Note: PUSHAD/POPAD are not available in 64-bit mode
        // This test is skipped for 64-bit mode
        if cfg!(target_arch = "x86_64") {
            return;
        }
        
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x123456789ABCDEF0;
        state.registers.rcx = 0xFEDCBA9876543210;
        state.registers.rdx = 0x1111111111111111;
        state.registers.rbx = 0x2222222222222222;
        state.registers.rsp = 0x10000;
        state.registers.rbp = 0x3333333333333333;
        state.registers.rsi = 0x4444444444444444;
        state.registers.rdi = 0x5555555555555555;
        
        let result = execute_instruction(&[0x60], &mut state).unwrap();
        
        // Stack pointer should decrease by 32 (8 registers * 4 bytes each)
        assert_eq!(result.registers.rsp, 0x10000 - 32);
        
        // Check that registers were pushed in correct order (EDI, ESI, EBP, ESP, EBX, EDX, ECX, EAX)
        let mut sp = result.registers.rsp;
        assert_eq!(result.read_u32(sp).unwrap(), 0x55555555); // EDI
        sp += 4;
        assert_eq!(result.read_u32(sp).unwrap(), 0x44444444); // ESI
        sp += 4;
        assert_eq!(result.read_u32(sp).unwrap(), 0x33333333); // EBP
        sp += 4;
        assert_eq!(result.read_u32(sp).unwrap(), 0x10000); // ESP (original value)
        sp += 4;
        assert_eq!(result.read_u32(sp).unwrap(), 0x22222222); // EBX
        sp += 4;
        assert_eq!(result.read_u32(sp).unwrap(), 0x11111111); // EDX
        sp += 4;
        assert_eq!(result.read_u32(sp).unwrap(), 0x76543210); // ECX
        sp += 4;
        assert_eq!(result.read_u32(sp).unwrap(), 0x9ABCDEF0); // EAX
    }

    #[test]
    fn test_popad_instruction() {
        // POPAD (0x61) - pop all 32-bit registers
        // Note: PUSHAD/POPAD are not available in 64-bit mode
        // This test is skipped for 64-bit mode
        if cfg!(target_arch = "x86_64") {
            return;
        }
        
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rsp = 0x10000;
        
        // Push test values in correct order
        state.write_u32(0x10000, 0x55555555); // EDI
        state.write_u32(0x10004, 0x44444444); // ESI
        state.write_u32(0x10008, 0x33333333); // EBP
        state.write_u32(0x1000C, 0x22222222); // ESP (will be discarded)
        state.write_u32(0x10010, 0x11111111); // EBX
        state.write_u32(0x10014, 0x00000000); // EDX
        state.write_u32(0x10018, 0xFFFFFFFF); // ECX
        state.write_u32(0x1001C, 0xAAAAAAAA); // EAX
        
        let result = execute_instruction(&[0x61], &mut state).unwrap();
        
        // Stack pointer should increase by 32
        assert_eq!(result.registers.rsp, 0x10020);
        
        // Check that registers were popped correctly
        assert_eq!(result.registers.rax & 0xFFFFFFFF, 0xAAAAAAAA);
        assert_eq!(result.registers.rcx & 0xFFFFFFFF, 0xFFFFFFFF);
        assert_eq!(result.registers.rdx & 0xFFFFFFFF, 0x00000000);
        assert_eq!(result.registers.rbx & 0xFFFFFFFF, 0x11111111);
        assert_eq!(result.registers.rbp & 0xFFFFFFFF, 0x33333333);
        assert_eq!(result.registers.rsi & 0xFFFFFFFF, 0x44444444);
        assert_eq!(result.registers.rdi & 0xFFFFFFFF, 0x55555555);
    }

    #[test]
    fn test_pushf_instruction() {
        // PUSHF (0x9C) - push flags register
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rflags = 0x123456789ABCDEF0;
        let initial_sp = state.registers.rsp;
        
        let result = execute_instruction(&[0x9C], &mut state).unwrap();
        
        // Stack pointer should decrease by 8
        assert_eq!(result.registers.rsp, initial_sp - 8);
        // Flags should be pushed to stack
        assert_eq!(result.read_u64(result.registers.rsp).unwrap(), 0x123456789ABCDEF0);
    }

    #[test]
    fn test_popf_instruction() {
        // POPF (0x9D) - pop flags register
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rsp = 0x10000;
        state.write_u64(0x10000, 0xFEDCBA9876543210).unwrap();
        
        let result = execute_instruction(&[0x9D], &mut state).unwrap();
        
        // Stack pointer should increase by 8
        assert_eq!(result.registers.rsp, 0x10008);
        // Flags should be popped from stack
        assert_eq!(result.registers.rflags, 0xFEDCBA9876543210);
    }

    #[test]
    fn test_popcnt_instruction() {
        // POPCNT RAX, RBX (0xF3 0x0F 0xB8 0xC3)
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rbx = 0x123456789ABCDEF0; // 32 bits set
        
        let result = execute_instruction(&[0xF3, 0x0F, 0xB8, 0xC3], &mut state).unwrap();
        
        // RAX should contain the population count
        assert_eq!(result.registers.rax, 32);
        // Zero flag should be false (count != 0)
        assert!(!result.registers.get_flag(RFlags::ZERO));
        // Carry, overflow, and sign flags should be false
        assert!(!result.registers.get_flag(RFlags::CARRY));
        assert!(!result.registers.get_flag(RFlags::OVERFLOW));
        assert!(!result.registers.get_flag(RFlags::SIGN));
    }

    #[test]
    fn test_popcnt_zero_result() {
        // POPCNT RAX, RBX with zero input
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rbx = 0x0; // 0 bits set
        
        let result = execute_instruction(&[0xF3, 0x0F, 0xB8, 0xC3], &mut state).unwrap();
        
        // RAX should contain 0
        assert_eq!(result.registers.rax, 0);
        // Zero flag should be true (count == 0)
        assert!(result.registers.get_flag(RFlags::ZERO));
    }

    #[test]
    fn test_popcnt_all_ones() {
        // POPCNT RAX, RBX with all ones
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rbx = 0xFFFFFFFFFFFFFFFF; // 64 bits set
        
        let result = execute_instruction(&[0xF3, 0x0F, 0xB8, 0xC3], &mut state).unwrap();
        
        // RAX should contain 64
        assert_eq!(result.registers.rax, 64);
        // Zero flag should be false (count != 0)
        assert!(!result.registers.get_flag(RFlags::ZERO));
    }

    #[test]
    fn test_pause_instruction() {
        // PAUSE (0xF3 0x90)
        let mut state = create_test_cpu_state().unwrap();
        let initial_state = state.clone();
        
        let result = execute_instruction(&[0xF3, 0x90], &mut state).unwrap();
        
        // PAUSE should not modify any state
        assert_eq!(result.registers.rax, initial_state.registers.rax);
        assert_eq!(result.registers.rbx, initial_state.registers.rbx);
        assert_eq!(result.registers.rcx, initial_state.registers.rcx);
        assert_eq!(result.registers.rdx, initial_state.registers.rdx);
        assert_eq!(result.registers.rsp, initial_state.registers.rsp);
        assert_eq!(result.registers.rbp, initial_state.registers.rbp);
        assert_eq!(result.registers.rsi, initial_state.registers.rsi);
        assert_eq!(result.registers.rdi, initial_state.registers.rdi);
        assert_eq!(result.registers.rflags, initial_state.registers.rflags);
    }

    #[test]
    fn test_stack_overflow_protection() {
        // Test that stack operations don't cause issues at boundaries
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rsp = 0x1000; // Near boundary
        state.registers.rax = 0x123456789ABCDEF0;
        
        // PUSH should work
        let result = execute_instruction(&[0x50], &mut state).unwrap();
        assert_eq!(result.registers.rsp, 0x1000 - 8);
        assert_eq!(result.read_u64(result.registers.rsp).unwrap(), 0x123456789ABCDEF0);
    }

    #[test]
    fn test_multiple_push_pop_sequence() {
        // Test multiple PUSH/POP operations
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x1111111111111111;
        state.registers.rbx = 0x2222222222222222;
        state.registers.rcx = 0x3333333333333333;
        let initial_sp = state.registers.rsp;
        
        // PUSH RAX, RBX, RCX
        let mut result1 = execute_instruction(&[0x50], &mut state).unwrap(); // PUSH RAX
        let mut result2 = execute_instruction(&[0x53], &mut result1).unwrap(); // PUSH RBX
        let mut result3 = execute_instruction(&[0x51], &mut result2).unwrap(); // PUSH RCX
        
        assert_eq!(result3.registers.rsp, initial_sp - 24);
        
        // POP RCX, RBX, RAX (reverse order)
        let mut result4 = execute_instruction(&[0x59], &mut result3).unwrap(); // POP RCX
        let mut result5 = execute_instruction(&[0x5B], &mut result4).unwrap(); // POP RBX
        let result6 = execute_instruction(&[0x58], &mut result5).unwrap(); // POP RAX
        
        assert_eq!(result6.registers.rsp, initial_sp);
        assert_eq!(result6.registers.rax, 0x1111111111111111);
        assert_eq!(result6.registers.rbx, 0x2222222222222222);
        assert_eq!(result6.registers.rcx, 0x3333333333333333);
    }

    #[test]
    fn test_flags_preservation() {
        // Test that PUSH/POP operations preserve flags
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rflags = 0x123456789ABCDEF0;
        state.registers.rax = 0x1111111111111111;
        
        // PUSH RAX
        let mut result1 = execute_instruction(&[0x50], &mut state).unwrap();
        assert_eq!(result1.registers.rflags, 0x123456789ABCDEF0);
        
        // POP RBX
        let result2 = execute_instruction(&[0x5B], &mut result1).unwrap();
        assert_eq!(result2.registers.rflags, 0x123456789ABCDEF0);
        assert_eq!(result2.registers.rbx, 0x1111111111111111);
    }

    #[test]
    fn test_popcnt_various_values() {
        // Test POPCNT with various bit patterns
        let test_cases = vec![
            (0x0000000000000001, 1),   // Single bit
            (0x0000000000000003, 2),   // Two bits
            (0x000000000000000F, 4),   // Four bits
            (0x00000000000000FF, 8),   // Eight bits
            (0x000000000000FFFF, 16),  // Sixteen bits
            (0x00000000FFFFFFFF, 32),  // Thirty-two bits
            (0x5555555555555555, 32),  // Alternating pattern
            (0xAAAAAAAAAAAAAAAA, 32),  // Alternating pattern
        ];
        
        for (input, expected_count) in test_cases {
            let mut state = create_test_cpu_state().unwrap();
            state.registers.rbx = input;
            
            let result = execute_instruction(&[0xF3, 0x0F, 0xB8, 0xC3], &mut state).unwrap();
            
            assert_eq!(result.registers.rax, expected_count);
            if expected_count == 0 {
                assert!(result.registers.get_flag(RFlags::ZERO));
            } else {
                assert!(!result.registers.get_flag(RFlags::ZERO));
            }
        }
    }
}