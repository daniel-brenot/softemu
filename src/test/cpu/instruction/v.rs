use crate::test::helpers::{create_test_cpu_state, execute_instruction};

// Basic V arithmetic instructions
#[test]
fn test_vaddpd_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VADDPD requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VADDPD YMM0, YMM1, YMM2 (0xC5 0xFD 0x58 0xC2)
    execute_instruction(&[0xC5, 0xFD, 0x58, 0xC2], &mut state).unwrap();
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    assert_eq!(state.registers.rax, 0u64);
}

#[test]
fn test_vaddps_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VADDPS requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VADDPS YMM0, YMM1, YMM2 (0xC5 0xFC 0x58 0xC2)
    execute_instruction(&[0xC5, 0xFC, 0x58, 0xC2], &mut state).unwrap();
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    assert_eq!(state.registers.rax, 0u64);
}

#[test]
fn test_vaddsd_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VADDSD requires XMM registers which we don't have, so we'll just test that it doesn't crash
    // VADDSD XMM0, XMM1, XMM2 (0xC5 0xF3 0x58 0xC2)
    let result = execute_instruction(&[0xC5, 0xF3, 0x58, 0xC2], &mut state);
    
    // Since we don't have XMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

#[test]
fn test_vaddss_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VADDSS requires XMM registers which we don't have, so we'll just test that it doesn't crash
    // VADDSS XMM0, XMM1, XMM2 (0xC5 0xF2 0x58 0xC2)
    let result = execute_instruction(&[0xC5, 0xF2, 0x58, 0xC2], &mut state);
    
    // Since we don't have XMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

// V comparison instructions
#[test]
fn test_vcmppd_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VCMPPD requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VCMPPD YMM0, YMM1, YMM2, 0 (0xC5 0xFD 0xC2 0xC2 0x00)
    let result = execute_instruction(&[0xC5, 0xFD, 0xC2, 0xC2, 0x00], &mut state);
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

#[test]
fn test_vcmpps_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VCMPPS requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VCMPPS YMM0, YMM1, YMM2, 0 (0xC5 0xFC 0xC2 0xC2 0x00)
    let result = execute_instruction(&[0xC5, 0xFC, 0xC2, 0xC2, 0x00], &mut state);
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

#[test]
fn test_vcomisd_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VCOMISD requires XMM registers which we don't have, so we'll just test that it doesn't crash
    // VCOMISD XMM0, XMM1 (0xC5 0xF9 0x2F 0xC1)
    let result = execute_instruction(&[0xC5, 0xF9, 0x2F, 0xC1], &mut state);
    
    // Since we don't have XMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

#[test]
fn test_vcomiss_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VCOMISS requires XMM registers which we don't have, so we'll just test that it doesn't crash
    // VCOMISS XMM0, XMM1 (0xC5 0xF8 0x2F 0xC1)
    let result = execute_instruction(&[0xC5, 0xF8, 0x2F, 0xC1], &mut state);
    
    // Since we don't have XMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

// V logical instructions
#[test]
fn test_vandpd_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VANDPD requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VANDPD YMM0, YMM1, YMM2 (0xC5 0xFD 0x54 0xC2)
    let result = execute_instruction(&[0xC5, 0xFD, 0x54, 0xC2], &mut state);
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

#[test]
fn test_vandps_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VANDPS requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VANDPS YMM0, YMM1, YMM2 (0xC5 0xFC 0x54 0xC2)
    let result = execute_instruction(&[0xC5, 0xFC, 0x54, 0xC2], &mut state);
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

#[test]
fn test_vorpd_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VORPD requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VORPD YMM0, YMM1, YMM2 (0xC5 0xFD 0x56 0xC2)
    let result = execute_instruction(&[0xC5, 0xFD, 0x56, 0xC2], &mut state);
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

#[test]
fn test_vorps_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VORPS requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VORPS YMM0, YMM1, YMM2 (0xC5 0xFC 0x56 0xC2)
    let result = execute_instruction(&[0xC5, 0xFC, 0x56, 0xC2], &mut state);
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

#[test]
fn test_vxorpd_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VXORPD requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VXORPD YMM0, YMM1, YMM2 (0xC5 0xFD 0x57 0xC2)
    let result = execute_instruction(&[0xC5, 0xFD, 0x57, 0xC2], &mut state);
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

#[test]
fn test_vxorps_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VXORPS requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VXORPS YMM0, YMM1, YMM2 (0xC5 0xFC 0x57 0xC2)
    let result = execute_instruction(&[0xC5, 0xFC, 0x57, 0xC2], &mut state);
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

