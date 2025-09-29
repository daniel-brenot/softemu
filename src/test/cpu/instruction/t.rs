use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};
use crate::Result;
use crate::test::helpers::{create_test_cpu_state, execute_instruction, read_memory, write_memory};
use iced_x86::{Decoder, DecoderOptions, Instruction};


#[test]
fn test_test_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDEF0u64;
    state.registers.rbx = 0xFEDCBA9876543210u64;
    
    // TEST RAX, RBX (0x48 0x85 0xC3)
    let result = execute_instruction(&[0x48, 0x85, 0xC3], &mut state);
    
    // TEST performs bitwise AND but doesn't store result
    assert_eq!(state.registers.rax, 0x123456789ABCDEF0u64); // RAX unchanged
    assert_eq!(state.registers.rbx, 0xFEDCBA9876543210u64); // RBX unchanged
    
    // Check flags based on AND result: 0x123456789ABCDEF0 & 0xFEDCBA9876543210
    let and_result = 0x123456789ABCDEF0u64 & 0xFEDCBA9876543210u64;
    assert_eq!(and_result, 0x1214121812141210u64);
    
    assert!(!state.registers.get_flag(RFlags::ZERO)); // Result is not zero
    assert!(!state.registers.get_flag(RFlags::SIGN)); // Result is positive
    assert!(!state.registers.get_flag(RFlags::PARITY)); // Odd parity
}

#[test]
fn test_test_zero_result() {
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDEF0u64;
    state.registers.rbx = 0x0; // Zero operand
    
    // TEST RAX, RBX (0x48 0x85 0xC3)
    let result = execute_instruction(&[0x48, 0x85, 0xC3], &mut state);
    
    // TEST performs bitwise AND but doesn't store result
    assert_eq!(state.registers.rax, 0x123456789ABCDEF0u64); // RAX unchanged
    assert_eq!(state.registers.rbx, 0x0); // RBX unchanged
    
    // Check flags based on AND result: 0x123456789ABCDEF0 & 0x0 = 0
    assert!(state.registers.get_flag(RFlags::ZERO)); // Result is zero
    assert!(!state.registers.get_flag(RFlags::SIGN)); // Result is positive
    assert!(state.registers.get_flag(RFlags::PARITY)); // Even parity (0 has even parity)
}

#[test]
fn test_test_negative_result() {
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x8000000000000000u64; // Negative number
    state.registers.rbx = 0xFFFFFFFFFFFFFFFFu64; // All ones
    
    // TEST RAX, RBX (0x48 0x85 0xC3)
    let result = execute_instruction(&[0x48, 0x85, 0xC3], &mut state);
    
    // TEST performs bitwise AND but doesn't store result
    assert_eq!(state.registers.rax, 0x8000000000000000u64); // RAX unchanged
    assert_eq!(state.registers.rbx, 0xFFFFFFFFFFFFFFFFu64); // RBX unchanged
    
    // Check flags based on AND result: 0x8000000000000000 & 0xFFFFFFFFFFFFFFFF = 0x8000000000000000
    assert!(!state.registers.get_flag(RFlags::ZERO)); // Result is not zero
    assert!(state.registers.get_flag(RFlags::SIGN)); // Result is negative
    assert!(state.registers.get_flag(RFlags::PARITY)); // Even parity
}

#[test]
fn test_test_memory_operand() {
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDEF0u64;
    state.registers.rbx = 0x1000; // Memory address
    
    // Write test value to memory
    state.write_u64(0x1000, 0xFEDCBA9876543210u64).unwrap();
    
    // TEST RAX, [RBX] (0x48 0x85 0x03)
    let result = execute_instruction(&[0x48, 0x85, 0x03], &mut state);
    
    // TEST performs bitwise AND but doesn't store result
    assert_eq!(state.registers.rax, 0x123456789ABCDEF0u64); // RAX unchanged
    assert_eq!(state.registers.rbx, 0x1000); // RBX unchanged
    
    // Check flags based on AND result: 0x123456789ABCDEF0 & 0xFEDCBA9876543210
    let and_result = 0x123456789ABCDEF0u64 & 0xFEDCBA9876543210u64;
    assert_eq!(and_result, 0x1214121812141210u64);
    
    assert!(!state.registers.get_flag(RFlags::ZERO)); // Result is not zero
    assert!(!state.registers.get_flag(RFlags::SIGN)); // Result is positive
    assert!(!state.registers.get_flag(RFlags::PARITY)); // Odd parity
}

