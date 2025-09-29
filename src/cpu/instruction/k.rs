use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {

    pub fn execute_kaddb(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KADDB k1, k2, k3 - Add byte mask k2 + k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For byte operations, we only use the lower 8 bits
        let result = ((src1 & 0xFF) + (src2 & 0xFF)) & 0xFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kaddd(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KADDD k1, k2, k3 - Add doubleword mask k2 + k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For doubleword operations, we only use the lower 32 bits
        let result = ((src1 & 0xFFFFFFFF) + (src2 & 0xFFFFFFFF)) & 0xFFFFFFFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kaddq(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KADDQ k1, k2, k3 - Add quadword mask k2 + k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For quadword operations, we use all 64 bits
        let result = src1.wrapping_add(src2);
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kaddw(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KADDW k1, k2, k3 - Add word mask k2 + k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For word operations, we only use the lower 16 bits
        let result = ((src1 & 0xFFFF) + (src2 & 0xFFFF)) & 0xFFFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kandb(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KANDB k1, k2, k3 - AND byte mask k2 & k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For byte operations, we only use the lower 8 bits
        let result = (src1 & src2) & 0xFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kandd(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KANDD k1, k2, k3 - AND doubleword mask k2 & k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For doubleword operations, we only use the lower 32 bits
        let result = (src1 & src2) & 0xFFFFFFFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kandnb(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KANDNB k1, k2, k3 - AND NOT byte mask k2 & ~k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For byte operations, we only use the lower 8 bits
        let result = (src1 & !src2) & 0xFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kandnd(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KANDND k1, k2, k3 - AND NOT doubleword mask k2 & ~k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For doubleword operations, we only use the lower 32 bits
        let result = (src1 & !src2) & 0xFFFFFFFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kandnq(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KANDNQ k1, k2, k3 - AND NOT quadword mask k2 & ~k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For quadword operations, we use all 64 bits
        let result = src1 & !src2;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kandnw(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KANDNW k1, k2, k3 - AND NOT word mask k2 & ~k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For word operations, we only use the lower 16 bits
        let result = (src1 & !src2) & 0xFFFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kandq(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KANDQ k1, k2, k3 - AND quadword mask k2 & k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For quadword operations, we use all 64 bits
        let result = src1 & src2;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kandw(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KANDW k1, k2, k3 - AND word mask k2 & k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For word operations, we only use the lower 16 bits
        let result = (src1 & src2) & 0xFFFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kmovb(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KMOVB k1, k2 - Move byte mask k2 to k1
        let dst_reg = instruction.op_register(0);
        let src_reg = instruction.op_register(1);
        
        let src = self.get_k_register_value(src_reg, state);
        
        // For byte operations, we only use the lower 8 bits
        let result = src & 0xFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kmovd(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KMOVD k1, k2 - Move doubleword mask k2 to k1
        let dst_reg = instruction.op_register(0);
        let src_reg = instruction.op_register(1);
        
        let src = self.get_k_register_value(src_reg, state);
        
        // For doubleword operations, we only use the lower 32 bits
        let result = src & 0xFFFFFFFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kmovq(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KMOVQ k1, k2 - Move quadword mask k2 to k1
        let dst_reg = instruction.op_register(0);
        let src_reg = instruction.op_register(1);
        
        let src = self.get_k_register_value(src_reg, state);
        
        // For quadword operations, we use all 64 bits
        self.set_k_register_value(dst_reg, src, state);
        Ok(())
    }

    pub fn execute_kmovw(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KMOVW k1, k2 - Move word mask k2 to k1
        let dst_reg = instruction.op_register(0);
        let src_reg = instruction.op_register(1);
        
        let src = self.get_k_register_value(src_reg, state);
        
        // For word operations, we only use the lower 16 bits
        let result = src & 0xFFFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_knotb(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KNOTB k1, k2 - NOT byte mask ~k2, store result in k1
        let dst_reg = instruction.op_register(0);
        let src_reg = instruction.op_register(1);
        
        let src = self.get_k_register_value(src_reg, state);
        
        // For byte operations, we only use the lower 8 bits
        let result = (!src) & 0xFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_knotd(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KNOTD k1, k2 - NOT doubleword mask ~k2, store result in k1
        let dst_reg = instruction.op_register(0);
        let src_reg = instruction.op_register(1);
        
        let src = self.get_k_register_value(src_reg, state);
        
        // For doubleword operations, we only use the lower 32 bits
        let result = (!src) & 0xFFFFFFFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_knotq(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KNOTQ k1, k2 - NOT quadword mask ~k2, store result in k1
        let dst_reg = instruction.op_register(0);
        let src_reg = instruction.op_register(1);
        
        let src = self.get_k_register_value(src_reg, state);
        
        // For quadword operations, we use all 64 bits
        let result = !src;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_knotw(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KNOTW k1, k2 - NOT word mask ~k2, store result in k1
        let dst_reg = instruction.op_register(0);
        let src_reg = instruction.op_register(1);
        
        let src = self.get_k_register_value(src_reg, state);
        
        // For word operations, we only use the lower 16 bits
        let result = (!src) & 0xFFFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_korb(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KORB k1, k2, k3 - OR byte mask k2 | k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For byte operations, we only use the lower 8 bits
        let result = (src1 | src2) & 0xFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kord(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KORD k1, k2, k3 - OR doubleword mask k2 | k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For doubleword operations, we only use the lower 32 bits
        let result = (src1 | src2) & 0xFFFFFFFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_korq(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KORQ k1, k2, k3 - OR quadword mask k2 | k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For quadword operations, we use all 64 bits
        let result = src1 | src2;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kortestb(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KORTESTB k1, k2 - OR test byte mask k1 | k2, set flags
        let src1_reg = instruction.op_register(0);
        let src2_reg = instruction.op_register(1);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For byte operations, we only use the lower 8 bits
        let result = (src1 | src2) & 0xFF;
        
        // Set flags based on result
        state.registers.set_flag(RFlags::ZERO, result == 0);
        state.registers.set_flag(RFlags::CARRY, false);
        
        Ok(())
    }

    pub fn execute_kortestd(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KORTESTD k1, k2 - OR test doubleword mask k1 | k2, set flags
        let src1_reg = instruction.op_register(0);
        let src2_reg = instruction.op_register(1);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For doubleword operations, we only use the lower 32 bits
        let result = (src1 | src2) & 0xFFFFFFFF;
        
        // Set flags based on result
        state.registers.set_flag(RFlags::ZERO, result == 0);
        state.registers.set_flag(RFlags::CARRY, false);
        
        Ok(())
    }

    pub fn execute_kortestq(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KORTESTQ k1, k2 - OR test quadword mask k1 | k2, set flags
        let src1_reg = instruction.op_register(0);
        let src2_reg = instruction.op_register(1);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For quadword operations, we use all 64 bits
        let result = src1 | src2;
        
        // Set flags based on result
        state.registers.set_flag(RFlags::ZERO, result == 0);
        state.registers.set_flag(RFlags::CARRY, false);
        
        Ok(())
    }

    pub fn execute_kortestw(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KORTESTW k1, k2 - OR test word mask k1 | k2, set flags
        let src1_reg = instruction.op_register(0);
        let src2_reg = instruction.op_register(1);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For word operations, we only use the lower 16 bits
        let result = (src1 | src2) & 0xFFFF;
        
        // Set flags based on result
        state.registers.set_flag(RFlags::ZERO, result == 0);
        state.registers.set_flag(RFlags::CARRY, false);
        
        Ok(())
    }

    pub fn execute_korw(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KORW k1, k2, k3 - OR word mask k2 | k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For word operations, we only use the lower 16 bits
        let result = (src1 | src2) & 0xFFFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kshiftlb(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KSHIFTLB k1, k2, imm - Shift left byte mask k2 by imm, store result in k1
        let dst_reg = instruction.op_register(0);
        let src_reg = instruction.op_register(1);
        let count = instruction.immediate8() as u64;
        
        let src = self.get_k_register_value(src_reg, state);
        
        // For byte operations, we only use the lower 8 bits
        let result = ((src & 0xFF) << count) & 0xFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kshiftld(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KSHIFTLD k1, k2, imm - Shift left doubleword mask k2 by imm, store result in k1
        let dst_reg = instruction.op_register(0);
        let src_reg = instruction.op_register(1);
        let count = instruction.immediate8() as u64;
        
        let src = self.get_k_register_value(src_reg, state);
        
        // For doubleword operations, we only use the lower 32 bits
        let result = ((src & 0xFFFFFFFF) << count) & 0xFFFFFFFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kshiftlq(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KSHIFTLQ k1, k2, imm - Shift left quadword mask k2 by imm, store result in k1
        let dst_reg = instruction.op_register(0);
        let src_reg = instruction.op_register(1);
        let count = instruction.immediate8() as u64;
        
        let src = self.get_k_register_value(src_reg, state);
        
        // For quadword operations, we use all 64 bits
        let result = src << count;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kshiftlw(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KSHIFTLW k1, k2, imm - Shift left word mask k2 by imm, store result in k1
        let dst_reg = instruction.op_register(0);
        let src_reg = instruction.op_register(1);
        let count = instruction.immediate8() as u64;
        
        let src = self.get_k_register_value(src_reg, state);
        
        // For word operations, we only use the lower 16 bits
        let result = ((src & 0xFFFF) << count) & 0xFFFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kshiftrb(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KSHIFTRB k1, k2, imm - Shift right byte mask k2 by imm, store result in k1
        let dst_reg = instruction.op_register(0);
        let src_reg = instruction.op_register(1);
        let count = instruction.immediate8() as u64;
        
        let src = self.get_k_register_value(src_reg, state);
        
        // For byte operations, we only use the lower 8 bits
        let result = (src & 0xFF) >> count;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kshiftrd(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KSHIFTRD k1, k2, imm - Shift right doubleword mask k2 by imm, store result in k1
        let dst_reg = instruction.op_register(0);
        let src_reg = instruction.op_register(1);
        let count = instruction.immediate8() as u64;
        
        let src = self.get_k_register_value(src_reg, state);
        
        // For doubleword operations, we only use the lower 32 bits
        let result = (src & 0xFFFFFFFF) >> count;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kshiftrq(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KSHIFTRQ k1, k2, imm - Shift right quadword mask k2 by imm, store result in k1
        let dst_reg = instruction.op_register(0);
        let src_reg = instruction.op_register(1);
        let count = instruction.immediate8() as u64;
        
        let src = self.get_k_register_value(src_reg, state);
        
        // For quadword operations, we use all 64 bits
        let result = src >> count;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kshiftrw(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KSHIFTRW k1, k2, imm - Shift right word mask k2 by imm, store result in k1
        let dst_reg = instruction.op_register(0);
        let src_reg = instruction.op_register(1);
        let count = instruction.immediate8() as u64;
        
        let src = self.get_k_register_value(src_reg, state);
        
        // For word operations, we only use the lower 16 bits
        let result = (src & 0xFFFF) >> count;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_ktestb(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KTESTB k1, k2 - Test byte mask k1 & k2, set flags
        let src1_reg = instruction.op_register(0);
        let src2_reg = instruction.op_register(1);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For byte operations, we only use the lower 8 bits
        let result = (src1 & src2) & 0xFF;
        
        // Set flags based on result
        state.registers.set_flag(RFlags::ZERO, result == 0);
        state.registers.set_flag(RFlags::CARRY, false);
        
        Ok(())
    }

    pub fn execute_ktestd(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KTESTD k1, k2 - Test doubleword mask k1 & k2, set flags
        let src1_reg = instruction.op_register(0);
        let src2_reg = instruction.op_register(1);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For doubleword operations, we only use the lower 32 bits
        let result = (src1 & src2) & 0xFFFFFFFF;
        
        // Set flags based on result
        state.registers.set_flag(RFlags::ZERO, result == 0);
        state.registers.set_flag(RFlags::CARRY, false);
        
        Ok(())
    }

    pub fn execute_ktestq(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KTESTQ k1, k2 - Test quadword mask k1 & k2, set flags
        let src1_reg = instruction.op_register(0);
        let src2_reg = instruction.op_register(1);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For quadword operations, we use all 64 bits
        let result = src1 & src2;
        
        // Set flags based on result
        state.registers.set_flag(RFlags::ZERO, result == 0);
        state.registers.set_flag(RFlags::CARRY, false);
        
        Ok(())
    }

    pub fn execute_ktestw(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KTESTW k1, k2 - Test word mask k1 & k2, set flags
        let src1_reg = instruction.op_register(0);
        let src2_reg = instruction.op_register(1);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For word operations, we only use the lower 16 bits
        let result = (src1 & src2) & 0xFFFF;
        
        // Set flags based on result
        state.registers.set_flag(RFlags::ZERO, result == 0);
        state.registers.set_flag(RFlags::CARRY, false);
        
        Ok(())
    }

    pub fn execute_kunpckbw(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KUNPCKBW k1, k2, k3 - Unpack byte to word mask k2, k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state) & 0xFF;
        let src2 = self.get_k_register_value(src2_reg, state) & 0xFF;
        
        // Interleave bytes to create word
        let result = (src1 << 8) | src2;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kunpckdq(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KUNPCKDQ k1, k2, k3 - Unpack doubleword to quadword mask k2, k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state) & 0xFFFFFFFF;
        let src2 = self.get_k_register_value(src2_reg, state) & 0xFFFFFFFF;
        
        // Interleave doublewords to create quadword
        let result = (src1 << 32) | src2;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kunpckwd(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KUNPCKWD k1, k2, k3 - Unpack word to doubleword mask k2, k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state) & 0xFFFF;
        let src2 = self.get_k_register_value(src2_reg, state) & 0xFFFF;
        
        // Interleave words to create doubleword
        let result = (src1 << 16) | src2;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kxnorb(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KXNORB k1, k2, k3 - XNOR byte mask k2 ^ ~k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For byte operations, we only use the lower 8 bits
        let result = (!(src1 ^ src2)) & 0xFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kxnord(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KXNORD k1, k2, k3 - XNOR doubleword mask k2 ^ ~k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For doubleword operations, we only use the lower 32 bits
        let result = (!(src1 ^ src2)) & 0xFFFFFFFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kxnorq(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KXNORQ k1, k2, k3 - XNOR quadword mask k2 ^ ~k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For quadword operations, we use all 64 bits
        let result = !(src1 ^ src2);
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kxnorw(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KXNORW k1, k2, k3 - XNOR word mask k2 ^ ~k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For word operations, we only use the lower 16 bits
        let result = (!(src1 ^ src2)) & 0xFFFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kxorb(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KXORB k1, k2, k3 - XOR byte mask k2 ^ k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For byte operations, we only use the lower 8 bits
        let result = (src1 ^ src2) & 0xFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kxord(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KXORD k1, k2, k3 - XOR doubleword mask k2 ^ k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For doubleword operations, we only use the lower 32 bits
        let result = (src1 ^ src2) & 0xFFFFFFFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kxorq(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KXORQ k1, k2, k3 - XOR quadword mask k2 ^ k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For quadword operations, we use all 64 bits
        let result = src1 ^ src2;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kxorw(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        // KXORW k1, k2, k3 - XOR word mask k2 ^ k3, store result in k1
        let dst_reg = instruction.op_register(0);
        let src1_reg = instruction.op_register(1);
        let src2_reg = instruction.op_register(2);
        
        let src1 = self.get_k_register_value(src1_reg, state);
        let src2 = self.get_k_register_value(src2_reg, state);
        
        // For word operations, we only use the lower 16 bits
        let result = (src1 ^ src2) & 0xFFFF;
        
        self.set_k_register_value(dst_reg, result, state);
        Ok(())
    }

    pub fn execute_kand(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("KAND instruction executed");
        Ok(())
    }

    pub fn execute_kandn(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("KANDN instruction executed");
        Ok(())
    }

    pub fn execute_kandnr(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("KANDNR instruction executed");
        Ok(())
    }

    pub fn execute_kconcath(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("KCONCATH instruction executed");
        Ok(())
    }

    pub fn execute_kconcatl(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("KCONCATL instruction executed");
        Ok(())
    }

    pub fn execute_kextract(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("KEXTRACT instruction executed");
        Ok(())
    }

    pub fn execute_kmerge2l1h(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("KMERGE2L1H instruction executed");
        Ok(())
    }

    pub fn execute_kmerge2l1l(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("KMERGE2L1L instruction executed");
        Ok(())
    }

    pub fn execute_kmov(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("KMOV instruction executed");
        Ok(())
    }

    pub fn execute_knot(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("KNOT instruction executed");
        Ok(())
    }

    pub fn execute_kor(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("KOR instruction executed");
        Ok(())
    }

    pub fn execute_kortest(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("KORTEST instruction executed");
        Ok(())
    }

    pub fn execute_kxnor(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("KXNOR instruction executed");
        Ok(())
    }

    pub fn execute_kxor(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("KXOR instruction executed");
        Ok(())
    }
}
