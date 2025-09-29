use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};
use crate::Result;
use crate::test::helpers::{create_test_cpu_state, execute_instruction, read_memory, write_memory};
use iced_x86::{Decoder, DecoderOptions, Instruction};

#[test]
fn test_ret_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Set up stack with return address
    state.registers.rsp = 0x1000;
    state.write_u64(0x1000, 0x123456789ABCDEF0).unwrap();
    
    // Execute RET instruction
    let result = execute_instruction(&[0xC3], &mut state);
    
    // Check that RIP was set to the return address
    assert_eq!(state.registers.rip, 0x123456789ABCDEF0);
    // Check that RSP was incremented by 8
    assert_eq!(state.registers.rsp, 0x1008);
}

#[test]
fn test_ret_instruction_with_immediate() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Set up stack with return address
    state.registers.rsp = 0x1000;
    state.write_u64(0x1000, 0x123456789ABCDEF0).unwrap();
    
    // Execute RET with 16-bit immediate (0x1000)
    let result = execute_instruction(&[0xC2, 0x00, 0x10], &mut state);
    
    // Check that RIP was set to the return address
    assert_eq!(state.registers.rip, 0x123456789ABCDEF0);
    // Check that RSP was incremented by 8 + immediate (0x1000)
    assert_eq!(state.registers.rsp, 0x2008);
}

#[test]
fn test_rol_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Set up registers: ROL RAX, 4
    state.registers.rax = 0x123456789ABCDEF0;
    
    // Execute ROL instruction (ROL RAX, 4)
    let result = execute_instruction(&[0x48, 0xC1, 0xC0, 0x04], &mut state);
    
    // Check that RAX was rotated left by 4 bits
    // 0x123456789ABCDEF0 << 4 = 0x23456789ABCDEF01
    assert_eq!(state.registers.rax, 0x23456789ABCDEF01);
}

#[test]
fn test_ror_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Set up registers: ROR RAX, 4
    state.registers.rax = 0x123456789ABCDEF0;
    
    // Execute ROR instruction (ROR RAX, 4)
    let result = execute_instruction(&[0x48, 0xC1, 0xC8, 0x04], &mut state);
    
    // Check that RAX was rotated right by 4 bits
    // 0x123456789ABCDEF0 >> 4 = 0x0123456789ABCDEF
    assert_eq!(state.registers.rax, 0x0123456789ABCDEF);
}

#[test]
fn test_rcl_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Set up registers: RCL RAX, 1 (with carry flag set)
    state.registers.rax = 0x8000000000000000;
    state.registers.set_flag(RFlags::CARRY, true);
    
    // Execute RCL instruction (RCL RAX, 1)
    let result = execute_instruction(&[0x48, 0xD1, 0xD0], &mut state);
    
    // Check that RAX was rotated left through carry
    // 0x8000000000000000 << 1 with carry = 0x0000000000000001
    assert_eq!(state.registers.rax, 0x0000000000000001);
    // Check that carry flag was set (MSB was 1)
    assert!(state.registers.get_flag(RFlags::CARRY));
}

#[test]
fn test_rcr_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Set up registers: RCR RAX, 1 (with carry flag set)
    state.registers.rax = 0x0000000000000001;
    state.registers.set_flag(RFlags::CARRY, true);
    
    // Execute RCR instruction (RCR RAX, 1)
    let result = execute_instruction(&[0x48, 0xD1, 0xD8], &mut state);
    
    // Check that RAX was rotated right through carry
    // 0x0000000000000001 >> 1 with carry = 0x8000000000000000
    assert_eq!(state.registers.rax, 0x8000000000000000);
    // Check that carry flag was set (LSB was 1)
    assert!(state.registers.get_flag(RFlags::CARRY));
}

#[test]
fn test_rdmsr_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Set up ECX with MSR number (IA32_APIC_BASE = 0x1B)
    state.registers.rcx = 0x1B;
    
    // Execute RDMSR instruction
    let result = execute_instruction(&[0x0F, 0x32], &mut state);
    
    // Check that RAX contains the MSR value
    assert_eq!(state.registers.rax, 0xFEE00000);
    // Check that RDX is zero
    assert_eq!(state.registers.rdx, 0);
}