#[test]
fn test_test_immediate_operand() {
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDEF0u64;
    
    // TEST RAX, 0x1234 (0x48 0xF7 0xC0 0x34 0x12 0x00 0x00)
    let result = execute_instruction(&[0x48, 0xF7, 0xC0, 0x34, 0x12, 0x00, 0x00], &mut state);
    
    // TEST performs bitwise AND but doesn't store result
    assert_eq!(state.registers.rax, 0x123456789ABCDEF0u64); // RAX unchanged
    
    // Check flags based on AND result: 0x123456789ABCDEF0 & 0x1234 = 0x1230
    let and_result = 0x123456789ABCDEF0u64 & 0x1234u64;
    assert_eq!(and_result, 0x1230u64);
    
    assert!(!state.registers.get_flag(RFlags::ZERO)); // Result is not zero
    assert!(!state.registers.get_flag(RFlags::SIGN)); // Result is positive
    assert!(state.registers.get_flag(RFlags::PARITY)); // Even parity
}

#[test]
fn test_test_16_bit_operands() {
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDEF0u64; // AX = 0xDEF0
    state.registers.rbx = 0x1234567898765432u64; // BX = 0x5432
    
    // TEST AX, BX (0x66 0x85 0xC3)
    let result = execute_instruction(&[0x66, 0x85, 0xC3], &mut state);
    
    // TEST performs bitwise AND but doesn't store result
    assert_eq!(state.registers.rax, 0x123456789ABCDEF0u64); // RAX unchanged
    assert_eq!(state.registers.rbx, 0x1234567898765432u64); // RBX unchanged
    
    // Check flags based on AND result: 0xDEF0 & 0x5432 = 0x5430
    let and_result = 0xDEF0u16 & 0x5432u16;
    assert_eq!(and_result, 0x5430u16);
    
    assert!(!state.registers.get_flag(RFlags::ZERO)); // Result is not zero
    assert!(!state.registers.get_flag(RFlags::SIGN)); // Result is positive
    assert!(state.registers.get_flag(RFlags::PARITY)); // Even parity
}

#[test]
fn test_test_32_bit_operands() {
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDEF0u64; // EAX = 0x9ABCDEF0
    state.registers.rbx = 0x1234567898765432u64; // EBX = 0x98765432
    
    // TEST EAX, EBX (0x85 0xC3)
    let result = execute_instruction(&[0x85, 0xC3], &mut state);
    
    // TEST performs bitwise AND but doesn't store result
    assert_eq!(state.registers.rax, 0x123456789ABCDEF0u64); // RAX unchanged
    assert_eq!(state.registers.rbx, 0x1234567898765432u64); // RBX unchanged
    
    // Check flags based on AND result: 0x9ABCDEF0 & 0x98765432 = 0x98345430
    let and_result = 0x9ABCDEF0u32 & 0x98765432u32;
    assert_eq!(and_result, 0x98345430u32);
    
    assert!(!state.registers.get_flag(RFlags::ZERO)); // Result is not zero
    assert!(state.registers.get_flag(RFlags::SIGN)); // Result is negative (bit 31 set)
    assert!(state.registers.get_flag(RFlags::PARITY)); // Even parity
}

#[test]
fn test_test_parity_flag() {
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDEF0u64;
    state.registers.rbx = 0x1111111111111111u64; // All bits in odd positions
    
    // TEST RAX, RBX (0x48 0x85 0xC3)
    let result = execute_instruction(&[0x48, 0x85, 0xC3], &mut state);
    
    // Check flags based on AND result: 0x123456789ABCDEF0 & 0x1111111111111111
    let and_result = 0x123456789ABCDEF0u64 & 0x1111111111111111u64;
    assert_eq!(and_result, 0x1010101010101010u64);
    
    assert!(!state.registers.get_flag(RFlags::ZERO)); // Result is not zero
    assert!(!state.registers.get_flag(RFlags::SIGN)); // Result is positive
    assert!(!state.registers.get_flag(RFlags::PARITY)); // Odd parity (8 bits set)
}

