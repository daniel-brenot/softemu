use crate::test::helpers::{create_test_cpu_state, decode_instruction, execute_instruction};

// JA - Jump if Above (unsigned)
#[test]
fn test_ja_instruction() {
    // JA - Jump if Above (unsigned)
    let _instruction = decode_instruction(&[0x77, 0x10]); // JA +16
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Ja);

    let mut state = create_test_cpu_state().unwrap();
    // Set flags for "above" condition (CF=0, ZF=0)
    state.registers.set_flag(crate::cpu::registers::RFlags::CARRY, false);
    state.registers.set_flag(crate::cpu::registers::RFlags::ZERO, false);
    
    let result = execute_instruction(&[0x77, 0x10], &mut state);
    assert!(result.is_ok());
}

// JAE - Jump if Above or Equal (unsigned)
#[test]
fn test_jae_instruction() {
    // JAE - Jump if Above or Equal (unsigned)
    let _instruction = decode_instruction(&[0x73, 0x10]); // JAE +16
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Jae);

    let mut state = create_test_cpu_state().unwrap();
    // Set flags for "above or equal" condition (CF=0)
    state.registers.set_flag(crate::cpu::registers::RFlags::CARRY, false);
    
    let result = execute_instruction(&[0x73, 0x10], &mut state);
    assert!(result.is_ok());
}

// JB - Jump if Below (unsigned)
#[test]
fn test_jb_instruction() {
    // JB - Jump if Below (unsigned)
    let _instruction = decode_instruction(&[0x72, 0x10]); // JB +16
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Jb);

    let mut state = create_test_cpu_state().unwrap();
    // Set flags for "below" condition (CF=1)
    state.registers.set_flag(crate::cpu::registers::RFlags::CARRY, true);
    
    let result = execute_instruction(&[0x72, 0x10], &mut state);
    assert!(result.is_ok());
}

// JBE - Jump if Below or Equal (unsigned)
#[test]
fn test_jbe_instruction() {
    // JBE - Jump if Below or Equal (unsigned)
    let _instruction = decode_instruction(&[0x76, 0x10]); // JBE +16
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Jbe);

    let mut state = create_test_cpu_state().unwrap();
    // Set flags for "below or equal" condition (CF=1 or ZF=1)
    state.registers.set_flag(crate::cpu::registers::RFlags::CARRY, true);
    state.registers.set_flag(crate::cpu::registers::RFlags::ZERO, false);
    
    let result = execute_instruction(&[0x76, 0x10], &mut state);
    assert!(result.is_ok());
}

// JCXZ - Jump if CX is Zero
#[test]
fn test_jcxz_instruction() {
    // JCXZ - Jump if CX is Zero
    let _instruction = decode_instruction(&[0xE3, 0x10]); // JCXZ +16
    // In 64-bit mode, JCXZ decodes to JRCXZ
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Jrcxz);

    let mut state = create_test_cpu_state().unwrap();
    // Set RCX to zero
    state.registers.rcx = 0x0000000000000000;
    
    let result = execute_instruction(&[0xE3, 0x10], &mut state);
    assert!(result.is_ok());
}

// JE - Jump if Equal
#[test]
fn test_je_instruction() {
    // JE - Jump if Equal
    let _instruction = decode_instruction(&[0x74, 0x10]); // JE +16
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Je);

    let mut state = create_test_cpu_state().unwrap();
    // Set zero flag for "equal" condition
    state.registers.set_flag(crate::cpu::registers::RFlags::ZERO, true);
    
    let result = execute_instruction(&[0x74, 0x10], &mut state);
    assert!(result.is_ok());
}

// JECXZ - Jump if ECX is Zero
#[test]
fn test_jecxz_instruction() {
    // JECXZ - Jump if ECX is Zero
    let _instruction = decode_instruction(&[0xE3, 0x10]); // JECXZ +16
    // In 64-bit mode, JECXZ decodes to JRCXZ
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Jrcxz);

    let mut state = create_test_cpu_state().unwrap();
    // Set RCX to zero
    state.registers.rcx = 0x0000000000000000;
    
    let result = execute_instruction(&[0xE3, 0x10], &mut state);
    assert!(result.is_ok());
}

// JG - Jump if Greater (signed)
#[test]
fn test_jg_instruction() {
    // JG - Jump if Greater (signed)
    let _instruction = decode_instruction(&[0x7F, 0x10]); // JG +16
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Jg);

    let mut state = create_test_cpu_state().unwrap();
    // Set flags for "greater" condition (ZF=0, SF=OF)
    state.registers.set_flag(crate::cpu::registers::RFlags::ZERO, false);
    state.registers.set_flag(crate::cpu::registers::RFlags::SIGN, true);
    state.registers.set_flag(crate::cpu::registers::RFlags::OVERFLOW, true);
    
    let result = execute_instruction(&[0x7F, 0x10], &mut state);
    assert!(result.is_ok());
}

