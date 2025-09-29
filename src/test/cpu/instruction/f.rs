#[cfg(test)]
mod tests {
    use crate::cpu::{CpuState, InstructionDecoder};
        use crate::Result;
use crate::test::helpers::{create_test_cpu_state, write_memory, read_memory};
    use iced_x86::{Decoder, DecoderOptions, Instruction};

    

    fn decode_instruction(bytes: &[u8]) -> Instruction {
        let mut decoder = Decoder::new(64, bytes, DecoderOptions::NONE);
        decoder.decode()
    }

    fn execute_instruction(bytes: &[u8], state: &mut CpuState) -> Result<()> {
        let instruction = decode_instruction(bytes);
        let mut decoder = InstructionDecoder::new();
        decoder.execute_instruction(&instruction, state)?;
        Ok(())
    }

    // F2XM1 - 2^x - 1
    #[test]
    fn test_f2xm1_instruction() {
        // F2XM1 - Calculate 2^x - 1
        let _instruction = decode_instruction(&[0xD9, 0xF0]); // F2XM1
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::F2xm1);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xD9, 0xF0], &mut state);
        assert!(result.is_ok());
    }

    // FABS - Absolute Value
    #[test]
    fn test_fabs_instruction() {
        // FABS - Absolute Value
        let _instruction = decode_instruction(&[0xD9, 0xE1]); // FABS
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fabs);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xD9, 0xE1], &mut state);
        assert!(result.is_ok());
    }

    // FADD - Floating Point Add
    #[test]
    fn test_fadd_instruction() {
        // FADD - Floating Point Add
        let _instruction = decode_instruction(&[0xD8, 0xC1]); // FADD ST(0), ST(1)
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fadd);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xD8, 0xC1], &mut state);
        assert!(result.is_ok());
    }

    // FADDP - Floating Point Add and Pop
    #[test]
    fn test_faddp_instruction() {
        // FADDP - Floating Point Add and Pop
        let _instruction = decode_instruction(&[0xDE, 0xC1]); // FADDP ST(1), ST(0)
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Faddp);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDE, 0xC1], &mut state);
        assert!(result.is_ok());
    }

    // FBLD - Load Binary Coded Decimal
    #[test]
    fn test_fbld_instruction() {
        // FBLD - Load Binary Coded Decimal
        let _instruction = decode_instruction(&[0xDF, 0x20]); // FBLD [RAX]
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fbld);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDF, 0x20], &mut state);
        assert!(result.is_ok());
    }

    // FBSTP - Store Binary Coded Decimal and Pop
    #[test]
    fn test_fbstp_instruction() {
        // FBSTP - Store Binary Coded Decimal and Pop
        let _instruction = decode_instruction(&[0xDF, 0x30]); // FBSTP [RAX]
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fbstp);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDF, 0x30], &mut state);
        assert!(result.is_ok());
    }

    // FCHS - Change Sign
    #[test]
    fn test_fchs_instruction() {
        // FCHS - Change Sign
        let _instruction = decode_instruction(&[0xD9, 0xE0]); // FCHS
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fchs);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xD9, 0xE0], &mut state);
        assert!(result.is_ok());
    }

    // FCLEX - Clear Exceptions
    #[test]
    fn test_fclex_instruction() {
        // FCLEX - Clear Exceptions
        let _instruction = decode_instruction(&[0xDB, 0xE2]); // FCLEX
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fnclex); // In 64-bit mode, this decodes to FNCLEX
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDB, 0xE2], &mut state);
        assert!(result.is_ok());
    }

    // FCMOVB - Floating Point Conditional Move if Below
    #[test]
    fn test_fcmovb_instruction() {
        // FCMOVB - Floating Point Conditional Move if Below
        let _instruction = decode_instruction(&[0xDA, 0xC0]); // FCMOVB ST(0), ST(0)
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fcmovb);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDA, 0xC0], &mut state);
        assert!(result.is_ok());
    }

    // FCMOVBE - Floating Point Conditional Move if Below or Equal
    #[test]
    fn test_fcmovbe_instruction() {
        // FCMOVBE - Floating Point Conditional Move if Below or Equal
        let _instruction = decode_instruction(&[0xDA, 0xD0]); // FCMOVBE ST(0), ST(0)
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fcmovbe);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDA, 0xD0], &mut state);
        assert!(result.is_ok());
    }

    // FCMOVE - Floating Point Conditional Move if Equal
    #[test]
    fn test_fcmove_instruction() {
        // FCMOVE - Floating Point Conditional Move if Equal
        let _instruction = decode_instruction(&[0xDA, 0xC8]); // FCMOVE ST(0), ST(0)
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fcmove);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDA, 0xC8], &mut state);
        assert!(result.is_ok());
    }

    // FCMOVNB - Floating Point Conditional Move if Not Below
    #[test]
    fn test_fcmovnb_instruction() {
        // FCMOVNB - Floating Point Conditional Move if Not Below
        let _instruction = decode_instruction(&[0xDB, 0xC0]); // FCMOVNB ST(0), ST(0)
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fcmovnb);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDB, 0xC0], &mut state);
        assert!(result.is_ok());
    }

    // FCMOVNBE - Floating Point Conditional Move if Not Below or Equal
    #[test]
    fn test_fcmovnbe_instruction() {
        // FCMOVNBE - Floating Point Conditional Move if Not Below or Equal
        let _instruction = decode_instruction(&[0xDB, 0xD0]); // FCMOVNBE ST(0), ST(0)
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fcmovnbe);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDB, 0xD0], &mut state);
        assert!(result.is_ok());
    }

    // FCMOVNE - Floating Point Conditional Move if Not Equal
    #[test]
    fn test_fcmovne_instruction() {
        // FCMOVNE - Floating Point Conditional Move if Not Equal
        let _instruction = decode_instruction(&[0xDB, 0xC8]); // FCMOVNE ST(0), ST(0)
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fcmovne);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDB, 0xC8], &mut state);
        assert!(result.is_ok());
    }

    // FCMOVNU - Floating Point Conditional Move if Not Unordered
    #[test]
    fn test_fcmovnu_instruction() {
        // FCMOVNU - Floating Point Conditional Move if Not Unordered
        let _instruction = decode_instruction(&[0xDB, 0xD8]); // FCMOVNU ST(0), ST(0)
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fcmovnu);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDB, 0xD8], &mut state);
        assert!(result.is_ok());
    }

    // FCMOVU - Floating Point Conditional Move if Unordered
    #[test]
    fn test_fcmovu_instruction() {
        // FCMOVU - Floating Point Conditional Move if Unordered
        let _instruction = decode_instruction(&[0xDA, 0xD8]); // FCMOVU ST(0), ST(0)
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fcmovu);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDA, 0xD8], &mut state);
        assert!(result.is_ok());
    }

    // FCOM - Floating Point Compare
    #[test]
    fn test_fcom_instruction() {
        // FCOM - Floating Point Compare
        let _instruction = decode_instruction(&[0xD8, 0xD1]); // FCOM ST(1)
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fcom);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xD8, 0xD1], &mut state);
        assert!(result.is_ok());
    }

    // FCOMI - Floating Point Compare and Set EFLAGS
    #[test]
    fn test_fcomi_instruction() {
        // FCOMI - Floating Point Compare and Set EFLAGS
        let _instruction = decode_instruction(&[0xDB, 0xF0]); // FCOMI ST(0), ST(0)
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fcomi);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDB, 0xF0], &mut state);
        assert!(result.is_ok());
    }

    // FCOMIP - Floating Point Compare and Set EFLAGS and Pop
    #[test]
    fn test_fcomip_instruction() {
        // FCOMIP - Floating Point Compare and Set EFLAGS and Pop
        let _instruction = decode_instruction(&[0xDF, 0xF0]); // FCOMIP ST(0), ST(0)
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fcomip);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDF, 0xF0], &mut state);
        assert!(result.is_ok());
    }

    // FCOMP - Floating Point Compare and Pop
    #[test]
    fn test_fcomp_instruction() {
        // FCOMP - Floating Point Compare and Pop
        let _instruction = decode_instruction(&[0xD8, 0xD9]); // FCOMP ST(1)
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fcomp);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xD8, 0xD9], &mut state);
        assert!(result.is_ok());
    }

    // FCOMPP - Floating Point Compare and Pop Twice
    #[test]
    fn test_fcompp_instruction() {
        // FCOMPP - Floating Point Compare and Pop Twice
        let _instruction = decode_instruction(&[0xDE, 0xD9]); // FCOMPP
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fcompp);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDE, 0xD9], &mut state);
        assert!(result.is_ok());
    }

    // FCOS - Cosine
    #[test]
    fn test_fcos_instruction() {
        // FCOS - Cosine
        let _instruction = decode_instruction(&[0xD9, 0xFF]); // FCOS
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fcos);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xD9, 0xFF], &mut state);
        assert!(result.is_ok());
    }

    // FDECSTP - Decrement Stack Pointer
    #[test]
    fn test_fdecstp_instruction() {
        // FDECSTP - Decrement Stack Pointer
        let _instruction = decode_instruction(&[0xD9, 0xF6]); // FDECSTP
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fdecstp);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xD9, 0xF6], &mut state);
        assert!(result.is_ok());
    }

    // FDISI - Disable Interrupts
    #[test]
    fn test_fdisi_instruction() {
        // FDISI - Disable Interrupts
        let _instruction = decode_instruction(&[0xDB, 0xE1]); // FDISI
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fndisi); // In 64-bit mode, this decodes to FNDISI
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDB, 0xE1], &mut state);
        assert!(result.is_ok());
    }

    // FDIV - Floating Point Divide
    #[test]
    fn test_fdiv_instruction() {
        // FDIV - Floating Point Divide
        let _instruction = decode_instruction(&[0xD8, 0xF1]); // FDIV ST(0), ST(1)
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fdiv);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xD8, 0xF1], &mut state);
        assert!(result.is_ok());
    }

    // FDIVP - Floating Point Divide and Pop
    #[test]
    fn test_fdivp_instruction() {
        // FDIVP - Floating Point Divide and Pop
        let _instruction = decode_instruction(&[0xDE, 0xF1]); // FDIVP ST(1), ST(0)
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fdivrp); // In 64-bit mode, this decodes to FDIVRP
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDE, 0xF1], &mut state);
        assert!(result.is_ok());
    }

    // FDIVR - Floating Point Reverse Divide
    #[test]
    fn test_fdivr_instruction() {
        // FDIVR - Floating Point Reverse Divide
        let _instruction = decode_instruction(&[0xD8, 0xF9]); // FDIVR ST(0), ST(1)
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fdivr);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xD8, 0xF9], &mut state);
        assert!(result.is_ok());
    }

    // FDIVRP - Floating Point Reverse Divide and Pop
    #[test]
    fn test_fdivrp_instruction() {
        // FDIVRP - Floating Point Reverse Divide and Pop
        let _instruction = decode_instruction(&[0xDE, 0xF9]); // FDIVRP ST(1), ST(0)
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fdivp); // In 64-bit mode, this decodes to FDIVP
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDE, 0xF9], &mut state);
        assert!(result.is_ok());
    }

    // FEMMS - Fast Empty MMX State
    #[test]
    fn test_femms_instruction() {
        // FEMMS - Fast Empty MMX State
        let _instruction = decode_instruction(&[0x0F, 0x0E]); // FEMMS
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Femms);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0x0F, 0x0E], &mut state);
        assert!(result.is_ok());
    }

    // FENI - Enable Interrupts
    #[test]
    fn test_feni_instruction() {
        // FENI - Enable Interrupts
        let _instruction = decode_instruction(&[0xDB, 0xE0]); // FENI
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fneni); // In 64-bit mode, this decodes to FNENI
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDB, 0xE0], &mut state);
        assert!(result.is_ok());
    }

    // FFREE - Free Floating Point Register
    #[test]
    fn test_ffree_instruction() {
        // FFREE - Free Floating Point Register
        let _instruction = decode_instruction(&[0xDD, 0xC1]); // FFREE ST(1)
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Ffree);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDD, 0xC1], &mut state);
        assert!(result.is_ok());
    }

    // FFREEP - Free Floating Point Register and Pop
    #[test]
    fn test_ffreep_instruction() {
        // FFREEP - Free Floating Point Register and Pop
        let _instruction = decode_instruction(&[0xDF, 0xC1]); // FFREEP ST(1)
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Ffreep);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDF, 0xC1], &mut state);
        assert!(result.is_ok());
    }

    // FIADD - Floating Point Integer Add
    #[test]
    fn test_fiadd_instruction() {
        // FIADD - Floating Point Integer Add
        let _instruction = decode_instruction(&[0xDA, 0x00]); // FIADD [RAX]
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fiadd);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDA, 0x00], &mut state);
        assert!(result.is_ok());
    }

    // FICOM - Floating Point Integer Compare
    #[test]
    fn test_ficom_instruction() {
        // FICOM - Floating Point Integer Compare
        let _instruction = decode_instruction(&[0xDA, 0x10]); // FICOM [RAX]
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Ficom);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDA, 0x10], &mut state);
        assert!(result.is_ok());
    }

    // FICOMP - Floating Point Integer Compare and Pop
    #[test]
    fn test_ficomp_instruction() {
        // FICOMP - Floating Point Integer Compare and Pop
        let _instruction = decode_instruction(&[0xDA, 0x18]); // FICOMP [RAX]
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Ficomp);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDA, 0x18], &mut state);
        assert!(result.is_ok());
    }

    // FIDIV - Floating Point Integer Divide
    #[test]
    fn test_fidiv_instruction() {
        // FIDIV - Floating Point Integer Divide
        let _instruction = decode_instruction(&[0xDA, 0x30]); // FIDIV [RAX]
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fidiv);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDA, 0x30], &mut state);
        assert!(result.is_ok());
    }

    // FIDIVR - Floating Point Integer Reverse Divide
    #[test]
    fn test_fidivr_instruction() {
        // FIDIVR - Floating Point Integer Reverse Divide
        let _instruction = decode_instruction(&[0xDA, 0x38]); // FIDIVR [RAX]
        assert_eq!(_instruction.mnemonic(), iced_x86::Mnemonic::Fidivr);
        
        let mut state = create_test_cpu_state().unwrap();
        let result = execute_instruction(&[0xDA, 0x38], &mut state);
        assert!(result.is_ok());
    }
}