#[test]
fn test_rdmsr_unsupported_register() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Set up ECX with unsupported MSR number
    state.registers.rcx = 0x999;
    
    // Execute RDMSR instruction
    let result = execute_instruction(&[0x0F, 0x32], &mut state);
    
    // Check that RAX and RDX are zero for unsupported MSR
    assert_eq!(state.registers.rax, 0);
    assert_eq!(state.registers.rdx, 0);
}

#[test]
fn test_rdpmc_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Set up ECX with performance counter number
    state.registers.rcx = 0x123;
    
    // Execute RDPMC instruction
    let result = execute_instruction(&[0x0F, 0x33], &mut state);
    
    // Check that RAX contains the counter value (ECX value)
    assert_eq!(state.registers.rax, 0x123);
    // Check that RDX is zero
    assert_eq!(state.registers.rdx, 0);
}

#[test]
fn test_rdrand_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Set up initial register values for pseudo-random generation
    state.registers.rax = 0x123456789ABCDEF0;
    state.registers.rcx = 0xFEDCBA9876543210;
    state.registers.rdx = 0x1111111111111111;
    
    // Execute RDRAND instruction (RDRAND RAX)
    let result = execute_instruction(&[0x48, 0x0F, 0xC7, 0xF0], &mut state);
    
    // Check that RAX contains a pseudo-random value
    let expected_random = 0x123456789ABCDEF0 ^ 0xFEDCBA9876543210 ^ 0x1111111111111111;
    assert_eq!(state.registers.rax, expected_random);
    // Check that carry flag was set (indicating success)
    assert!(state.registers.get_flag(RFlags::CARRY));
}

#[test]
fn test_rdseed_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Set up initial register values for pseudo-random generation
    state.registers.rax = 0xAAAAAAAAAAAAAAAA;
    state.registers.rcx = 0x5555555555555555;
    state.registers.rdx = 0xFFFFFFFFFFFFFFFF;
    
    // Execute RDSEED instruction (RDSEED RAX)
    let result = execute_instruction(&[0x48, 0x0F, 0xC7, 0xF8], &mut state);
    
    // Check that RAX contains a pseudo-random value
    let expected_random = 0xAAAAAAAAAAAAAAAA ^ 0x5555555555555555 ^ 0xFFFFFFFFFFFFFFFF;
    assert_eq!(state.registers.rax, expected_random);
    // Check that carry flag was set (indicating success)
    assert!(state.registers.get_flag(RFlags::CARRY));
}

#[test]
fn test_rdtsc_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Execute RDTSC instruction
    let result = execute_instruction(&[0x0F, 0x31], &mut state);
    
    // Check that RAX and RDX contain the dummy timestamp values
    assert_eq!(state.registers.rax, 0x12345678);
    assert_eq!(state.registers.rdx, 0x9ABCDEF0);
}

#[test]
fn test_rdtscp_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Execute RDTSCP instruction
    let result = execute_instruction(&[0x0F, 0x01, 0xF9], &mut state);
    
    // Check that RAX and RDX contain the dummy timestamp values
    assert_eq!(state.registers.rax, 0x12345678);
    assert_eq!(state.registers.rdx, 0x9ABCDEF0);
    // Check that RCX contains the processor ID (0)
    assert_eq!(state.registers.rcx, 0);
}

#[test]
fn test_rsm_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Set initial RIP value
    state.registers.rip = 0x1000;
    
    // Execute RSM instruction
    let result = execute_instruction(&[0x0F, 0xAA], &mut state);
    
    // RSM should execute without error (it just logs)
    // No specific register changes expected, RIP should remain unchanged
    assert_eq!(state.registers.rip, 0x1000);
}