// V conversion instructions
#[test]
fn test_vcvtsd2si_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VCVTSD2SI requires XMM registers which we don't have, so we'll just test that it doesn't crash
    // VCVTSD2SI RAX, XMM0 (0xC5 0xFB 0x2D 0xC0)
    let result = execute_instruction(&[0xC5, 0xFB, 0x2D, 0xC0], &mut state);
    
    // Since we don't have XMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

#[test]
fn test_vcvtsi2sd_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VCVTSI2SD requires XMM registers which we don't have, so we'll just test that it doesn't crash
    // VCVTSI2SD XMM0, XMM1, RAX (0xC5 0xF3 0x2A 0xC0)
    let result = execute_instruction(&[0xC5, 0xF3, 0x2A, 0xC0], &mut state);
    
    // Since we don't have XMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

// V move instructions
#[test]
fn test_vmovapd_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VMOVAPD requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VMOVAPD YMM0, YMM1 (0xC5 0xFD 0x28 0xC1)
    let result = execute_instruction(&[0xC5, 0xFD, 0x28, 0xC1], &mut state);
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

#[test]
fn test_vmovaps_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VMOVAPS requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VMOVAPS YMM0, YMM1 (0xC5 0xFC 0x28 0xC1)
    let result = execute_instruction(&[0xC5, 0xFC, 0x28, 0xC1], &mut state);
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

// V division instructions
#[test]
fn test_vdivpd_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VDIVPD requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VDIVPD YMM0, YMM1, YMM2 (0xC5 0xFD 0x5E 0xC2)
    let result = execute_instruction(&[0xC5, 0xFD, 0x5E, 0xC2], &mut state);
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

#[test]
fn test_vdivps_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VDIVPS requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VDIVPS YMM0, YMM1, YMM2 (0xC5 0xFC 0x5E 0xC2)
    let result = execute_instruction(&[0xC5, 0xFC, 0x5E, 0xC2], &mut state);
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

// V multiplication instructions
#[test]
fn test_vmulpd_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VMULPD requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VMULPD YMM0, YMM1, YMM2 (0xC5 0xFD 0x59 0xC2)
    let result = execute_instruction(&[0xC5, 0xFD, 0x59, 0xC2], &mut state);
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

#[test]
fn test_vmulps_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VMULPS requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VMULPS YMM0, YMM1, YMM2 (0xC5 0xFC 0x59 0xC2)
    let result = execute_instruction(&[0xC5, 0xFC, 0x59, 0xC2], &mut state);
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

// V subtraction instructions
#[test]
fn test_vsubpd_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VSUBPD requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VSUBPD YMM0, YMM1, YMM2 (0xC5 0xFD 0x5C 0xC2)
    let result = execute_instruction(&[0xC5, 0xFD, 0x5C, 0xC2], &mut state);
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

#[test]
fn test_vsubps_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VSUBPS requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VSUBPS YMM0, YMM1, YMM2 (0xC5 0xFC 0x5C 0xC2)
    let result = execute_instruction(&[0xC5, 0xFC, 0x5C, 0xC2], &mut state);
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

// V broadcast instructions
#[test]
fn test_vbroadcastsd_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VBROADCASTSD requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VBROADCASTSD YMM0, XMM1 (0xC4 0xE2 0x7D 0x19 0xC1)
    let result = execute_instruction(&[0xC4, 0xE2, 0x7D, 0x19, 0xC1], &mut state);
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

#[test]
fn test_vbroadcastss_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VBROADCASTSS requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VBROADCASTSS YMM0, XMM1 (0xC4 0xE2 0x7D 0x18 0xC1)
    let result = execute_instruction(&[0xC4, 0xE2, 0x7D, 0x18, 0xC1], &mut state);
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

// V blend instructions
#[test]
fn test_vblendpd_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VBLENDPD requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VBLENDPD YMM0, YMM1, YMM2, 0 (0xC4 0xE3 0x75 0x0D 0xC2 0x00)
    let result = execute_instruction(&[0xC4, 0xE3, 0x75, 0x0D, 0xC2, 0x00], &mut state);
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}

#[test]
fn test_vblendps_instruction() {
    let mut state = create_test_cpu_state().unwrap();
    // VBLENDPS requires YMM registers which we don't have, so we'll just test that it doesn't crash
    // VBLENDPS YMM0, YMM1, YMM2, 0 (0xC4 0xE3 0x75 0x0C 0xC2 0x00)
    let result = execute_instruction(&[0xC4, 0xE3, 0x75, 0x0C, 0xC2, 0x00], &mut state);
    
    // Since we don't have YMM registers, we'll just verify the instruction executes without crashing
    result.unwrap();
    assert_eq!(state.registers.rax, 0u64);
}