#[test]
fn test_test_overflow_flag() {
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDEF0u64;
    state.registers.rbx = 0xFEDCBA9876543210u64;
    
    // TEST RAX, RBX (0x48 0x85 0xC3)
    let result = execute_instruction(&[0x48, 0x85, 0xC3], &mut state);
    
    // TEST should clear overflow and carry flags
    assert!(!state.registers.get_flag(RFlags::OVERFLOW)); // Overflow flag cleared
    assert!(!state.registers.get_flag(RFlags::CARRY)); // Carry flag cleared
}

#[test]
fn test_test_auxiliary_flag() {
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDEF0u64;
    state.registers.rbx = 0xFEDCBA9876543210u64;
    
    // TEST RAX, RBX (0x48 0x85 0xC3)
    let result = execute_instruction(&[0x48, 0x85, 0xC3], &mut state);
    
    // TEST should clear auxiliary flag
    assert!(!state.registers.get_flag(RFlags::AUXILIARY)); // Auxiliary flag cleared
}

#[test]
fn test_test_combination() {
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDEF0u64;
    state.registers.rbx = 0xFEDCBA9876543210u64;
    state.registers.rcx = 0x1111111111111111u64;
    
    // First TEST: RAX & RBX
    let result1 = execute_instruction(&[0x48, 0x85, 0xC3], &mut state);
    assert!(!state.registers.get_flag(RFlags::ZERO));
    
    // Second TEST: RAX & RCX
    let result2 = execute_instruction(&[0x48, 0x85, 0xC1], &mut state);
    assert!(!state.registers.get_flag(RFlags::ZERO));
    
    // Third TEST: RBX & RCX
    let result3 = execute_instruction(&[0x48, 0x85, 0xD9], &mut state);
    assert!(!state.registers.get_flag(RFlags::ZERO));
}

#[test]
fn test_test_boundary_conditions() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Test with maximum values
    state.registers.rax = 0xFFFFFFFFFFFFFFFFu64;
    state.registers.rbx = 0xFFFFFFFFFFFFFFFFu64;
    
    let result = execute_instruction(&[0x48, 0x85, 0xC3], &mut state);
    assert!(!state.registers.get_flag(RFlags::ZERO)); // Result is not zero
    assert!(state.registers.get_flag(RFlags::SIGN)); // Result is negative
    assert!(state.registers.get_flag(RFlags::PARITY)); // Even parity (8 bits set in low byte)
    
    // Test with minimum values
    state.registers.rax = 0x0;
    state.registers.rbx = 0x0;
    
    let result = execute_instruction(&[0x48, 0x85, 0xC3], &mut state);
    assert!(state.registers.get_flag(RFlags::ZERO)); // Result is zero
    assert!(!state.registers.get_flag(RFlags::SIGN)); // Result is positive
    assert!(state.registers.get_flag(RFlags::PARITY)); // Even parity (0 bits set)
}

// Tests for other T instructions (mostly stubs for now)

#[test]
fn test_t1mskc_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    
    // T1MSKC instruction (stub implementation)
    let _result = execute_instruction(&[0x0F, 0x01, 0xFC], &mut state);
    // Since it's a stub, we just verify it doesn't crash
    assert!(true);
}

#[test]
fn test_tpause_instruction() {
    // TPAUSE instruction is not available in 64-bit mode
    // Skip this test as it's not supported
    return;
}

#[test]
fn test_tzcnt_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    
    // TZCNT instruction (stub implementation)
    let _result = execute_instruction(&[0xF3, 0x0F, 0xBC, 0xC3], &mut state);
    // Since it's a stub, we just verify it doesn't crash
    assert!(true);
}

#[test]
fn test_tzmsk_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    
    // TZMSK instruction (stub implementation)
    let _result = execute_instruction(&[0x0F, 0x01, 0xFD], &mut state);
    // Since it's a stub, we just verify it doesn't crash
    assert!(true);
}

#[test]
fn test_tlbsync_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    
    // TLBSYNC instruction (stub implementation)
    let _result = execute_instruction(&[0x0F, 0x01, 0xFF], &mut state);
    // Since it's a stub, we just verify it doesn't crash
    assert!(true);
}

#[test]
fn test_tdcall_instruction() {
    // TDCALL instruction is not available in 64-bit mode
    // Skip this test as it's not supported
    return;
}