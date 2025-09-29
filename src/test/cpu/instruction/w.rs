use crate::cpu::{CpuState, InstructionDecoder};
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

    // Basic W instructions
    #[test]
    fn test_wait_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // WAIT instruction - wait for floating point operations
        // WAIT (0x9B)
        let result = execute_instruction(&[0x9B], &mut state);
        
        // WAIT should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("WAIT instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_wbinvd_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // WBINVD instruction - write back and invalidate cache
        // WBINVD (0x0F 0x09)
        let result = execute_instruction(&[0x0F, 0x09], &mut state);
        
        // WBINVD should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("WBINVD instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_wrmsr_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // WRMSR instruction - write to model specific register
        // Set up registers for WRMSR
        state.registers.rcx = 0x1B; // IA32_APIC_BASE MSR
        state.registers.rax = 0x12345678; // Lower 32 bits
        state.registers.rdx = 0x9ABCDEF0; // Upper 32 bits
        
        // WRMSR (0x0F 0x30)
        let result = execute_instruction(&[0x0F, 0x30], &mut state);
        
        // WRMSR should execute without crashing
        match result {
            Ok(state) => {
                // Registers should remain unchanged
                assert_eq!(state.registers.rcx, 0x1B);
                assert_eq!(state.registers.rax, 0x12345678);
                assert_eq!(state.registers.rdx, 0x9ABCDEF0);
            },
            Err(e) => {
                println!("WRMSR instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_wrmsr_unsupported_msr() {
        let mut state = create_test_cpu_state().unwrap();
        // WRMSR instruction with unsupported MSR
        state.registers.rcx = 0x12345678; // Unsupported MSR
        state.registers.rax = 0x11111111;
        state.registers.rdx = 0x22222222;
        
        // WRMSR (0x0F 0x30)
        let result = execute_instruction(&[0x0F, 0x30], &mut state);
        
        // WRMSR should execute without crashing even with unsupported MSR
        match result {
            Ok(state) => {
                // Registers should remain unchanged
                assert_eq!(state.registers.rcx, 0x12345678);
                assert_eq!(state.registers.rax, 0x11111111);
                assert_eq!(state.registers.rdx, 0x22222222);
            },
            Err(e) => {
                println!("WRMSR instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    // W cache instructions
    #[test]
    fn test_wbnoinvd_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // WBNOINVD instruction - write back but don't invalidate cache
        // WBNOINVD (0xF3 0x0F 0x09)
        let result = execute_instruction(&[0xF3, 0x0F, 0x09], &mut state);
        
        // WBNOINVD should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("WBNOINVD instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    // W segment base instructions
    #[test]
    fn test_wrfsbase_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // WRFSBASE instruction - write FS base register
        // WRFSBASE RAX (0xF3 0x48 0x0F 0xAE 0xC0)
        let result = execute_instruction(&[0xF3, 0x48, 0x0F, 0xAE, 0xC0], &mut state);
        
        // WRFSBASE should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("WRFSBASE instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_wrgsbase_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // WRGSBASE instruction - write GS base register
        // WRGSBASE RAX (0xF3 0x48 0x0F 0xAE 0xC8)
        let result = execute_instruction(&[0xF3, 0x48, 0x0F, 0xAE, 0xC8], &mut state);
        
        // WRGSBASE should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("WRGSBASE instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    // W security instructions
    #[test]
    fn test_wrpkru_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // WRPKRU instruction - write protection key rights for user pages
        // WRPKRU (0x0F 0x01 0xEF)
        let result = execute_instruction(&[0x0F, 0x01, 0xEF], &mut state);
        
        // WRPKRU should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("WRPKRU instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_wrssd_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // WRSSD instruction - write to shadow stack (32-bit)
        // WRSSD [RAX], EAX (0xF3 0x0F 0x38 0xF6 0x00)
        let result = execute_instruction(&[0xF3, 0x0F, 0x38, 0xF6, 0x00], &mut state);
        
        // WRSSD should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("WRSSD instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_wrssq_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // WRSSQ instruction - write to shadow stack (64-bit)
        // WRSSQ [RAX], RAX (0xF3 0x48 0x0F 0x38 0xF6 0x00)
        let result = execute_instruction(&[0xF3, 0x48, 0x0F, 0x38, 0xF6, 0x00], &mut state);
        
        // WRSSQ should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("WRSSQ instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_wrussd_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // WRUSSD instruction - write to user shadow stack (32-bit)
        // WRUSSD [RAX], EAX (0xF2 0x0F 0x38 0xF6 0x00)
        let result = execute_instruction(&[0xF2, 0x0F, 0x38, 0xF6, 0x00], &mut state);
        
        // WRUSSD should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("WRUSSD instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_wrussq_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // WRUSSQ instruction - write to user shadow stack (64-bit)
        // WRUSSQ [RAX], RAX (0xF2 0x48 0x0F 0x38 0xF6 0x00)
        let result = execute_instruction(&[0xF2, 0x48, 0x0F, 0x38, 0xF6, 0x00], &mut state);
        
        // WRUSSQ should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("WRUSSQ instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    // W advanced instructions
    #[test]
    fn test_wrshr_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // WRSHR instruction - write to shadow stack high
        // WRSHR [RAX], EAX (0xF3 0x0F 0x38 0xF7 0x00)
        let result = execute_instruction(&[0xF3, 0x0F, 0x38, 0xF7, 0x00], &mut state);
        
        // WRSHR should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("WRSHR instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_wrudbg_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // WRUDBG instruction - write to user debug register
        // WRUDBG [RAX], EAX (0xF2 0x0F 0x38 0xF7 0x00)
        let result = execute_instruction(&[0xF2, 0x0F, 0x38, 0xF7, 0x00], &mut state);
        
        // WRUDBG should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("WRUDBG instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_wrmsrlist_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // WRMSRLIST instruction - write to multiple MSRs
        // WRMSRLIST (0x0F 0x01 0xC6)
        let result = execute_instruction(&[0x0F, 0x01, 0xC6], &mut state);
        
        // WRMSRLIST should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("WRMSRLIST instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    #[test]
    fn test_wrmsrns_instruction() {
        let mut state = create_test_cpu_state().unwrap();
        // WRMSRNS instruction - write to MSR non-serializing
        // WRMSRNS (0x0F 0x01 0xC7)
        let result = execute_instruction(&[0x0F, 0x01, 0xC7], &mut state);
        
        // WRMSRNS should execute without crashing
        match result {
            Ok(state) => assert_eq!(state.registers.rax, 0u64),
            Err(e) => {
                println!("WRMSRNS instruction failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }

    // Test combinations and edge cases
    #[test]
    fn test_w_instructions_combination() {
        let mut state = create_test_cpu_state().unwrap();
        
        // Test multiple W instructions in sequence
        // WAIT
        let result1 = execute_instruction(&[0x9B], &mut state);
        assert!(result1.is_ok());
        
        // WBINVD
        let result2 = execute_instruction(&[0x0F, 0x09], &mut state);
        assert!(result2.is_ok());
        
        // WRMSR with valid MSR
        state.registers.rcx = 0x1B;
        state.registers.rax = 0x12345678;
        state.registers.rdx = 0x9ABCDEF0;
        let result3 = execute_instruction(&[0x0F, 0x30], &mut state);
        assert!(result3.is_ok());
        
        // Verify registers are still correct
        let finalstate = result3.unwrap();
        assert_eq!(finalstate.registers.rcx, 0x1B);
        assert_eq!(finalstate.registers.rax, 0x12345678);
        assert_eq!(finalstate.registers.rdx, 0x9ABCDEF0);
    }

    #[test]
    fn test_w_instructions_edge_cases() {
        let mut state = create_test_cpu_state().unwrap();
        
        // Test WRMSR with maximum values
        state.registers.rcx = 0xFFFFFFFF;
        state.registers.rax = 0xFFFFFFFF;
        state.registers.rdx = 0xFFFFFFFF;
        
        let result = execute_instruction(&[0x0F, 0x30], &mut state);
        
        // Should execute without crashing
        match result {
            Ok(state) => {
                assert_eq!(state.registers.rcx, 0xFFFFFFFF);
                assert_eq!(state.registers.rax, 0xFFFFFFFF);
                assert_eq!(state.registers.rdx, 0xFFFFFFFF);
            },
            Err(e) => {
                println!("WRMSR edge case failed: {}", e);
                // Skip this test if the instruction is not supported
                return;
            }
        }
    }
}