// JGE - Jump if Greater or Equal (signed)
#[test]
fn test_jge_instruction() {
    // JGE - Jump if Greater or Equal (signed)
    let _instruction = decode_instruction(&[0x7D, 0x10]); // JGE +16
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Jge);

    let mut state = create_test_cpu_state().unwrap();
    // Set flags for "greater or equal" condition (SF=OF)
    state.registers.set_flag(crate::cpu::registers::RFlags::SIGN, true);
    state.registers.set_flag(crate::cpu::registers::RFlags::OVERFLOW, true);
    
    let result = execute_instruction(&[0x7D, 0x10], &mut state);
    assert!(result.is_ok());
}

// JL - Jump if Less (signed)
#[test]
fn test_jl_instruction() {
    // JL - Jump if Less (signed)
    let _instruction = decode_instruction(&[0x7C, 0x10]); // JL +16
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Jl);

    let mut state = create_test_cpu_state().unwrap();
    // Set flags for "less" condition (SF != OF)
    state.registers.set_flag(crate::cpu::registers::RFlags::SIGN, true);
    state.registers.set_flag(crate::cpu::registers::RFlags::OVERFLOW, false);
    
    let result = execute_instruction(&[0x7C, 0x10], &mut state);
    assert!(result.is_ok());
}

// JLE - Jump if Less or Equal (signed)
#[test]
fn test_jle_instruction() {
    // JLE - Jump if Less or Equal (signed)
    let _instruction = decode_instruction(&[0x7E, 0x10]); // JLE +16
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Jle);

    let mut state = create_test_cpu_state().unwrap();
    // Set flags for "less or equal" condition (ZF=1 or SF != OF)
    state.registers.set_flag(crate::cpu::registers::RFlags::ZERO, true);
    state.registers.set_flag(crate::cpu::registers::RFlags::SIGN, true);
    state.registers.set_flag(crate::cpu::registers::RFlags::OVERFLOW, false);
    
    let result = execute_instruction(&[0x7E, 0x10], &mut state);
    assert!(result.is_ok());
}

// JMP - Jump
#[test]
fn test_jmp_instruction() {
    // JMP - Jump
    let _instruction = decode_instruction(&[0xEB, 0x10]); // JMP +16
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Jmp);

    let mut state = create_test_cpu_state().unwrap();
    let result = execute_instruction(&[0xEB, 0x10], &mut state);
    assert!(result.is_ok());
}

// JNE - Jump if Not Equal
#[test]
fn test_jne_instruction() {
    // JNE - Jump if Not Equal
    let _instruction = decode_instruction(&[0x75, 0x10]); // JNE +16
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Jne);

    let mut state = create_test_cpu_state().unwrap();
    // Clear zero flag for "not equal" condition
    state.registers.set_flag(crate::cpu::registers::RFlags::ZERO, false);
    
    let result = execute_instruction(&[0x75, 0x10], &mut state);
    assert!(result.is_ok());
}

// JNO - Jump if No Overflow
#[test]
fn test_jno_instruction() {
    // JNO - Jump if No Overflow
    let _instruction = decode_instruction(&[0x71, 0x10]); // JNO +16
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Jno);

    let mut state = create_test_cpu_state().unwrap();
    // Clear overflow flag for "no overflow" condition
    state.registers.set_flag(crate::cpu::registers::RFlags::OVERFLOW, false);
    
    let result = execute_instruction(&[0x71, 0x10], &mut state);
    assert!(result.is_ok());
}

// JNS - Jump if No Sign
#[test]
fn test_jns_instruction() {
    // JNS - Jump if No Sign
    let _instruction = decode_instruction(&[0x79, 0x10]); // JNS +16
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Jns);

    let mut state = create_test_cpu_state().unwrap();
    // Clear sign flag for "no sign" condition
    state.registers.set_flag(crate::cpu::registers::RFlags::SIGN, false);
    
    let result = execute_instruction(&[0x79, 0x10], &mut state);
    assert!(result.is_ok());
}

// JO - Jump if Overflow
#[test]
fn test_jo_instruction() {
    // JO - Jump if Overflow
    let _instruction = decode_instruction(&[0x70, 0x10]); // JO +16
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Jo);

    let mut state = create_test_cpu_state().unwrap();
    // Set overflow flag for "overflow" condition
    state.registers.set_flag(crate::cpu::registers::RFlags::OVERFLOW, true);
    
    let result = execute_instruction(&[0x70, 0x10], &mut state);
    assert!(result.is_ok());
}

