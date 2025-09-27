use crate::cpu::state::CpuState;
use crate::cpu::registers::RFlags;
use crate::memory::guest_memory::GuestMemory;
use crate::cpu::instruction::InstructionDecoder;
use iced_x86::{Decoder, DecoderOptions, Instruction};
use crate::Result;

/// Test helper to create a CPU state with initialized memory
fn create_test_cpu_state() -> Result<CpuState> {
    let memory = GuestMemory::new(1024 * 1024)?; // 1MB of memory
    Ok(CpuState::new(memory))
}

/// Test helper to decode a single instruction from bytes
fn decode_instruction(bytes: &[u8]) -> Instruction {
    let mut decoder = Decoder::new(64, bytes, DecoderOptions::NONE);
    decoder.decode()
}

/// Test helper to execute an instruction and return the result
fn execute_instruction(bytes: &[u8], mut state: CpuState) -> Result<CpuState> {
    let instruction = decode_instruction(bytes);
    let mut decoder = InstructionDecoder::new();
    decoder.execute_instruction(&instruction, &mut state)?;
    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aaa_instruction() {
        // AAA - ASCII Adjust After Addition
        // Adjusts AL after addition of two unpacked BCD values
        
        // Test case 1: Normal case - AL = 0x05, AF = 0
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x05; // AL = 0x05
        state.registers.set_flag(RFlags::AUXILIARY, false);
        state.registers.set_flag(RFlags::CARRY, false);
        
        let result = execute_instruction(&[0x37], state).unwrap(); // AAA
        
        // After AAA: AL should remain 0x05, AH should be 0, flags unchanged
        assert_eq!(result.registers.rax & 0xFF, 0x05); // AL
        assert_eq!((result.registers.rax >> 8) & 0xFF, 0x00); // AH
        
        // Test case 2: Adjustment needed - AL = 0x0A, AF = 0
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x0A; // AL = 0x0A
        state.registers.set_flag(RFlags::AUXILIARY, false);
        state.registers.set_flag(RFlags::CARRY, false);
        
        let result = execute_instruction(&[0x37], state).unwrap(); // AAA
        
        // After AAA: AL should be 0x00, AH should be 0x01, AF and CF should be set
        assert_eq!(result.registers.rax & 0xFF, 0x00); // AL
        assert_eq!((result.registers.rax >> 8) & 0xFF, 0x01); // AH
        assert!(result.registers.get_flag(RFlags::AUXILIARY));
        assert!(result.registers.get_flag(RFlags::CARRY));
        
        // Test case 3: High nibble adjustment - AL = 0x15, AF = 0
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x15; // AL = 0x15
        state.registers.set_flag(RFlags::AUXILIARY, false);
        state.registers.set_flag(RFlags::CARRY, false);
        
        let result = execute_instruction(&[0x37], state).unwrap(); // AAA
        
        // After AAA: AL should be 0x05, AH should be 0x01, AF and CF should be set
        assert_eq!(result.registers.rax & 0xFF, 0x05); // AL
        assert_eq!((result.registers.rax >> 8) & 0xFF, 0x01); // AH
        assert!(result.registers.get_flag(RFlags::AUXILIARY));
        assert!(result.registers.get_flag(RFlags::CARRY));
    }

    #[test]
    fn test_aad_instruction() {
        // AAD - ASCII Adjust AX Before Division
        // Converts unpacked BCD digits in AH and AL to binary in AL
        
        // Test case 1: Normal case - AH = 0x02, AL = 0x05 (represents 25)
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x0205; // AH = 0x02, AL = 0x05
        
        let result = execute_instruction(&[0xD5, 0x0A], state).unwrap(); // AAD (implicit 10)
        
        // After AAD: AL should be 0x19 (25 in decimal), AH should be 0x00
        assert_eq!(result.registers.rax & 0xFF, 0x19); // AL = 25
        assert_eq!((result.registers.rax >> 8) & 0xFF, 0x00); // AH = 0
        
        // Test case 2: Different values - AH = 0x01, AL = 0x07 (represents 17)
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x0107; // AH = 0x01, AL = 0x07
        
        let result = execute_instruction(&[0xD5, 0x0A], state).unwrap(); // AAD (implicit 10)
        
        // After AAD: AL should be 0x11 (17 in decimal), AH should be 0x00
        assert_eq!(result.registers.rax & 0xFF, 0x11); // AL = 17
        assert_eq!((result.registers.rax >> 8) & 0xFF, 0x00); // AH = 0
        
        // Test case 3: Zero case - AH = 0x00, AL = 0x00
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x0000; // AH = 0x00, AL = 0x00
        
        let result = execute_instruction(&[0xD5, 0x0A], state).unwrap(); // AAD (implicit 10)
        
        // After AAD: AL should be 0x00, AH should be 0x00, ZF should be set
        assert_eq!(result.registers.rax & 0xFF, 0x00); // AL = 0
        assert_eq!((result.registers.rax >> 8) & 0xFF, 0x00); // AH = 0
        assert!(result.registers.get_flag(RFlags::ZERO));
    }

    #[test]
    fn test_aam_instruction() {
        // AAM - ASCII Adjust AX After Multiply
        // Converts binary result in AL to unpacked BCD in AH and AL
        
        // Test case 1: Normal case - AL = 0x19 (25 in decimal)
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x19; // AL = 25, AH = 0
        
        let result = execute_instruction(&[0xD4, 0x0A], state).unwrap(); // AAM (implicit 10)
        
        // After AAM: AL should be 0x05, AH should be 0x02 (represents 25)
        assert_eq!(result.registers.rax & 0xFF, 0x05); // AL = 5
        assert_eq!((result.registers.rax >> 8) & 0xFF, 0x02); // AH = 2
        
        // Test case 2: Different value - AL = 0x11 (17 in decimal)
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x11; // AL = 17, AH = 0
        
        let result = execute_instruction(&[0xD4, 0x0A], state).unwrap(); // AAM (implicit 10)
        
        // After AAM: AL should be 0x07, AH should be 0x01 (represents 17)
        assert_eq!(result.registers.rax & 0xFF, 0x07); // AL = 7
        assert_eq!((result.registers.rax >> 8) & 0xFF, 0x01); // AH = 1
        
        // Test case 3: Zero case - AL = 0x00
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x00; // AL = 0, AH = 0
        
        let result = execute_instruction(&[0xD4, 0x0A], state).unwrap(); // AAM (implicit 10)
        
        // After AAM: AL should be 0x00, AH should be 0x00, ZF should be set
        assert_eq!(result.registers.rax & 0xFF, 0x00); // AL = 0
        assert_eq!((result.registers.rax >> 8) & 0xFF, 0x00); // AH = 0
        assert!(result.registers.get_flag(RFlags::ZERO));
    }

    #[test]
    fn test_aas_instruction() {
        // AAS - ASCII Adjust AL After Subtraction
        // Adjusts AL after subtraction of two unpacked BCD values
        
        // Test case 1: Normal case - AL = 0x05, AF = 0
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x05; // AL = 0x05
        state.registers.set_flag(RFlags::AUXILIARY, false);
        state.registers.set_flag(RFlags::CARRY, false);
        
        let result = execute_instruction(&[0x3F], state).unwrap(); // AAS
        
        // After AAS: AL should remain 0x05, AH should be 0, flags unchanged
        assert_eq!(result.registers.rax & 0xFF, 0x05); // AL
        assert_eq!((result.registers.rax >> 8) & 0xFF, 0x00); // AH
        
        // Test case 2: Adjustment needed - AL = 0x0A, AF = 1
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x0A; // AL = 0x0A
        state.registers.set_flag(RFlags::AUXILIARY, true);
        state.registers.set_flag(RFlags::CARRY, false);
        
        let result = execute_instruction(&[0x3F], state).unwrap(); // AAS
        
        // After AAS: AL should be 0x04, AH should be 0xFF, AF and CF should be set
        assert_eq!(result.registers.rax & 0xFF, 0x04); // AL
        assert_eq!((result.registers.rax >> 8) & 0xFF, 0xFF); // AH
        assert!(result.registers.get_flag(RFlags::AUXILIARY));
        assert!(result.registers.get_flag(RFlags::CARRY));
        
        // Test case 3: High nibble adjustment - AL = 0x15, AF = 1
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x15; // AL = 0x15
        state.registers.set_flag(RFlags::AUXILIARY, true);
        state.registers.set_flag(RFlags::CARRY, false);
        
        let result = execute_instruction(&[0x3F], state).unwrap(); // AAS
        
        // After AAS: AL should be 0x0F, AH should be 0xFF, AF and CF should be set
        assert_eq!(result.registers.rax & 0xFF, 0x0F); // AL
        assert_eq!((result.registers.rax >> 8) & 0xFF, 0xFF); // AH
        assert!(result.registers.get_flag(RFlags::AUXILIARY));
        assert!(result.registers.get_flag(RFlags::CARRY));
    }

    #[test]
    fn test_adc_instruction() {
        // ADC - Add with Carry
        // Adds source, destination, and carry flag
        
        // Test case 1: Simple addition without carry
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x10; // RAX = 16
        state.registers.rcx = 0x20; // RCX = 32
        state.registers.set_flag(RFlags::CARRY, false);
        
        let result = execute_instruction(&[0x11, 0xC8], state).unwrap(); // ADC RAX, RCX
        
        // Result should be 0x30 (48), no carry
        assert_eq!(result.registers.rax, 0x30);
        assert!(!result.registers.get_flag(RFlags::CARRY));
        assert!(!result.registers.get_flag(RFlags::OVERFLOW));
        
        // Test case 2: Addition with carry flag set
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x10; // RAX = 16
        state.registers.rcx = 0x20; // RCX = 32
        state.registers.set_flag(RFlags::CARRY, true);
        
        let result = execute_instruction(&[0x11, 0xC8], state).unwrap(); // ADC RAX, RCX
        
        // Result should be 0x31 (49), no carry
        assert_eq!(result.registers.rax, 0x31);
        assert!(!result.registers.get_flag(RFlags::CARRY));
        
        // Test case 3: Addition with overflow
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x7FFFFFFFFFFFFFFF; // Max positive 64-bit value
        state.registers.rcx = 0x1; // RCX = 1
        state.registers.set_flag(RFlags::CARRY, false);
        
        let result = execute_instruction(&[0x11, 0xC8], state).unwrap(); // ADC RAX, RCX
        
        // Result should overflow to negative, overflow flag set
        assert_eq!(result.registers.rax, 0x8000000000000000);
        assert!(result.registers.get_flag(RFlags::OVERFLOW));
        assert!(result.registers.get_flag(RFlags::SIGN));
        
        // Test case 4: Addition with immediate
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x10; // RAX = 16
        state.registers.set_flag(RFlags::CARRY, true);
        
        let result = execute_instruction(&[0x15, 0x20, 0x00, 0x00, 0x00], state).unwrap(); // ADC EAX, 0x20
        
        // Result should be 0x31 (16 + 32 + 1)
        assert_eq!(result.registers.rax & 0xFFFFFFFF, 0x31);
    }

    #[test]
    fn test_add_instruction() {
        // ADD - Add
        // Adds source to destination
        
        // Test case 1: Simple addition
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x10; // RAX = 16
        state.registers.rcx = 0x20; // RCX = 32
        
        let result = execute_instruction(&[0x01, 0xC8], state).unwrap(); // ADD RAX, RCX
        
        // Result should be 0x30 (48)
        assert_eq!(result.registers.rax, 0x30);
        assert!(!result.registers.get_flag(RFlags::CARRY));
        assert!(!result.registers.get_flag(RFlags::OVERFLOW));
        assert!(!result.registers.get_flag(RFlags::ZERO));
        assert!(!result.registers.get_flag(RFlags::SIGN));
        
        // Test case 2: Addition with zero result
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x10; // RAX = 16
        state.registers.rcx = 0xFFFFFFFFFFFFFFF0; // RCX = -16 (two's complement)
        
        let result = execute_instruction(&[0x01, 0xC8], state).unwrap(); // ADD RAX, RCX
        
        // Result should be 0x00, zero flag set
        assert_eq!(result.registers.rax, 0x00);
        assert!(result.registers.get_flag(RFlags::ZERO));
        assert!(!result.registers.get_flag(RFlags::SIGN));
        
        // Test case 3: Addition with overflow
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x7FFFFFFFFFFFFFFF; // Max positive 64-bit value
        state.registers.rcx = 0x1; // RCX = 1
        
        let result = execute_instruction(&[0x01, 0xC8], state).unwrap(); // ADD RAX, RCX
        
        // Result should overflow to negative, overflow flag set
        assert_eq!(result.registers.rax, 0x8000000000000000);
        assert!(result.registers.get_flag(RFlags::OVERFLOW));
        assert!(result.registers.get_flag(RFlags::SIGN));
        
        // Test case 4: Addition with immediate
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x10; // RAX = 16
        
        let result = execute_instruction(&[0x05, 0x20, 0x00, 0x00, 0x00], state).unwrap(); // ADD EAX, 0x20
        
        // Result should be 0x30 (16 + 32)
        assert_eq!(result.registers.rax & 0xFFFFFFFF, 0x30);
        
        // Test case 5: Addition with memory operand
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x10; // RAX = 16
        state.registers.rbx = 0x1000; // RBX = memory address
        state.write_u64(0x1000, 0x20).unwrap(); // Store 32 at memory address
        
        let result = execute_instruction(&[0x03, 0x03], state).unwrap(); // ADD RAX, [RBX]
        
        // Result should be 0x30 (16 + 32)
        assert_eq!(result.registers.rax, 0x30);
    }

    #[test]
    fn test_and_instruction() {
        // AND - Logical AND
        // Performs bitwise AND operation
        
        // Test case 1: Simple AND operation
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x0F; // RAX = 0x0F (00001111)
        state.registers.rcx = 0x33; // RCX = 0x33 (00110011)
        
        let result = execute_instruction(&[0x21, 0xC8], state).unwrap(); // AND RAX, RCX
        
        // Result should be 0x03 (00000011)
        assert_eq!(result.registers.rax, 0x03);
        assert!(!result.registers.get_flag(RFlags::CARRY));
        assert!(!result.registers.get_flag(RFlags::OVERFLOW));
        assert!(!result.registers.get_flag(RFlags::ZERO));
        assert!(!result.registers.get_flag(RFlags::SIGN));
        
        // Test case 2: AND with zero result
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x0F; // RAX = 0x0F (00001111)
        state.registers.rcx = 0xF0; // RCX = 0xF0 (11110000)
        
        let result = execute_instruction(&[0x21, 0xC8], state).unwrap(); // AND RAX, RCX
        
        // Result should be 0x00, zero flag set
        assert_eq!(result.registers.rax, 0x00);
        assert!(result.registers.get_flag(RFlags::ZERO));
        assert!(!result.registers.get_flag(RFlags::SIGN));
        
        // Test case 3: AND with immediate
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0xFF; // RAX = 0xFF (11111111)
        
        let result = execute_instruction(&[0x25, 0x0F, 0x00, 0x00, 0x00], state).unwrap(); // AND EAX, 0x0F
        
        // Result should be 0x0F (00001111)
        assert_eq!(result.registers.rax & 0xFFFFFFFF, 0x0F);
        
        // Test case 4: AND with memory operand
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x0F; // RAX = 0x0F
        state.registers.rbx = 0x1000; // RBX = memory address
        state.write_u64(0x1000, 0x33).unwrap(); // Store 0x33 at memory address
        
        let result = execute_instruction(&[0x23, 0x03], state).unwrap(); // AND RAX, [RBX]
        
        // Result should be 0x03
        assert_eq!(result.registers.rax, 0x03);
        
        // Test case 5: AND with sign bit set
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x8000000000000000; // RAX = negative value
        state.registers.rcx = 0x8000000000000000; // RCX = negative value
        
        let result = execute_instruction(&[0x21, 0xC8], state).unwrap(); // AND RAX, RCX
        
        // Result should have sign bit set
        assert_eq!(result.registers.rax, 0x8000000000000000);
        assert!(result.registers.get_flag(RFlags::SIGN));
        assert!(!result.registers.get_flag(RFlags::ZERO));
    }

    #[test]
    fn test_arpl_instruction() {
        // ARPL - Adjust RPL Field of Segment Selector
        // Adjusts the RPL (Requested Privilege Level) field of a segment selector
        
        // Test case 1: RPL adjustment needed
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x1003; // Segment selector with RPL = 3
        state.registers.rcx = 0x0001; // RPL = 1
        
        let result = execute_instruction(&[0x63, 0xC1], state).unwrap(); // ARPL AX, CX
        
        // RPL should be adjusted to 3 (higher of the two)
        assert_eq!(result.registers.rax & 0xFFFF, 0x1003);
        assert!(result.registers.get_flag(RFlags::ZERO)); // ZF = 0 (no change needed)
        
        // Test case 2: RPL adjustment not needed
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rax = 0x1001; // Segment selector with RPL = 1
        state.registers.rcx = 0x0003; // RPL = 3
        
        let result = execute_instruction(&[0x63, 0xC1], state).unwrap(); // ARPL AX, CX
        
        // RPL should be adjusted to 3
        assert_eq!(result.registers.rax & 0xFFFF, 0x1003);
        assert!(!result.registers.get_flag(RFlags::ZERO)); // ZF = 1 (change was made)
        
        // Test case 3: Memory operand
        let mut state = create_test_cpu_state().unwrap();
        state.registers.rbx = 0x1000; // Memory address
        state.registers.rcx = 0x0002; // RPL = 2
        state.write_u16(0x1000, 0x1001).unwrap(); // Store segment selector with RPL = 1
        
        let result = execute_instruction(&[0x63, 0x0B], state).unwrap(); // ARPL [RBX], CX
        
        // RPL should be adjusted to 2
        let updated_selector = result.read_u16(0x1000).unwrap();
        assert_eq!(updated_selector, 0x1002);
        assert!(!result.registers.get_flag(RFlags::ZERO)); // ZF = 1 (change was made)
    }

    // Note: The following instructions are more complex and would require
    // additional test infrastructure for proper testing:
    // - ADOX, ADCX (carry-less multiply instructions)
    // - AES* instructions (AES encryption/decryption)
    // - ANDN (and-not instruction)
    // - ANDNPD, ANDNPS, ANDPD, ANDPS (SIMD instructions)
    // - ADDSUBPD, ADDSUBPS (SIMD instructions)
    // - ADDPD, ADDPS, ADDSD, ADDSS (SIMD instructions)

    #[test]
    fn test_instruction_basic_flags() {
        // Test that basic flag operations work correctly
        let mut state = create_test_cpu_state().unwrap();
        
        // Test zero flag
        state.registers.set_flag(RFlags::ZERO, true);
        assert!(state.registers.get_flag(RFlags::ZERO));
        
        state.registers.set_flag(RFlags::ZERO, false);
        assert!(!state.registers.get_flag(RFlags::ZERO));
        
        // Test carry flag
        state.registers.set_flag(RFlags::CARRY, true);
        assert!(state.registers.get_flag(RFlags::CARRY));
        
        // Test sign flag
        state.registers.set_flag(RFlags::SIGN, true);
        assert!(state.registers.get_flag(RFlags::SIGN));
        
        // Test overflow flag
        state.registers.set_flag(RFlags::OVERFLOW, true);
        assert!(state.registers.get_flag(RFlags::OVERFLOW));
        
        // Test auxiliary flag
        state.registers.set_flag(RFlags::AUXILIARY, true);
        assert!(state.registers.get_flag(RFlags::AUXILIARY));
    }

    #[test]
    fn test_memory_operations() {
        // Test basic memory read/write operations
        let mut state = create_test_cpu_state().unwrap();
        
        // Test 8-bit operations
        state.write_u8(0x1000, 0xAB).unwrap();
        let value = state.read_u8(0x1000).unwrap();
        assert_eq!(value, 0xAB);
        
        // Test 16-bit operations
        state.write_u16(0x1000, 0x1234).unwrap();
        let value = state.read_u16(0x1000).unwrap();
        assert_eq!(value, 0x1234);
        
        // Test 32-bit operations
        state.write_u32(0x1000, 0x12345678).unwrap();
        let value = state.read_u32(0x1000).unwrap();
        assert_eq!(value, 0x12345678);
        
        // Test 64-bit operations
        state.write_u64(0x1000, 0x123456789ABCDEF0).unwrap();
        let value = state.read_u64(0x1000).unwrap();
        assert_eq!(value, 0x123456789ABCDEF0);
    }

    #[test]
    fn test_register_operations() {
        // Test basic register operations
        let mut state = create_test_cpu_state().unwrap();
        
        // Test general purpose registers
        state.registers.rax = 0x123456789ABCDEF0;
        assert_eq!(state.registers.rax, 0x123456789ABCDEF0);
        
        state.registers.rbx = 0xFEDCBA9876543210;
        assert_eq!(state.registers.rbx, 0xFEDCBA9876543210);
        
        // Test register access by index
        state.registers.set_gp_register(0, 0x1111111111111111);
        assert_eq!(state.registers.get_gp_register(0), 0x1111111111111111);
        
        state.registers.set_gp_register(1, 0x2222222222222222);
        assert_eq!(state.registers.get_gp_register(1), 0x2222222222222222);
    }
}
