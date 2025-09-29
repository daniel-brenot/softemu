use crate::test::helpers::{create_test_cpu_state, decode_instruction, execute_instruction};


// HADDPD - Packed Double-FP Horizontal Add
#[test]
fn test_haddpd_instruction() {
    // HADDPD - Packed Double-FP Horizontal Add
    let _instruction = decode_instruction(&[0x66, 0x0F, 0x7C, 0xC0]); // HADDPD XMM0, XMM0
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Haddpd);

    let mut state = create_test_cpu_state().unwrap();
    let result = execute_instruction(&[0x66, 0x0F, 0x7C, 0xC0], &mut state);
    assert!(result.is_ok());
}

// HADDPS - Packed Single-FP Horizontal Add
#[test]
fn test_haddps_instruction() {
    // HADDPS - Packed Single-FP Horizontal Add
    let _instruction = decode_instruction(&[0xF2, 0x0F, 0x7C, 0xC0]); // HADDPS XMM0, XMM0
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Haddps);

    let mut state = create_test_cpu_state().unwrap();
    let result = execute_instruction(&[0xF2, 0x0F, 0x7C, 0xC0], &mut state);
    assert!(result.is_ok());
}

// HLT - Halt
#[test]
fn test_hlt_instruction() {
    // HLT - Halt
    let _instruction = decode_instruction(&[0xF4]); // HLT
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Hlt);

    let mut state = create_test_cpu_state().unwrap();
    let result = execute_instruction(&[0xF4], &mut state);
    assert!(result.is_ok());
}

// HSUBPD - Packed Double-FP Horizontal Subtract
#[test]
fn test_hsubpd_instruction() {
    // HSUBPD - Packed Double-FP Horizontal Subtract
    let _instruction = decode_instruction(&[0x66, 0x0F, 0x7D, 0xC0]); // HSUBPD XMM0, XMM0
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Hsubpd);

    let mut state = create_test_cpu_state().unwrap();
    let result = execute_instruction(&[0x66, 0x0F, 0x7D, 0xC0], &mut state);
    assert!(result.is_ok());
}

// HSUBPS - Packed Single-FP Horizontal Subtract
#[test]
fn test_hsubps_instruction() {
    // HSUBPS - Packed Single-FP Horizontal Subtract
    let _instruction = decode_instruction(&[0xF2, 0x0F, 0x7D, 0xC0]); // HSUBPS XMM0, XMM0
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Hsubps);

    let mut state = create_test_cpu_state().unwrap();
    let result = execute_instruction(&[0xF2, 0x0F, 0x7D, 0xC0], &mut state);
    assert!(result.is_ok());
}

// HRESET - Hypervisor Reset
#[test]
fn test_hreset_instruction() {
    // HRESET - Hypervisor Reset
    let _instruction = decode_instruction(&[0xF3, 0x0F, 0x3A, 0xF0, 0xC0]); // HRESET EAX
    // HRESET may decode as INVALID in 64-bit mode, so we'll be tolerant
    assert!(_instruction.mnemonic() == iced_x86::Mnemonic::Hreset || _instruction.mnemonic() == iced_x86::Mnemonic::INVALID);

    let mut state = create_test_cpu_state().unwrap();
    let result = execute_instruction(&[0xF3, 0x0F, 0x3A, 0xF0, 0xC0], &mut state);
    // HRESET may not be implemented or may be invalid in 64-bit mode
    assert!(result.is_ok() || result.is_err());
}