#[test]
fn test_rol_with_carry_flag() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Set up registers: ROL RAX, 1 (with MSB set to test carry)
    state.registers.rax = 0x8000000000000000;
    
    // Execute ROL instruction (ROL RAX, 1)
    let result = execute_instruction(&[0x48, 0xD1, 0xC0], &mut state);
    
    // Check that RAX was rotated left by 1 bit
    assert_eq!(state.registers.rax, 0x0000000000000001);
    // Check that carry flag was set (MSB was 1)
    assert!(state.registers.get_flag(RFlags::CARRY));
}

#[test]
fn test_ror_with_carry_flag() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Set up registers: ROR RAX, 1 (with LSB set to test carry)
    state.registers.rax = 0x0000000000000001;
    
    // Execute ROR instruction (ROR RAX, 1)
    let result = execute_instruction(&[0x48, 0xD1, 0xC8], &mut state);
    
    // Check that RAX was rotated right by 1 bit
    assert_eq!(state.registers.rax, 0x8000000000000000);
    // Check that carry flag was set (LSB was 1)
    assert!(state.registers.get_flag(RFlags::CARRY));
}

#[test]
fn test_rol_32_bit_register() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Set up registers: ROL EAX, 4
    state.registers.rax = 0x123456789ABCDEF0;
    
    // Execute ROL instruction (ROL EAX, 4)
    let result = execute_instruction(&[0xC1, 0xC0, 0x04], &mut state);
    
    // Check that EAX was rotated left by 4 bits (only lower 32 bits)
    // 0x9ABCDEF0 << 4 = 0xABCDEF00
    assert_eq!(state.registers.rax & 0xFFFFFFFF, 0xABCDEF00);
}

#[test]
fn test_ror_16_bit_register() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Set up registers: ROR AX, 4
    state.registers.rax = 0x123456789ABCDEF0;
    
    // Execute ROR instruction (ROR AX, 4)
    let result = execute_instruction(&[0x66, 0xC1, 0xC8, 0x04], &mut state);
    
    // Check that AX was rotated right by 4 bits (only lower 16 bits)
    // 0xDEF0 >> 4 = 0x0DEF
    assert_eq!(state.registers.rax & 0xFFFF, 0x0DEF);
}

#[test]
fn test_ret_instruction_stack_underflow() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Set up stack pointer at an address that would cause out-of-bounds access
    // when trying to read 8 bytes (0x1000000 - 8 = 0xFFFFF8, which is valid)
    // Let's use an address that's definitely out of bounds
    state.registers.rsp = 0x1000000; // 16MB, which is beyond our 1MB memory
    
    // This should fail due to invalid memory access
    let result = execute_instruction(&[0xC3], &mut state);
    assert!(result.is_err());
}

#[test]
fn test_rdrand_different_registers() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Set up initial register values
    state.registers.rax = 0x1111111111111111;
    state.registers.rcx = 0x2222222222222222;
    state.registers.rdx = 0x3333333333333333;
    
    // Execute RDRAND instruction (RDRAND RBX)
    let result = execute_instruction(&[0x48, 0x0F, 0xC7, 0xF3], &mut state);
    
    // Check that RBX contains a pseudo-random value
    let expected_random = 0x1111111111111111 ^ 0x2222222222222222 ^ 0x3333333333333333;
    assert_eq!(state.registers.rbx, expected_random);
    // Check that carry flag was set (indicating success)
    assert!(state.registers.get_flag(RFlags::CARRY));
}

#[test]
fn test_rdmsr_32_bit_register() {
    let mut state = create_test_cpu_state().unwrap();
    
    // Set up ECX with MSR number (only lower 32 bits matter)
    state.registers.rcx = 0x123456780000001B; // Upper bits should be ignored
    
    // Execute RDMSR instruction
    let result = execute_instruction(&[0x0F, 0x32], &mut state);
    
    // Check that RAX contains the MSR value (ECX was 0x1B)
    assert_eq!(state.registers.rax, 0xFEE00000);
    // Check that RDX is zero
    assert_eq!(state.registers.rdx, 0);
}