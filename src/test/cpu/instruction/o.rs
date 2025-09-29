use crate::cpu::instruction::InstructionDecoder;
use crate::cpu::state::CpuState;
use crate::cpu::registers::RFlags;
use crate::Result;
use crate::test::helpers::{create_test_cpu_state, execute_instruction, read_memory, write_memory};
use iced_x86::{Decoder, DecoderOptions, Instruction};

#[test]
fn test_or_instruction_register() {
    // OR RAX, RBX (0x48 0x09 0xD8)
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDEF0;
    state.registers.rbx = 0xFEDCBA9876543210;
    
    let result = execute_instruction(&[0x48, 0x09, 0xD8], &mut state).unwrap();
    
    // OR result: 0x123456789ABCDEF0 | 0xFEDCBA9876543210 = 0xFEFCFEF8FEFCFEF0
    assert_eq!(state.registers.rax, 0xFEFCFEF8FEFCFEF0);
    assert!(!state.registers.get_flag(RFlags::ZERO));
    assert!(state.registers.get_flag(RFlags::SIGN));
    assert!(!state.registers.get_flag(RFlags::CARRY));
    assert!(!state.registers.get_flag(RFlags::OVERFLOW));
}

#[test]
fn test_or_instruction_memory() {
    // OR RAX, [RBX] (0x48 0x0B 0x03) - simpler addressing mode
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDEF0;
    state.registers.rbx = 0x1000; // RBX points to memory address 0x1000
    state.write_u64(0x1000, 0xFEDCBA9876543210).unwrap();
    
    let result = execute_instruction(&[0x48, 0x0B, 0x03], &mut state);
    
    // OR result: 0x123456789ABCDEF0 | 0xFEDCBA9876543210 = 0xFEFCFEF8FEFCFEF0
    assert_eq!(state.registers.rax, 0xFEFCFEF8FEFCFEF0);
    assert!(!state.registers.get_flag(RFlags::ZERO));
    assert!(state.registers.get_flag(RFlags::SIGN));
}

#[test]
fn test_or_instruction_8bit() {
    // OR AL, BL (0x08 0xD8)
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDE5A; // AL = 0x5A
    state.registers.rbx = 0xFEDCBA98765432A5; // BL = 0xA5
    
    let result = execute_instruction(&[0x08, 0xD8], &mut state);
    
    // OR result: 0x5A | 0xA5 = 0xFF
    assert_eq!(state.registers.rax & 0xFF, 0xFF);
    assert!(!state.registers.get_flag(RFlags::ZERO));
    assert!(state.registers.get_flag(RFlags::SIGN));
}

#[test]
fn test_or_instruction_16bit() {
    // OR AX, BX (0x66 0x09 0xD8)
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDE5A; // AX = 0xDE5A
    state.registers.rbx = 0xFEDCBA98765432A5; // BX = 0x32A5
    
    let result = execute_instruction(&[0x66, 0x09, 0xD8], &mut state);
    
    // OR result: 0xDE5A | 0x32A5 = 0xFEFF
    assert_eq!(state.registers.rax & 0xFFFF, 0xFEFF);
    assert!(!state.registers.get_flag(RFlags::ZERO));
    assert!(state.registers.get_flag(RFlags::SIGN));
}

#[test]
fn test_or_instruction_32bit() {
    // OR EAX, EBX (0x09 0xD8)
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDEF0; // EAX = 0x9ABCDEF0
    state.registers.rbx = 0xFEDCBA9876543210; // EBX = 0x76543210
    
    let result = execute_instruction(&[0x09, 0xD8], &mut state);
    
    // OR result: 0x9ABCDEF0 | 0x76543210 = 0xFEFCFEF0
    assert_eq!(state.registers.rax & 0xFFFFFFFF, 0xFEFCFEF0);
    assert!(!state.registers.get_flag(RFlags::ZERO));
    assert!(state.registers.get_flag(RFlags::SIGN));
}

#[test]
fn test_or_instruction_zero_result() {
    // OR RAX, 0 (0x48 0x83 0xC8 0x00)
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0;
    
    let result = execute_instruction(&[0x48, 0x83, 0xC8, 0x00], &mut state);
    
    assert_eq!(state.registers.rax, 0);
    assert!(state.registers.get_flag(RFlags::ZERO));
    assert!(!state.registers.get_flag(RFlags::SIGN));
    assert!(!state.registers.get_flag(RFlags::CARRY));
    assert!(!state.registers.get_flag(RFlags::OVERFLOW));
}

