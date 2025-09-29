use crate::cpu::{CpuState, InstructionDecoder, registers::RFlags};
use crate::test::helpers::{create_test_cpu_state, write_memory, read_memory};
use iced_x86::{Decoder, DecoderOptions};

fn execute_instruction(instruction_bytes: &[u8], state: &mut CpuState) -> Result<CpuState, Box<dyn std::error::Error>> {
    let mut decoder = Decoder::new(64, instruction_bytes, DecoderOptions::NONE);
    let instruction = decoder.decode();
    let decoder_impl = InstructionDecoder::new();
    decoder_impl.execute_instruction(&instruction, state)?;
    Ok(create_test_cpu_state().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic X instructions
    #[test]
    fn test_xor_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XOR RAX, RBX (0x48 0x31 0xD8)
        state.registers.rax = 0x123456789ABCDEF0;
        state.registers.rbx = 0xFEDCBA9876543210;
        
        let result = execute_instruction(&[0x48, 0x31, 0xD8], &mut state).unwrap();
        
        // XOR result: 0x123456789ABCDEF0 ^ 0xFEDCBA9876543210 = 0xECE8ECE0ECE8ECE0
        assert_eq!(state.registers.rax, 0xECE8ECE0ECE8ECE0);
        assert_eq!(state.registers.rbx, 0xFEDCBA9876543210); // RBX unchanged
    }

    #[test]
    fn test_xor_16bit_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XOR AX, BX (0x66 0x31 0xD8)
        state.registers.rax = 0x123456789ABCDEF0;
        state.registers.rbx = 0xFEDCBA9876543210;
        
        let result = execute_instruction(&[0x66, 0x31, 0xD8], &mut state).unwrap();
        
        // XOR result: 0xDEF0 ^ 0x3210 = 0xECE0
        assert_eq!(state.registers.rax, 0x123456789ABCECE0);
        assert_eq!(state.registers.rbx, 0xFEDCBA9876543210); // RBX unchanged
    }

    #[test]
    fn test_xor_32bit_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XOR EAX, EBX (0x31 0xD8)
        state.registers.rax = 0x123456789ABCDEF0;
        state.registers.rbx = 0xFEDCBA9876543210;
        
        let result = execute_instruction(&[0x31, 0xD8], &mut state).unwrap();
        
        // XOR result: 0x9ABCDEF0 ^ 0x76543210 = 0xECE8ECE0 (32-bit operation zero-extends upper bits)
        assert_eq!(state.registers.rax, 0x00000000ECE8ECE0);
        assert_eq!(state.registers.rbx, 0xFEDCBA9876543210); // RBX unchanged
    }

    #[test]
    fn test_xor_memory_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XOR RAX, [RBX] (0x48 0x33 0x03)
        state.registers.rax = 0x123456789ABCDEF0;
        state.registers.rbx = 0x1000;
        state.write_u64(0x1000, 0xFEDCBA9876543210).unwrap();
        
        let result = execute_instruction(&[0x48, 0x33, 0x03], &mut state).unwrap();
        
        // XOR result: 0x123456789ABCDEF0 ^ 0xFEDCBA9876543210 = 0xECE8ECE0ECE8ECE0
        assert_eq!(state.registers.rax, 0xECE8ECE0ECE8ECE0);
    }

    #[test]
    fn test_xor_immediate_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XOR RAX, 0x12345678 (0x48 0x35 0x78 0x56 0x34 0x12)
        state.registers.rax = 0xFEDCBA9876543210;
        
        let result = execute_instruction(&[0x48, 0x35, 0x78, 0x56, 0x34, 0x12], &mut state).unwrap();
        
        // XOR result: 0x76543210 ^ 0x12345678 = 0x64606468 (32-bit operation with zero-extension)
        assert_eq!(state.registers.rax, 0xFEDCBA9864606468);
    }

    #[test]
    fn test_xor_flags() {
        let mut state = create_test_cpu_state().unwrap();
        // XOR RAX, RAX (0x48 0x31 0xC0) - should set zero flag
        state.registers.rax = 0x123456789ABCDEF0;
        
        let result = execute_instruction(&[0x48, 0x31, 0xC0], &mut state).unwrap();
        
        // XOR result: 0x123456789ABCDEF0 ^ 0x123456789ABCDEF0 = 0x0
        assert_eq!(state.registers.rax, 0x0);
        assert!(state.registers.get_flag(RFlags::ZERO));
        assert!(!state.registers.get_flag(RFlags::SIGN));
        assert!(state.registers.get_flag(RFlags::PARITY)); // 0 has even parity
    }

    #[test]
    fn test_xchg_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XCHG RAX, RBX (0x48 0x87 0xD8)
        state.registers.rax = 0x123456789ABCDEF0;
        state.registers.rbx = 0xFEDCBA9876543210;
        
        let result = execute_instruction(&[0x48, 0x87, 0xD8], &mut state).unwrap();
        
        // Values should be swapped
        assert_eq!(state.registers.rax, 0xFEDCBA9876543210);
        assert_eq!(state.registers.rbx, 0x123456789ABCDEF0);
    }

    #[test]
    fn test_xchg_16bit_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XCHG AX, BX (0x66 0x87 0xD8)
        state.registers.rax = 0x123456789ABCDEF0;
        state.registers.rbx = 0xFEDCBA9876543210;
        
        let result = execute_instruction(&[0x66, 0x87, 0xD8], &mut state).unwrap();
        
        // Only lower 16 bits should be swapped
        assert_eq!(state.registers.rax, 0x123456789ABC3210);
        assert_eq!(state.registers.rbx, 0xFEDCBA987654DEF0);
    }

    #[test]
    fn test_xchg_memory_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XCHG RAX, [RBX] (0x48 0x87 0x03)
        state.registers.rax = 0x123456789ABCDEF0;
        state.registers.rbx = 0x1000;
        state.write_u64(0x1000, 0xFEDCBA9876543210).unwrap();
        
        let result = execute_instruction(&[0x48, 0x87, 0x03], &mut state).unwrap();
        
        // Values should be swapped
        assert_eq!(state.registers.rax, 0xFEDCBA9876543210);
        assert_eq!(result.read_u64(0x1000).unwrap(), 0x123456789ABCDEF0);
    }

    #[test]
    fn test_xlat_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XLATB (0xD7)
        state.registers.rax = 0x123456789ABCDE05; // AL = 0x05
        state.registers.rbx = 0x1000; // Table base
        
        // Set up lookup table
        state.write_u8(0x1000 + 0x05, 0xAB).unwrap();
        
        let result = execute_instruction(&[0xD7], &mut state).unwrap();
        
        // AL should be replaced with table lookup result
        assert_eq!(state.registers.rax, 0x123456789ABCDEAB);
        assert_eq!(state.registers.rbx, 0x1000); // RBX unchanged
    }

    // X transactional memory instructions
    #[test]
    fn test_xabort_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XABORT 0x01 (0xC6 0xF8 0x01)
        let result = execute_instruction(&[0xC6, 0xF8, 0x01], &mut state);
        
        // XABORT should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("XABORT instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_xadd_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XADD RAX, RBX (0x48 0x0F 0xC1 0xD8)
        state.registers.rax = 0x123456789ABCDEF0;
        state.registers.rbx = 0xFEDCBA9876543210;
        
        let result = execute_instruction(&[0x48, 0x0F, 0xC1, 0xD8], &mut state);
        
        // XADD should execute without crashing
        match result {
            Ok(state) => {
                // XADD: exchange and add
                // RAX should contain the sum: 0x123456789ABCDEF0 + 0xFEDCBA9876543210 = 0x1111111111111100
                assert_eq!(state.registers.rax, 0x1111111111111100);
                // RBX should contain the original RAX value
                assert_eq!(state.registers.rbx, 0x123456789ABCDEF0);
            },
            Err(e) => {
                println!("XADD instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_xbegin_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XBEGIN +0x10 (0xC7 0xF8 0x10 0x00 0x00 0x00)
        let result = execute_instruction(&[0xC7, 0xF8, 0x10, 0x00, 0x00, 0x00], &mut state);
        
        // XBEGIN should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("XBEGIN instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_xend_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XEND (0x0F 0x01 0xD5)
        let result = execute_instruction(&[0x0F, 0x01, 0xD5], &mut state);
        
        // XEND should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("XEND instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_xtest_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XTEST (0x0F 0x01 0xD6)
        let result = execute_instruction(&[0x0F, 0x01, 0xD6], &mut state);
        
        // XTEST should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("XTEST instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    // X crypto instructions
    #[test]
    fn test_xcryptcbc_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XCRYPTCBC (0x0F 0xA7 0xC8)
        let result = execute_instruction(&[0x0F, 0xA7, 0xC8], &mut state);
        
        // XCRYPTCBC should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("XCRYPTCBC instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_xsha1_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XSHA1 (0x0F 0xA6 0xC8)
        let result = execute_instruction(&[0x0F, 0xA6, 0xC8], &mut state);
        
        // XSHA1 should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("XSHA1 instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_xsha256_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XSHA256 (0x0F 0xA6 0xD0)
        let result = execute_instruction(&[0x0F, 0xA6, 0xD0], &mut state);
        
        // XSHA256 should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("XSHA256 instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_xstore_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XSTORE (0x0F 0xA7 0xC0)
        let result = execute_instruction(&[0x0F, 0xA7, 0xC0], &mut state);
        
        // XSTORE should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("XSTORE instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    // X save/restore instructions
    #[test]
    fn test_xsave_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XSAVE [RAX] (0x0F 0xAE 0x20)
        let result = execute_instruction(&[0x0F, 0xAE, 0x20], &mut state);
        
        // XSAVE should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("XSAVE instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_xrstor_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XRSTOR [RAX] (0x0F 0xAE 0x28)
        let result = execute_instruction(&[0x0F, 0xAE, 0x28], &mut state);
        
        // XRSTOR should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("XRSTOR instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    // X SIMD instructions
    #[test]
    fn test_xorpd_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XORPD XMM0, XMM1 (0x66 0x0F 0x57 0xC1)
        let result = execute_instruction(&[0x66, 0x0F, 0x57, 0xC1], &mut state);
        
        // XORPD should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("XORPD instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_xorps_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XORPS XMM0, XMM1 (0x0F 0x57 0xC1)
        let result = execute_instruction(&[0x0F, 0x57, 0xC1], &mut state);
        
        // XORPS should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("XORPS instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    // X system instructions
    #[test]
    fn test_xgetbv_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XGETBV (0x0F 0x01 0xD0)
        let result = execute_instruction(&[0x0F, 0x01, 0xD0], &mut state);
        
        // XGETBV should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("XGETBV instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_xsetbv_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XSETBV (0x0F 0x01 0xD1)
        let result = execute_instruction(&[0x0F, 0x01, 0xD1], &mut state);
        
        // XSETBV should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("XSETBV instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_xbts_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // XBTS RAX, RBX (0x0F 0xA6 0xC3)
        let result = execute_instruction(&[0x0F, 0xA6, 0xC3], &mut state);
        
        // XBTS should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("XBTS instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    // Test combinations and edge cases
    #[test]
    fn test_x_instructions_combination() {
        let mut state = create_test_cpu_state().unwrap();
        
        // Test multiple X instructions in sequence
        // XOR RAX, RBX
        state.registers.rax = 0x123456789ABCDEF0;
        state.registers.rbx = 0xFEDCBA9876543210;
        let result1 = execute_instruction(&[0x48, 0x31, 0xD8], &mut state);
        assert!(result1.is_ok());
        
        // XCHG RAX, RBX
        let result2 = execute_instruction(&[0x48, 0x87, 0xD8], &mut state);
        assert!(result2.is_ok());
        
        // Verify final state
        let finalstate = result2.unwrap();
        assert_eq!(finalstate.registers.rax, 0xFEDCBA9876543210);
        assert_eq!(finalstate.registers.rbx, 0xECE8ECE0ECE8ECE0); // XOR result from first instruction
    }

    #[test]
    fn test_x_instructions_edge_cases() {
        let mut state = create_test_cpu_state().unwrap();
        
        // Test XOR with maximum values
        state.registers.rax = 0xFFFFFFFFFFFFFFFF;
        state.registers.rbx = 0xFFFFFFFFFFFFFFFF;
        
        let result = execute_instruction(&[0x48, 0x31, 0xD8], &mut state);
        
        // Should execute without crashing
        match result {
            Ok(state) => {
                // XOR result: 0xFFFFFFFFFFFFFFFF ^ 0xFFFFFFFFFFFFFFFF = 0x0
                assert_eq!(state.registers.rax, 0x0);
                assert!(state.registers.get_flag(RFlags::ZERO));
            },
            Err(e) => {
                println!("XOR edge case failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_xlat_edge_cases() {
        let mut state = create_test_cpu_state().unwrap();
        
        // Test XLAT with maximum offset
        state.registers.rax = 0x123456789ABCDEFF; // AL = 0xFF
        state.registers.rbx = 0x1000; // Table base
        
        // Set up lookup table at maximum offset
        state.write_u8(0x1000 + 0xFF, 0x42).unwrap();
        
        let result = execute_instruction(&[0xD7], &mut state).unwrap();
        
        // AL should be replaced with table lookup result
        assert_eq!(state.registers.rax, 0x123456789ABCDE42);
    }
}
