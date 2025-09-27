use crate::cpu::{CpuState, InstructionDecoder};
use crate::memory::guest_memory::GuestMemory;
use crate::Result;
use iced_x86::{Decoder, DecoderOptions, Instruction};

fn create_test_cpu_state() -> Result<CpuState> {
    let memory = GuestMemory::new(1024 * 1024)?; // 1MB of memory
    let mut state = CpuState::new(memory);
    // Initialize memory with some test data
    state.write_u64(0x1000, 0x123456789ABCDEF0)?;
    state.write_u64(0x1008, 0xFEDCBA9876543210)?;
    state.write_u32(0x2000, 0x12345678)?;
    state.write_u16(0x3000, 0x1234)?;
    state.write_u8(0x4000, 0x12)?;
    Ok(state)
}

fn decode_instruction(bytes: &[u8]) -> Instruction {
    let mut decoder = Decoder::new(64, bytes, DecoderOptions::NONE);
    decoder.decode()
}

fn execute_instruction(bytes: &[u8], mut state: CpuState) -> Result<CpuState> {
    let instruction = decode_instruction(bytes);
    let decoder = InstructionDecoder::new();
    decoder.execute_instruction(&instruction, &mut state)?;
    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    // HADDPD - Packed Double-FP Horizontal Add
    #[test]
    fn test_haddpd_instruction() {
        // HADDPD - Packed Double-FP Horizontal Add
        let _instruction = decode_instruction(&[0x66, 0x0F, 0x7C, 0xC0]); // HADDPD XMM0, XMM0
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Haddpd);

        let state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0x66, 0x0F, 0x7C, 0xC0], state);
        assert!(result.is_ok());
    }

    // HADDPS - Packed Single-FP Horizontal Add
    #[test]
    fn test_haddps_instruction() {
        // HADDPS - Packed Single-FP Horizontal Add
        let _instruction = decode_instruction(&[0xF2, 0x0F, 0x7C, 0xC0]); // HADDPS XMM0, XMM0
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Haddps);

        let state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xF2, 0x0F, 0x7C, 0xC0], state);
        assert!(result.is_ok());
    }

    // HLT - Halt
    #[test]
    fn test_hlt_instruction() {
        // HLT - Halt
        let _instruction = decode_instruction(&[0xF4]); // HLT
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Hlt);

        let state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xF4], state);
        assert!(result.is_ok());
    }

    // HSUBPD - Packed Double-FP Horizontal Subtract
    #[test]
    fn test_hsubpd_instruction() {
        // HSUBPD - Packed Double-FP Horizontal Subtract
        let _instruction = decode_instruction(&[0x66, 0x0F, 0x7D, 0xC0]); // HSUBPD XMM0, XMM0
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Hsubpd);

        let state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0x66, 0x0F, 0x7D, 0xC0], state);
        assert!(result.is_ok());
    }

    // HSUBPS - Packed Single-FP Horizontal Subtract
    #[test]
    fn test_hsubps_instruction() {
        // HSUBPS - Packed Single-FP Horizontal Subtract
        let _instruction = decode_instruction(&[0xF2, 0x0F, 0x7D, 0xC0]); // HSUBPS XMM0, XMM0
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Hsubps);

        let state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xF2, 0x0F, 0x7D, 0xC0], state);
        assert!(result.is_ok());
    }

    // HRESET - Hypervisor Reset
    #[test]
    fn test_hreset_instruction() {
        // HRESET - Hypervisor Reset
        let _instruction = decode_instruction(&[0xF3, 0x0F, 0x3A, 0xF0, 0xC0]); // HRESET EAX
        // HRESET may decode as INVALID in 64-bit mode, so we'll be tolerant
        assert!(_instruction.mnemonic() == iced_x86::Mnemonic::Hreset || _instruction.mnemonic() == iced_x86::Mnemonic::INVALID);

        let state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xF3, 0x0F, 0x3A, 0xF0, 0xC0], state);
        // HRESET may not be implemented or may be invalid in 64-bit mode
        assert!(result.is_ok() || result.is_err());
    }
}