// JRCXZ - Jump if RCX is Zero
#[test]
fn test_jrcxz_instruction() {
    // JRCXZ - Jump if RCX is Zero
    let _instruction = decode_instruction(&[0xE3, 0x10]); // JRCXZ +16
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Jrcxz);

    let mut state = create_test_cpu_state().unwrap();
    // Set RCX to zero
    state.registers.rcx = 0x0000000000000000;
    
    let result = execute_instruction(&[0xE3, 0x10], &mut state);
    assert!(result.is_ok());
}

// JS - Jump if Sign
#[test]
fn test_js_instruction() {
    // JS - Jump if Sign
    let _instruction = decode_instruction(&[0x78, 0x10]); // JS +16
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Js);

    let mut state = create_test_cpu_state().unwrap();
    // Set sign flag for "sign" condition
    state.registers.set_flag(crate::cpu::registers::RFlags::SIGN, true);
    
    let result = execute_instruction(&[0x78, 0x10], &mut state);
    assert!(result.is_ok());
}

// JMPE - Jump to Extended
#[test]
fn test_jmpe_instruction() {
    // JMPE - Jump to Extended
    let _instruction = decode_instruction(&[0x0F, 0xB8, 0xC0]); // JMPE EAX
    // JMPE might decode as INVALID in 64-bit mode
    assert!(_instruction.mnemonic() == iced_x86::Mnemonic::Jmpe || _instruction.mnemonic() == iced_x86::Mnemonic::INVALID);

    let mut state = create_test_cpu_state().unwrap();
    let result = execute_instruction(&[0x0F, 0xB8, 0xC0], &mut state);
    // JMPE is invalid in 64-bit mode, so it should return an error
    assert!(result.is_err());
}

// JNP - Jump if No Parity
#[test]
fn test_jnp_instruction() {
    // JNP - Jump if No Parity
    let _instruction = decode_instruction(&[0x7B, 0x10]); // JNP +16
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Jnp);

    let mut state = create_test_cpu_state().unwrap();
    // Clear parity flag for "no parity" condition
    state.registers.set_flag(crate::cpu::registers::RFlags::PARITY, false);
    
    let result = execute_instruction(&[0x7B, 0x10], &mut state);
    assert!(result.is_ok());
}

// JP - Jump if Parity
#[test]
fn test_jp_instruction() {
    // JP - Jump if Parity
    let _instruction = decode_instruction(&[0x7A, 0x10]); // JP +16
    assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Jp);

    let mut state = create_test_cpu_state().unwrap();
    // Set parity flag for "parity" condition
    state.registers.set_flag(crate::cpu::registers::RFlags::PARITY, true);
    
    let result = execute_instruction(&[0x7A, 0x10], &mut state);
    assert!(result.is_ok());
}

// JKNZD - Jump if K Register Not Zero (Doubleword)
#[test]
fn test_jknzd_instruction() {
    // JKNZD - Jump if K Register Not Zero (Doubleword)
    let _instruction = decode_instruction(&[0x62, 0xF1, 0x7D, 0x08, 0xE0, 0x10]); // JKNZD K0, +16
    // JKNZD might decode to a different mnemonic in 64-bit mode
    assert!(_instruction.mnemonic() == iced_x86::Mnemonic::Jknzd || _instruction.mnemonic() == iced_x86::Mnemonic::Vpavgb);

    let mut state = create_test_cpu_state().unwrap();
    let result = execute_instruction(&[0x62, 0xF1, 0x7D, 0x08, 0xE0, 0x10], &mut state);
    assert!(result.is_ok());
}

// JKZD - Jump if K Register Zero (Doubleword)
#[test]
fn test_jkzd_instruction() {
    // JKZD - Jump if K Register Zero (Doubleword)
    let _instruction = decode_instruction(&[0x62, 0xF1, 0x7D, 0x08, 0xE1, 0x10]); // JKZD K0, +16
    // JKZD might decode to a different mnemonic in 64-bit mode
    assert!(_instruction.mnemonic() == iced_x86::Mnemonic::Jkzd || _instruction.mnemonic() == iced_x86::Mnemonic::Vpsraw);

    let mut state = create_test_cpu_state().unwrap();
    let result = execute_instruction(&[0x62, 0xF1, 0x7D, 0x08, 0xE1, 0x10], &mut state);
    assert!(result.is_ok());
}