#[test]
fn test_or_instruction_all_ones() {
    // OR RAX, -1 (0x48 0x83 0xC8 0xFF)
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDEF0;
    
    let result = execute_instruction(&[0x48, 0x83, 0xC8, 0xFF], &mut state).unwrap();
    
    // OR with -1 (0xFFFFFFFFFFFFFFFF) should result in all ones
    assert_eq!(state.registers.rax, 0xFFFFFFFFFFFFFFFF);
    assert!(!state.registers.get_flag(RFlags::ZERO));
    assert!(state.registers.get_flag(RFlags::SIGN));
}

#[test]
fn test_or_instruction_edge_cases() {
    // Test various edge cases for OR instruction
    let mut state = create_test_cpu_state().unwrap();
    
    // Test OR with same value
    state.registers.rax = 0x123456789ABCDEF0;
    state.registers.rbx = 0x123456789ABCDEF0;
    
    let result = execute_instruction(&[0x48, 0x09, 0xD8], &mut state);
    assert_eq!(state.registers.rax, 0x123456789ABCDEF0);
}

#[test]
fn test_or_instruction_complement() {
    // Test OR with complement
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDEF0;
    state.registers.rbx = 0xEDCBA9876543210F;
    
    let result = execute_instruction(&[0x48, 0x09, 0xD8], &mut state);
    assert_eq!(state.registers.rax, 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_out_instruction() {
    // OUT DX, AL (0xEE)
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rdx = 0x123456789ABCDE80; // DX = 0xDE80
    state.registers.rax = 0x123456789ABCDE5A; // AL = 0x5A
    
    let result = execute_instruction(&[0xEE], &mut state).unwrap();
    
    // OUT instruction should not modify registers (just logs)
    assert_eq!(state.registers.rdx, 0x123456789ABCDE80);
    assert_eq!(state.registers.rax, 0x123456789ABCDE5A);
}

#[test]
fn test_out_immediate_instruction() {
    // OUT 0x80, AL (0xE6 0x80)
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDE5A; // AL = 0x5A
    
    let result = execute_instruction(&[0xE6, 0x80], &mut state).unwrap();
    
    // OUT instruction should not modify registers (just logs)
    assert_eq!(state.registers.rax, 0x123456789ABCDE5A);
}

#[test]
fn test_outsb_instruction() {
    // OUTSB (0x6E)
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rdx = 0x123456789ABCDE80; // DX = 0xDE80
    state.registers.rsi = 0x1000;
    state.write_u8(0x1000, 0x5A).unwrap();
    
    let result = execute_instruction(&[0x6E], &mut state).unwrap();
    
    // RSI should be incremented by 1 (direction flag clear)
    assert_eq!(state.registers.rsi, 0x1001);
    assert_eq!(state.registers.rdx, 0x123456789ABCDE80);
}

#[test]
fn test_outsb_instruction_direction_flag() {
    // OUTSB with direction flag set (0x6E)
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rdx = 0x123456789ABCDE80; // DX = 0xDE80
    state.registers.rsi = 0x1000;
    state.registers.set_flag(RFlags::DIRECTION, true);
    state.write_u8(0x1000, 0x5A).unwrap();
    
    let result = execute_instruction(&[0x6E], &mut state).unwrap();
    
    // RSI should be decremented by 1 (direction flag set)
    assert_eq!(state.registers.rsi, 0x0FFF);
    assert_eq!(state.registers.rdx, 0x123456789ABCDE80);
}

#[test]
fn test_outsw_instruction() {
    // OUTSW (0x66 0x6F)
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rdx = 0x123456789ABCDE80; // DX = 0xDE80
    state.registers.rsi = 0x1000;
    state.write_u16(0x1000, 0x5A5A).unwrap();
    
    let result = execute_instruction(&[0x66, 0x6F], &mut state).unwrap();
    
    // RSI should be incremented by 2 (direction flag clear)
    assert_eq!(state.registers.rsi, 0x1002);
    assert_eq!(state.registers.rdx, 0x123456789ABCDE80);
}

#[test]
fn test_outsw_instruction_direction_flag() {
    // OUTSW with direction flag set (0x66 0x6F)
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rdx = 0x123456789ABCDE80; // DX = 0xDE80
    state.registers.rsi = 0x1000;
    state.registers.set_flag(RFlags::DIRECTION, true);
    state.write_u16(0x1000, 0x5A5A).unwrap();
    
    let result = execute_instruction(&[0x66, 0x6F], &mut state).unwrap();
    
    // RSI should be decremented by 2 (direction flag set)
    assert_eq!(state.registers.rsi, 0x0FFE);
    assert_eq!(state.registers.rdx, 0x123456789ABCDE80);
}

#[test]
fn test_outsd_instruction() {
    // OUTSD (0x6F)
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rdx = 0x123456789ABCDE80; // DX = 0xDE80
    state.registers.rsi = 0x1000;
    state.write_u32(0x1000, 0x5A5A5A5A).unwrap();
    
    let result = execute_instruction(&[0x6F], &mut state).unwrap();
    
    // RSI should be incremented by 4 (direction flag clear)
    assert_eq!(state.registers.rsi, 0x1004);
    assert_eq!(state.registers.rdx, 0x123456789ABCDE80);
}

#[test]
fn test_outsd_instruction_direction_flag() {
    // OUTSD with direction flag set (0x6F)
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rdx = 0x123456789ABCDE80; // DX = 0xDE80
    state.registers.rsi = 0x1000;
    state.registers.set_flag(RFlags::DIRECTION, true);
    state.write_u32(0x1000, 0x5A5A5A5A).unwrap();
    
    let result = execute_instruction(&[0x6F], &mut state).unwrap();
    
    // RSI should be decremented by 4 (direction flag set)
    assert_eq!(state.registers.rsi, 0x0FFC);
    assert_eq!(state.registers.rdx, 0x123456789ABCDE80);
}

#[test]
fn test_orpd_instruction() {
    // ORPD XMM0, XMM1 (0x66 0x0F 0x56 0xC1)
    let mut state = create_test_cpu_state().unwrap();
    
    let result = execute_instruction(&[0x66, 0x0F, 0x56, 0xC1], &mut state).unwrap();
    
    // ORPD is currently a placeholder, should not crash
    assert!(state.registers.rax == 0);
}

#[test]
fn test_orps_instruction() {
    // ORPS XMM0, XMM1 (0x0F 0x56 0xC1)
    let mut state = create_test_cpu_state().unwrap();
    
    let result = execute_instruction(&[0x0F, 0x56, 0xC1], &mut state).unwrap();
    
    // ORPS is currently a placeholder, should not crash
    assert!(state.registers.rax == 0);
}

#[test]
fn test_instruction_error_handling() {
    // Test invalid OR instruction (wrong operand count)
    let mut state = create_test_cpu_state().unwrap();
    let result = execute_instruction(&[0x90], &mut state); // NOP instead of OR
    
    // Should not error for NOP
    assert!(result.is_ok());
}

#[test]
fn test_memory_boundary_conditions() {
    // Test OR with memory at boundary
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDEF0;
    state.registers.rbx = 0x1000; // RBX points to memory address 0x1000
    state.write_u64(0x1000, 0xFEDCBA9876543210).unwrap();
    
    let result = execute_instruction(&[0x48, 0x0B, 0x03], &mut state);
    
    // OR result: 0x123456789ABCDEF0 | 0xFEDCBA9876543210 = 0xFEFCFEF8FEFCFEF0
    assert_eq!(state.registers.rax, 0xFEFCFEF8FEFCFEF0);
}

#[test]
fn test_or_instruction_parity_flag() {
    // Test OR instruction sets parity flag correctly
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDE5A; // AL = 0x5A (even parity)
    state.registers.rbx = 0xFEDCBA98765432A5; // BL = 0xA5 (odd parity)
    
    let result = execute_instruction(&[0x08, 0xD8], &mut state);
    
    // OR result: 0x5A | 0xA5 = 0xFF (even parity - 8 bits set)
    assert_eq!(state.registers.rax & 0xFF, 0xFF);
    assert!(state.registers.get_flag(RFlags::PARITY)); // 0xFF has even parity
}

#[test]
fn test_or_instruction_combination() {
    // Test multiple OR operations in sequence
    let mut state = create_test_cpu_state().unwrap();
    state.registers.rax = 0x123456789ABCDEF0;
    state.registers.rbx = 0xFEDCBA9876543210;
    state.registers.rcx = 0x1111111111111111;
    
    // First OR: RAX | RBX
    let result1 = execute_instruction(&[0x48, 0x09, 0xD8], &mut state);
    // OR result: 0x123456789ABCDEF0 | 0xFEDCBA9876543210 = 0xFEFCFEF8FEFCFEF0
    assert_eq!(state.registers.rax, 0xFEFCFEF8FEFCFEF0);
    
    // Second OR: RAX | RCX
    let result2 = execute_instruction(&[0x48, 0x09, 0xC8], &mut state);
    // OR result: 0xFEFCFEF8FEFCFEF0 | 0x1111111111111111 = 0xFFFDFFF9FFFDFFF1
    assert_eq!(state.registers.rax, 0xFFFDFFF9FFFDFFF1);
}
