use crate::cpu::instruction::InstructionDecoder;
use crate::cpu::state::CpuState;
use crate::cpu::registers::RFlags;
use crate::Result;
use iced_x86::{Decoder, DecoderOptions, Instruction};
use crate::test::helpers::{create_test_cpu_state, write_memory, read_memory};

fn decode_instruction(bytes: &[u8]) -> Instruction {
    let mut decoder = Decoder::new(64, bytes, DecoderOptions::NONE);
    decoder.decode()
}

fn execute_instruction(bytes: &[u8], state: &mut CpuState) -> Result<()> {
    let decoder = InstructionDecoder::new();
    let instruction = decode_instruction(bytes);
    decoder.execute_instruction(&instruction, state)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        let initial_rax = 0x123456789ABCDEF0;
        state.registers.rax = initial_rax;
        
        // NOP instruction (0x90)
        let result = execute_instruction(&[0x90], &mut state);
        
        // NOP should not change any registers
        assert_eq!(state.registers.rax, initial_rax);
    }

    #[test]
    fn test_nop_instruction_multiple() {
        let mut state = create_test_cpu_state().unwrap();
        let initial_rax = 0x123456789ABCDEF0;
        let initial_rbx = 0xFEDCBA9876543210;
        state.registers.rax = initial_rax;
        state.registers.rbx = initial_rbx;
        
        // Multiple NOP instructions
        let result = execute_instruction(&[0x90, 0x90, 0x90], &mut state);
        
        // NOP should not change any registers
        assert_eq!(state.registers.rax, initial_rax);
        assert_eq!(state.registers.rbx, initial_rbx);
    }

    #[test]
    fn test_neg_instruction_register() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 5;
        
        // NEG RAX (0x48, 0xF7, 0xD8)
        let result = execute_instruction(&[0x48, 0xF7, 0xD8], &mut state);
        
        // NEG 5 = -5 = 0xFFFFFFFFFFFFFFFB
        assert_eq!(state.registers.rax, 0xFFFFFFFFFFFFFFFB);
        
        // Debug: Check what flags are actually set
        println!("CARRY flag: {}", state.registers.get_flag(RFlags::CARRY));
        println!("OVERFLOW flag: {}", state.registers.get_flag(RFlags::OVERFLOW));
        println!("All flags: {:?}", state.registers.rflags);
        
        // Check flags - NEG should set CARRY flag for non-zero results, OVERFLOW only for most negative value
        assert!(state.registers.get_flag(RFlags::CARRY));
        assert!(!state.registers.get_flag(RFlags::OVERFLOW)); // 5 is not the most negative value
    }

    #[test]
    fn test_neg_instruction_most_negative() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x8000000000000000; // Most negative 64-bit value
        
        // NEG RAX (0x48, 0xF7, 0xD8)
        let result = execute_instruction(&[0x48, 0xF7, 0xD8], &mut state);
        
        // NEG of most negative value should overflow and result in the same value
        assert_eq!(state.registers.rax, 0x8000000000000000);
        
        // Check flags - CARRY should be set (non-zero), OVERFLOW should be set (most negative value)
        assert!(state.registers.get_flag(RFlags::CARRY));
        assert!(state.registers.get_flag(RFlags::OVERFLOW));
    }

    #[test]
    fn test_neg_instruction_zero() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0;
        
        // NEG RAX (0x48, 0xF7, 0xD8)
        let result = execute_instruction(&[0x48, 0xF7, 0xD8], &mut state);
        
        // NEG 0 = 0
        assert_eq!(state.registers.rax, 0);
        
        // Check flags - NEG of zero should NOT set CARRY flag (zero is zero), and should NOT set OVERFLOW flag
        assert!(!state.registers.get_flag(RFlags::CARRY));
        assert!(!state.registers.get_flag(RFlags::OVERFLOW));
    }

    #[test]
    fn test_neg_instruction_negative() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0xFFFFFFFFFFFFFFFB; // -5
        
        // NEG RAX (0x48, 0xF7, 0xD8)
        let result = execute_instruction(&[0x48, 0xF7, 0xD8], &mut state);
        
        // NEG (-5) = 5
        assert_eq!(state.registers.rax, 5);
        
        // Check flags - NEG should set CARRY flag for non-zero results, but NOT OVERFLOW (not most negative value)
        assert!(state.registers.get_flag(RFlags::CARRY));
        assert!(!state.registers.get_flag(RFlags::OVERFLOW));
    }

    #[test]
    fn test_neg_instruction_memory() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x1000; // Memory address
        write_memory(&mut state, 0x1000, 10).unwrap();
        
        // NEG QWORD PTR [RAX] (0x48, 0xF7, 0x18)
        let result = execute_instruction(&[0x48, 0xF7, 0x18], &mut state);
        
        // NEG 10 = -10 = 0xFFFFFFFFFFFFFFF6
        let value = read_memory(&state, 0x1000).unwrap();
        assert_eq!(value, 0xFFFFFFFFFFFFFFF6);
        
        // Check flags - CARRY should be set (non-zero), OVERFLOW should NOT be set (not most negative value)
        assert!(state.registers.get_flag(RFlags::CARRY));
        assert!(!state.registers.get_flag(RFlags::OVERFLOW));
    }

    #[test]
    fn test_not_instruction_register() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x123456789ABCDEF0;
        
        // NOT RAX (0x48, 0xF7, 0xD0)
        let result = execute_instruction(&[0x48, 0xF7, 0xD0], &mut state);
        
        // NOT should flip all bits
        assert_eq!(state.registers.rax, 0xEDCBA9876543210F);
        
        // NOT doesn't affect flags
        assert!(!state.registers.get_flag(RFlags::CARRY));
        assert!(!state.registers.get_flag(RFlags::OVERFLOW));
    }

    #[test]
    fn test_not_instruction_zero() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0;
        
        // NOT RAX (0x48, 0xF7, 0xD0)
        let result = execute_instruction(&[0x48, 0xF7, 0xD0], &mut state);
        
        // NOT 0 = 0xFFFFFFFFFFFFFFFF
        assert_eq!(state.registers.rax, 0xFFFFFFFFFFFFFFFF);
        
        // NOT doesn't affect flags
        assert!(!state.registers.get_flag(RFlags::CARRY));
        assert!(!state.registers.get_flag(RFlags::OVERFLOW));
    }

    #[test]
    fn test_not_instruction_all_ones() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0xFFFFFFFFFFFFFFFF;
        
        // NOT RAX (0x48, 0xF7, 0xD0)
        let result = execute_instruction(&[0x48, 0xF7, 0xD0], &mut state);
        
        // NOT 0xFFFFFFFFFFFFFFFF = 0
        assert_eq!(state.registers.rax, 0);
        
        // NOT doesn't affect flags
        assert!(!state.registers.get_flag(RFlags::CARRY));
        assert!(!state.registers.get_flag(RFlags::OVERFLOW));
    }

    #[test]
    fn test_not_instruction_memory() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x1000; // Memory address
        write_memory(&mut state, 0x1000, 0x123456789ABCDEF0).unwrap();
        
        // NOT QWORD PTR [RAX] (0x48, 0xF7, 0x10)
        let result = execute_instruction(&[0x48, 0xF7, 0x10], &mut state);
        
        // NOT should flip all bits
        let value = read_memory(&state, 0x1000).unwrap();
        assert_eq!(value, 0xEDCBA9876543210F);
        
        // NOT doesn't affect flags
        assert!(!state.registers.get_flag(RFlags::CARRY));
        assert!(!state.registers.get_flag(RFlags::OVERFLOW));
    }

    #[test]
    fn test_neg_instruction_32bit() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 5; // Set EAX (lower 32 bits of RAX)
        
        // NEG EAX (0xF7, 0xD8)
        let result = execute_instruction(&[0xF7, 0xD8], &mut state);
        
        // NEG 5 = -5 = 0xFFFFFFFB (32-bit result in lower 32 bits of RAX)
        assert_eq!(state.registers.rax & 0xFFFFFFFF, 0xFFFFFFFB);
        
        // Check flags - CARRY should be set (non-zero), OVERFLOW should NOT be set (5 is not 0x80000000)
        assert!(state.registers.get_flag(RFlags::CARRY));
        assert!(!state.registers.get_flag(RFlags::OVERFLOW));
    }

    #[test]
    fn test_not_instruction_32bit() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x12345678; // Set EAX (lower 32 bits of RAX)
        
        // NOT EAX (0xF7, 0xD0)
        let result = execute_instruction(&[0xF7, 0xD0], &mut state);
        
        // NOT should flip all bits (32-bit result in lower 32 bits of RAX)
        assert_eq!(state.registers.rax & 0xFFFFFFFF, 0xEDCBA987);
        
        // NOT doesn't affect flags
        assert!(!state.registers.get_flag(RFlags::CARRY));
        assert!(!state.registers.get_flag(RFlags::OVERFLOW));
    }

    #[test]
    fn test_neg_instruction_16bit() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 5; // Set AX (lower 16 bits of RAX)
        
        // NEG AX (0x66, 0xF7, 0xD8)
        let result = execute_instruction(&[0x66, 0xF7, 0xD8], &mut state);
        
        // NEG 5 = -5 = 0xFFFB (16-bit result in lower 16 bits of RAX)
        assert_eq!(state.registers.rax & 0xFFFF, 0xFFFB);
        
        // Check flags - CARRY should be set (non-zero), OVERFLOW should NOT be set (5 is not 0x8000)
        assert!(state.registers.get_flag(RFlags::CARRY));
        assert!(!state.registers.get_flag(RFlags::OVERFLOW));
    }

    #[test]
    fn test_not_instruction_16bit() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x1234; // Set AX (lower 16 bits of RAX)
        
        // NOT AX (0x66, 0xF7, 0xD0)
        let result = execute_instruction(&[0x66, 0xF7, 0xD0], &mut state);
        
        // NOT should flip all bits (16-bit result in lower 16 bits of RAX)
        assert_eq!(state.registers.rax & 0xFFFF, 0xEDCB);
        
        // NOT doesn't affect flags
        assert!(!state.registers.get_flag(RFlags::CARRY));
        assert!(!state.registers.get_flag(RFlags::OVERFLOW));
    }

    #[test]
    fn test_neg_instruction_8bit() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 5; // Set AL (lower 8 bits of RAX)
        
        // NEG AL (0xF6, 0xD8)
        let result = execute_instruction(&[0xF6, 0xD8], &mut state);
        
        // NEG 5 = -5 = 0xFB (8-bit result in lower 8 bits of RAX)
        assert_eq!(state.registers.rax & 0xFF, 0xFB);
        
        // Check flags - CARRY should be set (non-zero), OVERFLOW should NOT be set (5 is not 0x80)
        assert!(state.registers.get_flag(RFlags::CARRY));
        assert!(!state.registers.get_flag(RFlags::OVERFLOW));
    }

    #[test]
    fn test_not_instruction_8bit() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x12; // Set AL (lower 8 bits of RAX)
        
        // NOT AL (0xF6, 0xD0)
        let result = execute_instruction(&[0xF6, 0xD0], &mut state);
        
        // NOT should flip all bits (8-bit result in lower 8 bits of RAX)
        assert_eq!(state.registers.rax & 0xFF, 0xED);
        
        // NOT doesn't affect flags
        assert!(!state.registers.get_flag(RFlags::CARRY));
        assert!(!state.registers.get_flag(RFlags::OVERFLOW));
    }

    #[test]
    fn test_nop_variants() {
        let initial_rax = 0x123456789ABCDEF0;
        
        // Test NOPW (0x66, 0x90) - 16-bit NOP
        let mut state1 = create_test_cpu_state().unwrap();
        state1.registers.rax = initial_rax;
        let result1 = execute_instruction(&[0x66, 0x90], &mut state1);
        assert_eq!(state1.registers.rax, initial_rax);
        
        // Test NOPL (0x90) - 32-bit NOP (same as regular NOP)
        let mut state2 = create_test_cpu_state().unwrap();
        state2.registers.rax = initial_rax;
        let result2 = execute_instruction(&[0x90], &mut state2);
        assert_eq!(state2.registers.rax, initial_rax);
        
        // Test NOPQ (0x48, 0x90) - 64-bit NOP
        let mut state3 = create_test_cpu_state().unwrap();
        state3.registers.rax = initial_rax;
        let result3 = execute_instruction(&[0x48, 0x90], &mut state3);
        assert_eq!(state3.registers.rax, initial_rax);
    }

    #[test]
    fn test_neg_instruction_edge_cases() {
        let mut state = create_test_cpu_state().unwrap();
        
        // Test NEG of minimum negative value
        state.registers.rax = 0x8000000000000000; // Minimum 64-bit negative value
        let result = execute_instruction(&[0x48, 0xF7, 0xD8], &mut state);
        
        // NEG of minimum negative value should overflow
        assert_eq!(state.registers.rax, 0x8000000000000000);
        assert!(state.registers.get_flag(RFlags::CARRY));
        assert!(state.registers.get_flag(RFlags::OVERFLOW));
    }

    #[test]
    fn test_not_instruction_edge_cases() {
        let mut state = create_test_cpu_state().unwrap();
        
        // Test NOT of alternating pattern
        state.registers.rax = 0xAAAAAAAAAAAAAAAA;
        let result = execute_instruction(&[0x48, 0xF7, 0xD0], &mut state);
        
        // NOT should flip all bits
        assert_eq!(state.registers.rax, 0x5555555555555555);
        
        // NOT doesn't affect flags
        assert!(!state.registers.get_flag(RFlags::CARRY));
        assert!(!state.registers.get_flag(RFlags::OVERFLOW));
    }

    #[test]
    fn test_instruction_error_handling() {
        let mut state = create_test_cpu_state().unwrap();
        let decoder = InstructionDecoder::new();
        
        // Test with invalid instruction bytes
        let invalid_bytes = &[0xFF, 0xFF, 0xFF, 0xFF];
        let instruction = decode_instruction(invalid_bytes);
        
        // This should not panic, but may return an error
        let result = decoder.execute_instruction(&instruction, &mut state);
        // We don't assert the result here as it depends on the specific invalid instruction
    }

    #[test]
    fn test_memory_boundary_conditions() {
        let mut state = create_test_cpu_state().unwrap();
        
        // Test NEG at memory boundary
        state.registers.rax = 0xF0000; // Near end of 1MB memory
        write_memory(&mut state, 0xF0000, 10).unwrap();
        
        // NEG QWORD PTR [RAX] (0x48, 0xF7, 0x18)
        let result = execute_instruction(&[0x48, 0xF7, 0x18], &mut state);
        
        // Should work correctly
        let value = read_memory(&state, 0xF0000).unwrap();
        assert_eq!(value, 0xFFFFFFFFFFFFFFF6);
    }

    #[test]
    fn test_neg_not_combination() {
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 5;
        
        // First NEG: 5 -> -5
        let result1 = execute_instruction(&[0x48, 0xF7, 0xD8], &mut state);
        assert_eq!(state.registers.rax, 0xFFFFFFFFFFFFFFFB);
        
        // Then NOT: -5 -> 4
        let result2 = execute_instruction(&[0x48, 0xF7, 0xD0], &mut state);
        assert_eq!(state.registers.rax, 4);
    }

    #[test]
    fn test_nop_instruction_sequence() {
        let mut state = create_test_cpu_state().unwrap();
        let initial_rax = 0x123456789ABCDEF0;
        let initial_rbx = 0xFEDCBA9876543210;
        state.registers.rax = initial_rax;
        state.registers.rbx = initial_rbx;
        
        // Sequence of NOP instructions
        let result = execute_instruction(&[0x90, 0x90, 0x90, 0x90, 0x90], &mut state);
        
        // All registers should remain unchanged
        assert_eq!(state.registers.rax, initial_rax);
        assert_eq!(state.registers.rbx, initial_rbx);
    }
}
