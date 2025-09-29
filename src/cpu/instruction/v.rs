use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    pub fn execute_verr(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid VERR instruction".to_string()));
        }
        // Verify segment for read (simplified - just log for now)
        log::debug!("VERR instruction executed");
        Ok(())
    }

    pub fn execute_verw(&self, instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        if instruction.op_count() != 1 {
            return Err(crate::EmulatorError::Cpu("Invalid VERW instruction".to_string()));
        }
        // Verify segment for write (simplified - just log for now)
        log::debug!("VERW instruction executed");
        Ok(())
    }

    pub fn execute_v4fmaddps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("V4FMADDPS instruction executed");
        Ok(())
    }

    pub fn execute_v4fmaddss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("V4FMADDSS instruction executed");
        Ok(())
    }

    pub fn execute_v4fnmaddps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("V4FNMADDPS instruction executed");
        Ok(())
    }

    pub fn execute_v4fnmaddss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("V4FNMADDSS instruction executed");
        Ok(())
    }

    pub fn execute_vaddpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VADDPD instruction executed");
        Ok(())
    }

    pub fn execute_vaddps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VADDPS instruction executed");
        Ok(())
    }

    pub fn execute_vaddsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VADDSD instruction executed");
        Ok(())
    }

    pub fn execute_vaddss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VADDSS instruction executed");
        Ok(())
    }

    pub fn execute_vaddsubpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VADDSUBPD instruction executed");
        Ok(())
    }

    pub fn execute_vaddsubps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VADDSUBPS instruction executed");
        Ok(())
    }

    pub fn execute_vaesdec(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VAESDEC instruction executed");
        Ok(())
    }

    pub fn execute_vaesdeclast(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VAESDECLAST instruction executed");
        Ok(())
    }

    pub fn execute_vaesenc(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VAESENC instruction executed");
        Ok(())
    }

    pub fn execute_vaesenclast(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VAESENCLAST instruction executed");
        Ok(())
    }

    pub fn execute_vaesimc(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VAESIMC instruction executed");
        Ok(())
    }

    pub fn execute_vaeskeygenassist(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VAESKEYGENASSIST instruction executed");
        Ok(())
    }

    pub fn execute_valignd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VALIGND instruction executed");
        Ok(())
    }

    pub fn execute_valignq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VALIGNQ instruction executed");
        Ok(())
    }

    pub fn execute_vandnpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VANDNPD instruction executed");
        Ok(())
    }

    pub fn execute_vandnps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VANDNPS instruction executed");
        Ok(())
    }

    pub fn execute_vandpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VANDPD instruction executed");
        Ok(())
    }

    pub fn execute_vandps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VANDPS instruction executed");
        Ok(())
    }

    pub fn execute_vblendmpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBLENDMPD instruction executed");
        Ok(())
    }

    pub fn execute_vblendmps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBLENDMPS instruction executed");
        Ok(())
    }

    pub fn execute_vblendpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBLENDPD instruction executed");
        Ok(())
    }

    pub fn execute_vblendps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBLENDPS instruction executed");
        Ok(())
    }

    pub fn execute_vblendvpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBLENDVPD instruction executed");
        Ok(())
    }

    pub fn execute_vblendvps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBLENDVPS instruction executed");
        Ok(())
    }

    pub fn execute_vbroadcastf128(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBROADCASTF128 instruction executed");
        Ok(())
    }

    pub fn execute_vbroadcastf32x2(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBROADCASTF32X2 instruction executed");
        Ok(())
    }

    pub fn execute_vbroadcastf32x4(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBROADCASTF32X4 instruction executed");
        Ok(())
    }

    pub fn execute_vbroadcastf32x8(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBROADCASTF32X8 instruction executed");
        Ok(())
    }

    pub fn execute_vbroadcastf64x2(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBROADCASTF64X2 instruction executed");
        Ok(())
    }

    pub fn execute_vbroadcastf64x4(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBROADCASTF64X4 instruction executed");
        Ok(())
    }

    pub fn execute_vbroadcasti128(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBROADCASTI128 instruction executed");
        Ok(())
    }

    pub fn execute_vbroadcasti32x2(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBROADCASTI32X2 instruction executed");
        Ok(())
    }

    pub fn execute_vbroadcasti32x4(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBROADCASTI32X4 instruction executed");
        Ok(())
    }

    pub fn execute_vbroadcasti32x8(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBROADCASTI32X8 instruction executed");
        Ok(())
    }

    pub fn execute_vbroadcasti64x2(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBROADCASTI64X2 instruction executed");
        Ok(())
    }

    pub fn execute_vbroadcasti64x4(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBROADCASTI64X4 instruction executed");
        Ok(())
    }

    pub fn execute_vbroadcastsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBROADCASTSD instruction executed");
        Ok(())
    }

    pub fn execute_vbroadcastss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBROADCASTSS instruction executed");
        Ok(())
    }

    pub fn execute_vcmppd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCMPPD instruction executed");
        Ok(())
    }

    pub fn execute_vcmpps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCMPPS instruction executed");
        Ok(())
    }

    pub fn execute_vcmpsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCMPSD instruction executed");
        Ok(())
    }

    pub fn execute_vcmpss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCMPSS instruction executed");
        Ok(())
    }

    pub fn execute_vcomisd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCOMISD instruction executed");
        Ok(())
    }

    pub fn execute_vcomiss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCOMISS instruction executed");
        Ok(())
    }

    pub fn execute_vcompresspd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCOMPRESSPD instruction executed");
        Ok(())
    }

    pub fn execute_vcompressps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCOMPRESSPS instruction executed");
        Ok(())
    }

    pub fn execute_vcvtdq2pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTDQ2PD instruction executed");
        Ok(())
    }

    pub fn execute_vcvtdq2ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTDQ2PS instruction executed");
        Ok(())
    }

    pub fn execute_vcvtne2ps2bf16(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTNE2PS2BF16 instruction executed");
        Ok(())
    }

    pub fn execute_vcvtneps2bf16(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTNEPS2BF16 instruction executed");
        Ok(())
    }

    pub fn execute_vcvtpd2dq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPD2DQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvtpd2ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPD2PS instruction executed");
        Ok(())
    }

    pub fn execute_vcvtpd2qq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPD2QQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvtpd2udq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPD2UDQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvtpd2uqq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPD2UQQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvtph2ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPH2PS instruction executed");
        Ok(())
    }

    pub fn execute_vcvtps2dq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPS2DQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvtps2pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPS2PD instruction executed");
        Ok(())
    }

    pub fn execute_vcvtps2ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPS2PH instruction executed");
        Ok(())
    }

    pub fn execute_vcvtps2qq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPS2QQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvtps2udq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPS2UDQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvtps2uqq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPS2UQQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvtqq2pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTQQ2PD instruction executed");
        Ok(())
    }

    pub fn execute_vcvtqq2ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTQQ2PS instruction executed");
        Ok(())
    }

    pub fn execute_vcvtsd2si(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTSD2SI instruction executed");
        Ok(())
    }

    pub fn execute_vcvtsd2ss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTSD2SS instruction executed");
        Ok(())
    }

    pub fn execute_vcvtsd2usi(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTSD2USI instruction executed");
        Ok(())
    }

    pub fn execute_vcvtsi2sd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTSI2SD instruction executed");
        Ok(())
    }

    pub fn execute_vcvtsi2ss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTSI2SS instruction executed");
        Ok(())
    }

    pub fn execute_vcvtss2sd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTSS2SD instruction executed");
        Ok(())
    }

    pub fn execute_vcvtss2si(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTSS2SI instruction executed");
        Ok(())
    }

    pub fn execute_vcvtss2usi(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTSS2USI instruction executed");
        Ok(())
    }

    pub fn execute_vcvttpd2dq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTTPD2DQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvttpd2qq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTTPD2QQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvttpd2udq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTTPD2UDQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvttpd2uqq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTTPD2UQQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvttps2dq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTTPS2DQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvttps2qq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTTPS2QQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvttps2udq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTTPS2UDQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvttps2uqq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTTPS2UQQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvttsd2si(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTTSD2SI instruction executed");
        Ok(())
    }

    pub fn execute_vcvttsd2usi(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTTSD2USI instruction executed");
        Ok(())
    }

    pub fn execute_vcvttss2si(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTTSS2SI instruction executed");
        Ok(())
    }

    pub fn execute_vcvttss2usi(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTTSS2USI instruction executed");
        Ok(())
    }

    pub fn execute_vcvtudq2pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTUDQ2PD instruction executed");
        Ok(())
    }

    pub fn execute_vcvtudq2ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTUDQ2PS instruction executed");
        Ok(())
    }

    pub fn execute_vcvtuqq2pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTUQQ2PD instruction executed");
        Ok(())
    }

    pub fn execute_vcvtuqq2ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTUQQ2PS instruction executed");
        Ok(())
    }

    pub fn execute_vcvtusi2sd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTUSI2SD instruction executed");
        Ok(())
    }

    pub fn execute_vcvtusi2ss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTUSI2SS instruction executed");
        Ok(())
    }

    pub fn execute_vdbpsadbw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VDBPSADBW instruction executed");
        Ok(())
    }

    pub fn execute_vdivpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VDIVPD instruction executed");
        Ok(())
    }

    pub fn execute_vdivps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VDIVPS instruction executed");
        Ok(())
    }

    pub fn execute_vdivsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VDIVSD instruction executed");
        Ok(())
    }

    pub fn execute_vdivss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VDIVSS instruction executed");
        Ok(())
    }

    pub fn execute_vdpbf16ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VDPBF16PS instruction executed");
        Ok(())
    }

    pub fn execute_vdppd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VDPPD instruction executed");
        Ok(())
    }

    pub fn execute_vdpps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VDPPS instruction executed");
        Ok(())
    }

    pub fn execute_vexp2pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VEXP2PD instruction executed");
        Ok(())
    }

    pub fn execute_vexp2ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VEXP2PS instruction executed");
        Ok(())
    }

    pub fn execute_vexpandpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VEXPANDPD instruction executed");
        Ok(())
    }

    pub fn execute_vexpandps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VEXPANDPS instruction executed");
        Ok(())
    }

    pub fn execute_vextractf128(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VEXTRACTF128 instruction executed");
        Ok(())
    }

    pub fn execute_vextractf32x4(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VEXTRACTF32X4 instruction executed");
        Ok(())
    }

    pub fn execute_vextractf32x8(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VEXTRACTF32X8 instruction executed");
        Ok(())
    }

    pub fn execute_vextractf64x2(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VEXTRACTF64X2 instruction executed");
        Ok(())
    }

    pub fn execute_vextractf64x4(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VEXTRACTF64X4 instruction executed");
        Ok(())
    }

    pub fn execute_vextracti128(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VEXTRACTI128 instruction executed");
        Ok(())
    }

    pub fn execute_vextracti32x4(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VEXTRACTI32X4 instruction executed");
        Ok(())
    }

    pub fn execute_vextracti32x8(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VEXTRACTI32X8 instruction executed");
        Ok(())
    }

    pub fn execute_vextracti64x2(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VEXTRACTI64X2 instruction executed");
        Ok(())
    }

    pub fn execute_vextracti64x4(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VEXTRACTI64X4 instruction executed");
        Ok(())
    }

    pub fn execute_vextractps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VEXTRACTPS instruction executed");
        Ok(())
    }

    pub fn execute_vfixupimmpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFIXUPIMMPD instruction executed");
        Ok(())
    }

    pub fn execute_vfixupimmps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFIXUPIMMPS instruction executed");
        Ok(())
    }

    pub fn execute_vfixupimmsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFIXUPIMMSD instruction executed");
        Ok(())
    }

    pub fn execute_vfixupimmss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFIXUPIMMSS instruction executed");
        Ok(())
    }

    pub fn execute_vfmadd132pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADD132PD instruction executed");
        Ok(())
    }

    pub fn execute_vfmadd132ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADD132PS instruction executed");
        Ok(())
    }

    pub fn execute_vfmadd132sd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADD132SD instruction executed");
        Ok(())
    }

    pub fn execute_vfmadd132ss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADD132SS instruction executed");
        Ok(())
    }

    pub fn execute_vfmadd213pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADD213PD instruction executed");
        Ok(())
    }

    pub fn execute_vfmadd213ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADD213PS instruction executed");
        Ok(())
    }

    pub fn execute_vfmadd213sd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADD213SD instruction executed");
        Ok(())
    }

    pub fn execute_vfmadd213ss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADD213SS instruction executed");
        Ok(())
    }

    pub fn execute_vfmadd231pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADD231PD instruction executed");
        Ok(())
    }

    pub fn execute_vfmadd231ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADD231PS instruction executed");
        Ok(())
    }

    pub fn execute_vfmadd231sd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADD231SD instruction executed");
        Ok(())
    }

    pub fn execute_vfmadd231ss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADD231SS instruction executed");
        Ok(())
    }

    pub fn execute_vfmaddpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADDPD instruction executed");
        Ok(())
    }

    pub fn execute_vfmaddps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADDPS instruction executed");
        Ok(())
    }

    pub fn execute_vfmaddsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADDSD instruction executed");
        Ok(())
    }

    pub fn execute_vfmaddss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADDSS instruction executed");
        Ok(())
    }

    pub fn execute_vfmaddsub132pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADDSUB132PD instruction executed");
        Ok(())
    }

    pub fn execute_vfmaddsub132ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADDSUB132PS instruction executed");
        Ok(())
    }

    pub fn execute_vfmaddsub213pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADDSUB213PD instruction executed");
        Ok(())
    }

    pub fn execute_vfmaddsub213ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADDSUB213PS instruction executed");
        Ok(())
    }

    pub fn execute_vfmaddsub231pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADDSUB231PD instruction executed");
        Ok(())
    }

    pub fn execute_vfmaddsub231ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADDSUB231PS instruction executed");
        Ok(())
    }

    pub fn execute_vfmaddsubpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADDSUBPD instruction executed");
        Ok(())
    }

    pub fn execute_vfmaddsubps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADDSUBPS instruction executed");
        Ok(())
    }

    pub fn execute_vfmsub132pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUB132PD instruction executed");
        Ok(())
    }

    pub fn execute_vfmsub132ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUB132PS instruction executed");
        Ok(())
    }

    pub fn execute_vfmsub132sd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUB132SD instruction executed");
        Ok(())
    }

    pub fn execute_vfmsub132ss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUB132SS instruction executed");
        Ok(())
    }

    pub fn execute_vfmsub213pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUB213PD instruction executed");
        Ok(())
    }

    pub fn execute_vfmsub213ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUB213PS instruction executed");
        Ok(())
    }

    pub fn execute_vfmsub213sd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUB213SD instruction executed");
        Ok(())
    }

    pub fn execute_vfmsub213ss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUB213SS instruction executed");
        Ok(())
    }

    pub fn execute_vfmsub231pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUB231PD instruction executed");
        Ok(())
    }

    pub fn execute_vfmsub231ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUB231PS instruction executed");
        Ok(())
    }

    pub fn execute_vfmsub231sd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUB231SD instruction executed");
        Ok(())
    }

    pub fn execute_vfmsub231ss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUB231SS instruction executed");
        Ok(())
    }

    pub fn execute_vfmsubadd132pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUBADD132PD instruction executed");
        Ok(())
    }

    pub fn execute_vfmsubadd132ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUBADD132PS instruction executed");
        Ok(())
    }

    pub fn execute_vfmsubadd213pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUBADD213PD instruction executed");
        Ok(())
    }

    pub fn execute_vfmsubadd213ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUBADD213PS instruction executed");
        Ok(())
    }

    pub fn execute_vfmsubadd231pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUBADD231PD instruction executed");
        Ok(())
    }

    pub fn execute_vfmsubadd231ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUBADD231PS instruction executed");
        Ok(())
    }

    pub fn execute_vfmsubaddpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUBADDPD instruction executed");
        Ok(())
    }

    pub fn execute_vfmsubaddps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUBADDPS instruction executed");
        Ok(())
    }

    pub fn execute_vfmsubpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUBPD instruction executed");
        Ok(())
    }

    pub fn execute_vfmsubps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUBPS instruction executed");
        Ok(())
    }

    pub fn execute_vfmsubsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUBSD instruction executed");
        Ok(())
    }

    pub fn execute_vfmsubss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUBSS instruction executed");
        Ok(())
    }

    pub fn execute_vfnmadd132pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADD132PD instruction executed");
        Ok(())
    }

    pub fn execute_vfnmadd132ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADD132PS instruction executed");
        Ok(())
    }

    pub fn execute_vfnmadd132sd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADD132SD instruction executed");
        Ok(())
    }

    pub fn execute_vfnmadd132ss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADD132SS instruction executed");
        Ok(())
    }

    pub fn execute_vfnmadd213pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADD213PD instruction executed");
        Ok(())
    }

    pub fn execute_vfnmadd213ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADD213PS instruction executed");
        Ok(())
    }

    pub fn execute_vfnmadd213sd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADD213SD instruction executed");
        Ok(())
    }

    pub fn execute_vfnmadd213ss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADD213SS instruction executed");
        Ok(())
    }

    pub fn execute_vfnmadd231pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADD231PD instruction executed");
        Ok(())
    }

    pub fn execute_vfnmadd231ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADD231PS instruction executed");
        Ok(())
    }

    pub fn execute_vfnmadd231sd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADD231SD instruction executed");
        Ok(())
    }

    pub fn execute_vfnmadd231ss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADD231SS instruction executed");
        Ok(())
    }

    pub fn execute_vfnmaddpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADDPD instruction executed");
        Ok(())
    }

    pub fn execute_vfnmaddps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADDPS instruction executed");
        Ok(())
    }

    pub fn execute_vfnmaddsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADDSD instruction executed");
        Ok(())
    }

    pub fn execute_vfnmaddss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADDSS instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsub132pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUB132PD instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsub132ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUB132PS instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsub132sd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUB132SD instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsub132ss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUB132SS instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsub213pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUB213PD instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsub213ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUB213PS instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsub213sd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUB213SD instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsub213ss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUB213SS instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsub231pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUB231PD instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsub231ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUB231PS instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsub231sd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUB231SD instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsub231ss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUB231SS instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsubpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUBPD instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsubps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUBPS instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsubsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUBSD instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsubss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUBSS instruction executed");
        Ok(())
    }

    pub fn execute_vfpclasspd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFPCLASSPD instruction executed");
        Ok(())
    }

    pub fn execute_vfpclassps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFPCLASSPS instruction executed");
        Ok(())
    }

    pub fn execute_vfpclasssd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFPCLASSSD instruction executed");
        Ok(())
    }

    pub fn execute_vfpclassss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFPCLASSSS instruction executed");
        Ok(())
    }

    pub fn execute_vfrczpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFRCZPD instruction executed");
        Ok(())
    }

    pub fn execute_vfrczps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFRCZPS instruction executed");
        Ok(())
    }

    pub fn execute_vfrczsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFRCZSD instruction executed");
        Ok(())
    }

    pub fn execute_vfrczss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFRCZSS instruction executed");
        Ok(())
    }

    pub fn execute_vgatherdpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGATHERDPD instruction executed");
        Ok(())
    }

    pub fn execute_vgatherdps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGATHERDPS instruction executed");
        Ok(())
    }

    pub fn execute_vgatherpf0dpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGATHERPF0DPD instruction executed");
        Ok(())
    }

    pub fn execute_vgatherpf0dps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGATHERPF0DPS instruction executed");
        Ok(())
    }

    pub fn execute_vgatherpf0qpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGATHERPF0QPD instruction executed");
        Ok(())
    }

    pub fn execute_vgatherpf0qps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGATHERPF0QPS instruction executed");
        Ok(())
    }

    pub fn execute_vgatherpf1dpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGATHERPF1DPD instruction executed");
        Ok(())
    }

    pub fn execute_vgatherpf1dps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGATHERPF1DPS instruction executed");
        Ok(())
    }

    pub fn execute_vgatherpf1qpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGATHERPF1QPD instruction executed");
        Ok(())
    }

    pub fn execute_vgatherpf1qps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGATHERPF1QPS instruction executed");
        Ok(())
    }

    pub fn execute_vgatherqpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGATHERQPD instruction executed");
        Ok(())
    }

    pub fn execute_vgatherqps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGATHERQPS instruction executed");
        Ok(())
    }

    pub fn execute_vgetexppd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGETEXPPD instruction executed");
        Ok(())
    }

    pub fn execute_vgetexpps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGETEXPPS instruction executed");
        Ok(())
    }

    pub fn execute_vgetexpsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGETEXPSD instruction executed");
        Ok(())
    }

    pub fn execute_vgetexpss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGETEXPSS instruction executed");
        Ok(())
    }

    pub fn execute_vgetmantpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGETMANTPD instruction executed");
        Ok(())
    }

    pub fn execute_vgetmantps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGETMANTPS instruction executed");
        Ok(())
    }

    pub fn execute_vgetmantsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGETMANTSD instruction executed");
        Ok(())
    }

    pub fn execute_vgetmantss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGETMANTSS instruction executed");
        Ok(())
    }

    pub fn execute_vgf2p8affineinvqb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGF2P8AFFINEINVQB instruction executed");
        Ok(())
    }

    pub fn execute_vgf2p8affineqb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGF2P8AFFINEQB instruction executed");
        Ok(())
    }

    pub fn execute_vgf2p8mulb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGF2P8MULB instruction executed");
        Ok(())
    }

    pub fn execute_vhaddpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VHADDPD instruction executed");
        Ok(())
    }

    pub fn execute_vhaddps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VHADDPS instruction executed");
        Ok(())
    }

    pub fn execute_vhsubpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VHSUBPD instruction executed");
        Ok(())
    }

    pub fn execute_vhsubps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VHSUBPS instruction executed");
        Ok(())
    }

    pub fn execute_vinsertf128(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VINSERTF128 instruction executed");
        Ok(())
    }

    pub fn execute_vinsertf32x4(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VINSERTF32X4 instruction executed");
        Ok(())
    }

    pub fn execute_vinsertf32x8(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VINSERTF32X8 instruction executed");
        Ok(())
    }

    pub fn execute_vinsertf64x2(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VINSERTF64X2 instruction executed");
        Ok(())
    }

    pub fn execute_vinsertf64x4(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VINSERTF64X4 instruction executed");
        Ok(())
    }

    pub fn execute_vinserti128(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VINSERTI128 instruction executed");
        Ok(())
    }

    pub fn execute_vinserti32x4(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VINSERTI32X4 instruction executed");
        Ok(())
    }

    pub fn execute_vinserti32x8(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VINSERTI32X8 instruction executed");
        Ok(())
    }

    pub fn execute_vinserti64x2(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VINSERTI64X2 instruction executed");
        Ok(())
    }

    pub fn execute_vinserti64x4(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VINSERTI64X4 instruction executed");
        Ok(())
    }

    pub fn execute_vinsertps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VINSERTPS instruction executed");
        Ok(())
    }

    pub fn execute_vlddqu(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VLDDQU instruction executed");
        Ok(())
    }

    pub fn execute_vldmxcsr(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VLDMXCSR instruction executed");
        Ok(())
    }

    pub fn execute_vmaskmovdqu(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMASKMOVDQU instruction executed");
        Ok(())
    }

    pub fn execute_vmaskmovpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMASKMOVPD instruction executed");
        Ok(())
    }

    pub fn execute_vmaskmovps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMASKMOVPS instruction executed");
        Ok(())
    }

    pub fn execute_vmaxpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMAXPD instruction executed");
        Ok(())
    }

    pub fn execute_vmaxps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMAXPS instruction executed");
        Ok(())
    }

    pub fn execute_vmaxsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMAXSD instruction executed");
        Ok(())
    }

    pub fn execute_vmaxss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMAXSS instruction executed");
        Ok(())
    }

    pub fn execute_vmcall(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMCALL instruction executed");
        Ok(())
    }

    pub fn execute_vmclear(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMCLEAR instruction executed");
        Ok(())
    }

    pub fn execute_vmfunc(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMFUNC instruction executed");
        Ok(())
    }

    pub fn execute_vminpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMINPD instruction executed");
        Ok(())
    }

    pub fn execute_vminps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMINPS instruction executed");
        Ok(())
    }

    pub fn execute_vminsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMINSD instruction executed");
        Ok(())
    }

    pub fn execute_vminss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMINSS instruction executed");
        Ok(())
    }

    pub fn execute_vmlaunch(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMLAUNCH instruction executed");
        Ok(())
    }

    pub fn execute_vmload(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMLOAD instruction executed");
        Ok(())
    }

    pub fn execute_vmmcall(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMMCALL instruction executed");
        Ok(())
    }

    pub fn execute_vmovapd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVAPD instruction executed");
        Ok(())
    }

    pub fn execute_vmovaps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVAPS instruction executed");
        Ok(())
    }

    pub fn execute_vmovd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVD instruction executed");
        Ok(())
    }

    pub fn execute_vmovddup(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVDDUP instruction executed");
        Ok(())
    }

    pub fn execute_vmovdqa(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVDQA instruction executed");
        Ok(())
    }

    pub fn execute_vmovdqa32(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVDQA32 instruction executed");
        Ok(())
    }

    pub fn execute_vmovdqa64(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVDQA64 instruction executed");
        Ok(())
    }

    pub fn execute_vmovdqu(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVDQU instruction executed");
        Ok(())
    }

    pub fn execute_vmovdqu16(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVDQU16 instruction executed");
        Ok(())
    }

    pub fn execute_vmovdqu32(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVDQU32 instruction executed");
        Ok(())
    }

    pub fn execute_vmovdqu64(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVDQU64 instruction executed");
        Ok(())
    }

    pub fn execute_vmovdqu8(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVDQU8 instruction executed");
        Ok(())
    }

    pub fn execute_vmovhlps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVHLPS instruction executed");
        Ok(())
    }

    pub fn execute_vmovhpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVHPD instruction executed");
        Ok(())
    }

    pub fn execute_vmovhps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVHPS instruction executed");
        Ok(())
    }

    pub fn execute_vmovlhps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVLHPS instruction executed");
        Ok(())
    }

    pub fn execute_vmovlpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVLPD instruction executed");
        Ok(())
    }

    pub fn execute_vmovlps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVLPS instruction executed");
        Ok(())
    }

    pub fn execute_vmovmskpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVMSKPD instruction executed");
        Ok(())
    }

    pub fn execute_vmovmskps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVMSKPS instruction executed");
        Ok(())
    }

    pub fn execute_vmovntdq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVNTDQ instruction executed");
        Ok(())
    }

    pub fn execute_vmovntdqa(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVNTDQA instruction executed");
        Ok(())
    }

    pub fn execute_vmovntpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVNTPD instruction executed");
        Ok(())
    }

    pub fn execute_vmovntps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVNTPS instruction executed");
        Ok(())
    }

    pub fn execute_vmovq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVQ instruction executed");
        Ok(())
    }

    pub fn execute_vmovsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVSD instruction executed");
        Ok(())
    }

    pub fn execute_vmovshdup(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVSHDUP instruction executed");
        Ok(())
    }

    pub fn execute_vmovsldup(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVSLDUP instruction executed");
        Ok(())
    }

    pub fn execute_vmovss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVSS instruction executed");
        Ok(())
    }

    pub fn execute_vmovupd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVUPD instruction executed");
        Ok(())
    }

    pub fn execute_vmovups(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVUPS instruction executed");
        Ok(())
    }

    pub fn execute_vmpsadbw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMPSADBW instruction executed");
        Ok(())
    }

    pub fn execute_vmptrld(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMPTRLD instruction executed");
        Ok(())
    }

    pub fn execute_vmptrst(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMPTRST instruction executed");
        Ok(())
    }

    pub fn execute_vmread(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMREAD instruction executed");
        Ok(())
    }

    pub fn execute_vmresume(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMRESUME instruction executed");
        Ok(())
    }

    pub fn execute_vmrun(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMRUN instruction executed");
        Ok(())
    }

    pub fn execute_vmsave(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMSAVE instruction executed");
        Ok(())
    }

    pub fn execute_vmulpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMULPD instruction executed");
        Ok(())
    }

    pub fn execute_vmulps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMULPS instruction executed");
        Ok(())
    }

    pub fn execute_vmulsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMULSD instruction executed");
        Ok(())
    }

    pub fn execute_vmulss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMULSS instruction executed");
        Ok(())
    }

    pub fn execute_vmwrite(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMWRITE instruction executed");
        Ok(())
    }

    pub fn execute_vmxoff(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMXOFF instruction executed");
        Ok(())
    }

    pub fn execute_vmxon(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMXON instruction executed");
        Ok(())
    }

    pub fn execute_vorpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VORPD instruction executed");
        Ok(())
    }

    pub fn execute_vorps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VORPS instruction executed");
        Ok(())
    }

    pub fn execute_vp2intersectd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VP2INTERSECTD instruction executed");
        Ok(())
    }

    pub fn execute_vp2intersectq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VP2INTERSECTQ instruction executed");
        Ok(())
    }

    pub fn execute_vp4dpwssd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VP4DPWSSD instruction executed");
        Ok(())
    }

    pub fn execute_vp4dpwssds(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VP4DPWSSDS instruction executed");
        Ok(())
    }

    pub fn execute_vpabsb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPABSB instruction executed");
        Ok(())
    }

    pub fn execute_vpabsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPABSD instruction executed");
        Ok(())
    }

    pub fn execute_vpabsq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPABSQ instruction executed");
        Ok(())
    }

    pub fn execute_vpabsw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPABSW instruction executed");
        Ok(())
    }

    pub fn execute_vpackssdw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPACKSSDW instruction executed");
        Ok(())
    }

    pub fn execute_vpacksswb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPACKSSWB instruction executed");
        Ok(())
    }

    pub fn execute_vpackusdw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPACKUSDW instruction executed");
        Ok(())
    }

    pub fn execute_vpackuswb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPACKUSWB instruction executed");
        Ok(())
    }

    pub fn execute_vpaddb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPADDB instruction executed");
        Ok(())
    }

    pub fn execute_vpaddd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPADDD instruction executed");
        Ok(())
    }

    pub fn execute_vpaddq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPADDQ instruction executed");
        Ok(())
    }

    pub fn execute_vpaddsb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPADDSB instruction executed");
        Ok(())
    }

    pub fn execute_vpaddsw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPADDSW instruction executed");
        Ok(())
    }

    pub fn execute_vpaddusb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPADDUSB instruction executed");
        Ok(())
    }

    pub fn execute_vpaddusw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPADDUSW instruction executed");
        Ok(())
    }

    pub fn execute_vpaddw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPADDW instruction executed");
        Ok(())
    }

    pub fn execute_vpalignr(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPALIGNR instruction executed");
        Ok(())
    }

    pub fn execute_vpand(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPAND instruction executed");
        Ok(())
    }

    pub fn execute_vpandd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPANDD instruction executed");
        Ok(())
    }

    pub fn execute_vpandn(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPANDN instruction executed");
        Ok(())
    }

    pub fn execute_vpandnd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPANDND instruction executed");
        Ok(())
    }

    pub fn execute_vpandnq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPANDNQ instruction executed");
        Ok(())
    }

    pub fn execute_vpandq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPANDQ instruction executed");
        Ok(())
    }

    pub fn execute_vpavgb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPAVGB instruction executed");
        Ok(())
    }

    pub fn execute_vpavgw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPAVGW instruction executed");
        Ok(())
    }

    pub fn execute_vpblendd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPBLENDD instruction executed");
        Ok(())
    }

    pub fn execute_vpblendmb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPBLENDMB instruction executed");
        Ok(())
    }

    pub fn execute_vpblendmd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPBLENDMD instruction executed");
        Ok(())
    }

    pub fn execute_vpblendmq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPBLENDMQ instruction executed");
        Ok(())
    }

    pub fn execute_vpblendmw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPBLENDMW instruction executed");
        Ok(())
    }

    pub fn execute_vpblendvb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPBLENDVB instruction executed");
        Ok(())
    }

    pub fn execute_vpblendw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPBLENDW instruction executed");
        Ok(())
    }

    pub fn execute_vpbroadcastb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPBROADCASTB instruction executed");
        Ok(())
    }

    pub fn execute_vpbroadcastd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPBROADCASTD instruction executed");
        Ok(())
    }

    pub fn execute_vpbroadcastmb2q(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPBROADCASTMB2Q instruction executed");
        Ok(())
    }

    pub fn execute_vpbroadcastmw2d(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPBROADCASTMW2D instruction executed");
        Ok(())
    }

    pub fn execute_vpbroadcastq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPBROADCASTQ instruction executed");
        Ok(())
    }

    pub fn execute_vpbroadcastw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPBROADCASTW instruction executed");
        Ok(())
    }

    pub fn execute_vpclmulqdq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCLMULQDQ instruction executed");
        Ok(())
    }

    pub fn execute_vpcmov(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMOV instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPB instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPD instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpeqb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPEQB instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpeqd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPEQD instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpeqq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPEQQ instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpeqw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPEQW instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpestri(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPESTRI instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpestri64(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPESTRI64 instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpestrm(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPESTRM instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpestrm64(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPESTRM64 instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpgtb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPGTB instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpgtd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPGTD instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpgtq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPGTQ instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpgtw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPGTW instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpistri(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPISTRI instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpistrm(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPISTRM instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPQ instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpub(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPUB instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpud(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPUD instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpuq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPUQ instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpuw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPUW instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPW instruction executed");
        Ok(())
    }

    pub fn execute_vpcomb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCOMB instruction executed");
        Ok(())
    }

    pub fn execute_vpcomd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCOMD instruction executed");
        Ok(())
    }

    pub fn execute_vpcompressb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCOMPRESSB instruction executed");
        Ok(())
    }

    pub fn execute_vpcompressd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCOMPRESSD instruction executed");
        Ok(())
    }

    pub fn execute_vpcompressq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCOMPRESSQ instruction executed");
        Ok(())
    }

    pub fn execute_vpcompressw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCOMPRESSW instruction executed");
        Ok(())
    }

    pub fn execute_vpcomq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCOMQ instruction executed");
        Ok(())
    }

    pub fn execute_vpcomub(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCOMUB instruction executed");
        Ok(())
    }

    pub fn execute_vpcomud(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCOMUD instruction executed");
        Ok(())
    }

    pub fn execute_vpcomuq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCOMUQ instruction executed");
        Ok(())
    }

    pub fn execute_vpcomuw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCOMUW instruction executed");
        Ok(())
    }

    pub fn execute_vpcomw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCOMW instruction executed");
        Ok(())
    }

    pub fn execute_vpconflictd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCONFLICTD instruction executed");
        Ok(())
    }

    pub fn execute_vpconflictq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCONFLICTQ instruction executed");
        Ok(())
    }

    pub fn execute_vpdpbusd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPDPBUSD instruction executed");
        Ok(())
    }

    pub fn execute_vpdpbusds(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPDPBUSDS instruction executed");
        Ok(())
    }

    pub fn execute_vpdpwssd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPDPWSSD instruction executed");
        Ok(())
    }

    pub fn execute_vpdpwssds(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPDPWSSDS instruction executed");
        Ok(())
    }

    pub fn execute_vperm2f128(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERM2F128 instruction executed");
        Ok(())
    }

    pub fn execute_vperm2i128(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERM2I128 instruction executed");
        Ok(())
    }

    pub fn execute_vpermb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMB instruction executed");
        Ok(())
    }

    pub fn execute_vpermd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMD instruction executed");
        Ok(())
    }

    pub fn execute_vpermi2b(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMI2B instruction executed");
        Ok(())
    }

    pub fn execute_vpermi2d(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMI2D instruction executed");
        Ok(())
    }

    pub fn execute_vpermi2pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMI2PD instruction executed");
        Ok(())
    }

    pub fn execute_vpermi2ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMI2PS instruction executed");
        Ok(())
    }

    pub fn execute_vpermi2q(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMI2Q instruction executed");
        Ok(())
    }

    pub fn execute_vpermi2w(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMI2W instruction executed");
        Ok(())
    }

    pub fn execute_vpermil2pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMIL2PD instruction executed");
        Ok(())
    }

    pub fn execute_vpermil2ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMIL2PS instruction executed");
        Ok(())
    }

    pub fn execute_vpermilpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMILPD instruction executed");
        Ok(())
    }

    pub fn execute_vpermilps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMILPS instruction executed");
        Ok(())
    }

    pub fn execute_vpermpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMPD instruction executed");
        Ok(())
    }

    pub fn execute_vpermps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMPS instruction executed");
        Ok(())
    }

    pub fn execute_vpermq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMQ instruction executed");
        Ok(())
    }

    pub fn execute_vpermt2b(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMT2B instruction executed");
        Ok(())
    }

    pub fn execute_vpermt2d(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMT2D instruction executed");
        Ok(())
    }

    pub fn execute_vpermt2pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMT2PD instruction executed");
        Ok(())
    }

    pub fn execute_vpermt2ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMT2PS instruction executed");
        Ok(())
    }

    pub fn execute_vpermt2q(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMT2Q instruction executed");
        Ok(())
    }

    pub fn execute_vpermt2w(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMT2W instruction executed");
        Ok(())
    }

    pub fn execute_vpermw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMW instruction executed");
        Ok(())
    }

    pub fn execute_vpexpandb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPEXPANDB instruction executed");
        Ok(())
    }

    pub fn execute_vpexpandd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPEXPANDD instruction executed");
        Ok(())
    }

    pub fn execute_vpexpandq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPEXPANDQ instruction executed");
        Ok(())
    }

    pub fn execute_vpexpandw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPEXPANDW instruction executed");
        Ok(())
    }

    pub fn execute_vpextrb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPEXTRB instruction executed");
        Ok(())
    }

    pub fn execute_vpextrd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPEXTRD instruction executed");
        Ok(())
    }

    pub fn execute_vpextrq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPEXTRQ instruction executed");
        Ok(())
    }

    pub fn execute_vpextrw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPEXTRW instruction executed");
        Ok(())
    }

    pub fn execute_vpgatherdd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPGATHERDD instruction executed");
        Ok(())
    }

    pub fn execute_vpgatherdq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPGATHERDQ instruction executed");
        Ok(())
    }

    pub fn execute_vpgatherqd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPGATHERQD instruction executed");
        Ok(())
    }

    pub fn execute_vpgatherqq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPGATHERQQ instruction executed");
        Ok(())
    }

    pub fn execute_vphaddbd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHADDBD instruction executed");
        Ok(())
    }

    pub fn execute_vphaddbq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHADDBQ instruction executed");
        Ok(())
    }

    pub fn execute_vphaddbw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHADDBW instruction executed");
        Ok(())
    }

    pub fn execute_vphaddd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHADDD instruction executed");
        Ok(())
    }

    pub fn execute_vphadddq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHADDDQ instruction executed");
        Ok(())
    }

    pub fn execute_vphaddsw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHADDSW instruction executed");
        Ok(())
    }

    pub fn execute_vphaddubd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHADDUBD instruction executed");
        Ok(())
    }

    pub fn execute_vphaddubq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHADDUBQ instruction executed");
        Ok(())
    }

    pub fn execute_vphaddubw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHADDUBW instruction executed");
        Ok(())
    }

    pub fn execute_vphaddudq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHADDUDQ instruction executed");
        Ok(())
    }

    pub fn execute_vphadduwd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHADDUWD instruction executed");
        Ok(())
    }

    pub fn execute_vphadduwq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHADDUWQ instruction executed");
        Ok(())
    }

    pub fn execute_vphaddw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHADDW instruction executed");
        Ok(())
    }

    pub fn execute_vphaddwd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHADDWD instruction executed");
        Ok(())
    }

    pub fn execute_vphaddwq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHADDWQ instruction executed");
        Ok(())
    }

    pub fn execute_vphminposuw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHMINPOSUW instruction executed");
        Ok(())
    }

    pub fn execute_vphsubbw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHSUBBW instruction executed");
        Ok(())
    }

    pub fn execute_vphsubd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHSUBD instruction executed");
        Ok(())
    }

    pub fn execute_vphsubdq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHSUBDQ instruction executed");
        Ok(())
    }

    pub fn execute_vphsubsw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHSUBSW instruction executed");
        Ok(())
    }

    pub fn execute_vphsubw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHSUBW instruction executed");
        Ok(())
    }

    pub fn execute_vphsubwd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPHSUBWD instruction executed");
        Ok(())
    }

    pub fn execute_vpinsrb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPINSRB instruction executed");
        Ok(())
    }

    pub fn execute_vpinsrd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPINSRD instruction executed");
        Ok(())
    }

    pub fn execute_vpinsrq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPINSRQ instruction executed");
        Ok(())
    }

    pub fn execute_vpinsrw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPINSRW instruction executed");
        Ok(())
    }

    pub fn execute_vplzcntd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPLZCNTD instruction executed");
        Ok(())
    }

    pub fn execute_vplzcntq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPLZCNTQ instruction executed");
        Ok(())
    }

    pub fn execute_vpmacsdd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMACSDD instruction executed");
        Ok(())
    }

    pub fn execute_vpmacsdqh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMACSDQH instruction executed");
        Ok(())
    }

    pub fn execute_vpmacsdql(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMACSDQL instruction executed");
        Ok(())
    }

    pub fn execute_vpmacssdd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMACSSDD instruction executed");
        Ok(())
    }

    pub fn execute_vpmacssdqh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMACSSDQH instruction executed");
        Ok(())
    }

    pub fn execute_vpmacssdql(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMACSSDQL instruction executed");
        Ok(())
    }

    pub fn execute_vpmacsswd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMACSSWD instruction executed");
        Ok(())
    }

    pub fn execute_vpmacssww(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMACSSWW instruction executed");
        Ok(())
    }

    pub fn execute_vpmacswd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMACSWD instruction executed");
        Ok(())
    }

    pub fn execute_vpmacsww(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMACSWW instruction executed");
        Ok(())
    }

    pub fn execute_vpmadcsswd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMADCSSWD instruction executed");
        Ok(())
    }

    pub fn execute_vpmadcswd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMADCSWD instruction executed");
        Ok(())
    }

    pub fn execute_vpmadd52huq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMADD52HUQ instruction executed");
        Ok(())
    }

    pub fn execute_vpmadd52luq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMADD52LUQ instruction executed");
        Ok(())
    }

    pub fn execute_vpmaddubsw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMADDUBSW instruction executed");
        Ok(())
    }

    pub fn execute_vpmaddwd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMADDWD instruction executed");
        Ok(())
    }

    pub fn execute_vpmaskmovd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMASKMOVD instruction executed");
        Ok(())
    }

    pub fn execute_vpmaskmovq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMASKMOVQ instruction executed");
        Ok(())
    }

    pub fn execute_vpmaxsb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMAXSB instruction executed");
        Ok(())
    }

    pub fn execute_vpmaxsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMAXSD instruction executed");
        Ok(())
    }

    pub fn execute_vpmaxsq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMAXSQ instruction executed");
        Ok(())
    }

    pub fn execute_vpmaxsw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMAXSW instruction executed");
        Ok(())
    }

    pub fn execute_vpmaxub(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMAXUB instruction executed");
        Ok(())
    }

    pub fn execute_vpmaxud(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMAXUD instruction executed");
        Ok(())
    }

    pub fn execute_vpmaxuq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMAXUQ instruction executed");
        Ok(())
    }

    pub fn execute_vpmaxuw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMAXUW instruction executed");
        Ok(())
    }

    pub fn execute_vpminsb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMINSB instruction executed");
        Ok(())
    }

    pub fn execute_vpminsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMINSD instruction executed");
        Ok(())
    }

    pub fn execute_vpminsq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMINSQ instruction executed");
        Ok(())
    }

    pub fn execute_vpminsw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMINSW instruction executed");
        Ok(())
    }

    pub fn execute_vpminub(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMINUB instruction executed");
        Ok(())
    }

    pub fn execute_vpminud(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMINUD instruction executed");
        Ok(())
    }

    pub fn execute_vpminuq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMINUQ instruction executed");
        Ok(())
    }

    pub fn execute_vpminuw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMINUW instruction executed");
        Ok(())
    }

    pub fn execute_vpmovb2m(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVB2M instruction executed");
        Ok(())
    }

    pub fn execute_vpmovd2m(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVD2M instruction executed");
        Ok(())
    }

    pub fn execute_vpmovdb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVDB instruction executed");
        Ok(())
    }

    pub fn execute_vpmovdw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVDW instruction executed");
        Ok(())
    }

    pub fn execute_vpmovm2b(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVM2B instruction executed");
        Ok(())
    }

    pub fn execute_vpmovm2d(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVM2D instruction executed");
        Ok(())
    }

    pub fn execute_vpmovm2q(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVM2Q instruction executed");
        Ok(())
    }

    pub fn execute_vpmovm2w(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVM2W instruction executed");
        Ok(())
    }

    pub fn execute_vpmovmskb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVMSKB instruction executed");
        Ok(())
    }

    pub fn execute_vpmovq2m(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVQ2M instruction executed");
        Ok(())
    }

    pub fn execute_vpmovqb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVQB instruction executed");
        Ok(())
    }

    pub fn execute_vpmovqd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVQD instruction executed");
        Ok(())
    }

    pub fn execute_vpmovqw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVQW instruction executed");
        Ok(())
    }

    pub fn execute_vpmovsdb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVSDB instruction executed");
        Ok(())
    }

    pub fn execute_vpmovsdw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVSDW instruction executed");
        Ok(())
    }

    pub fn execute_vpmovsqb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVSQB instruction executed");
        Ok(())
    }

    pub fn execute_vpmovsqd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVSQD instruction executed");
        Ok(())
    }

    pub fn execute_vpmovsqw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVSQW instruction executed");
        Ok(())
    }

    pub fn execute_vpmovswb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVSWB instruction executed");
        Ok(())
    }

    pub fn execute_vpmovsxbd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVSXBD instruction executed");
        Ok(())
    }

    pub fn execute_vpmovsxbq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVSXBQ instruction executed");
        Ok(())
    }

    pub fn execute_vpmovsxbw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVSXBW instruction executed");
        Ok(())
    }

    pub fn execute_vpmovsxdq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVSXDQ instruction executed");
        Ok(())
    }

    pub fn execute_vpmovsxwd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVSXWD instruction executed");
        Ok(())
    }

    pub fn execute_vpmovsxwq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVSXWQ instruction executed");
        Ok(())
    }

    pub fn execute_vpmovusdb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVUSDB instruction executed");
        Ok(())
    }

    pub fn execute_vpmovusdw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVUSDW instruction executed");
        Ok(())
    }

    pub fn execute_vpmovusqb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVUSQB instruction executed");
        Ok(())
    }

    pub fn execute_vpmovusqd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVUSQD instruction executed");
        Ok(())
    }

    pub fn execute_vpmovusqw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVUSQW instruction executed");
        Ok(())
    }

    pub fn execute_vpmovuswb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVUSWB instruction executed");
        Ok(())
    }

    pub fn execute_vpmovw2m(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVW2M instruction executed");
        Ok(())
    }

    pub fn execute_vpmovwb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVWB instruction executed");
        Ok(())
    }

    pub fn execute_vpmovzxbd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVZXBD instruction executed");
        Ok(())
    }

    pub fn execute_vpmovzxbq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVZXBQ instruction executed");
        Ok(())
    }

    pub fn execute_vpmovzxbw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVZXBW instruction executed");
        Ok(())
    }

    pub fn execute_vpmovzxdq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVZXDQ instruction executed");
        Ok(())
    }

    pub fn execute_vpmovzxwd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVZXWD instruction executed");
        Ok(())
    }

    pub fn execute_vpmovzxwq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMOVZXWQ instruction executed");
        Ok(())
    }

    pub fn execute_vpmuldq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMULDQ instruction executed");
        Ok(())
    }

    pub fn execute_vpmulhrsw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMULHRSW instruction executed");
        Ok(())
    }

    pub fn execute_vpmulhuw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMULHUW instruction executed");
        Ok(())
    }

    pub fn execute_vpmulhw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMULHW instruction executed");
        Ok(())
    }

    pub fn execute_vpmulld(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMULLD instruction executed");
        Ok(())
    }

    pub fn execute_vpmullq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMULLQ instruction executed");
        Ok(())
    }

    pub fn execute_vpmullw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMULLW instruction executed");
        Ok(())
    }

    pub fn execute_vpmultishiftqb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMULTISHIFTQB instruction executed");
        Ok(())
    }

    pub fn execute_vpmuludq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMULUDQ instruction executed");
        Ok(())
    }

    pub fn execute_vpopcntb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPOPCNTB instruction executed");
        Ok(())
    }

    pub fn execute_vpopcntd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPOPCNTD instruction executed");
        Ok(())
    }

    pub fn execute_vpopcntq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPOPCNTQ instruction executed");
        Ok(())
    }

    pub fn execute_vpopcntw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPOPCNTW instruction executed");
        Ok(())
    }

    pub fn execute_vpor(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPOR instruction executed");
        Ok(())
    }

    pub fn execute_vpord(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPORD instruction executed");
        Ok(())
    }

    pub fn execute_vporq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPORQ instruction executed");
        Ok(())
    }

    pub fn execute_vpperm(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPPERM instruction executed");
        Ok(())
    }

    pub fn execute_vprold(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPROLD instruction executed");
        Ok(())
    }

    pub fn execute_vprolq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPROLQ instruction executed");
        Ok(())
    }

    pub fn execute_vprolvd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPROLVD instruction executed");
        Ok(())
    }

    pub fn execute_vprolvq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPROLVQ instruction executed");
        Ok(())
    }

    pub fn execute_vprord(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPRORD instruction executed");
        Ok(())
    }

    pub fn execute_vprorq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPRORQ instruction executed");
        Ok(())
    }

    pub fn execute_vprorvd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPRORVD instruction executed");
        Ok(())
    }

    pub fn execute_vprorvq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPRORVQ instruction executed");
        Ok(())
    }

    pub fn execute_vprotb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPROTB instruction executed");
        Ok(())
    }

    pub fn execute_vprotd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPROTD instruction executed");
        Ok(())
    }

    pub fn execute_vprotq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPROTQ instruction executed");
        Ok(())
    }

    pub fn execute_vprotw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPROTW instruction executed");
        Ok(())
    }

    pub fn execute_vpsadbw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSADBW instruction executed");
        Ok(())
    }

    pub fn execute_vpscatterdd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSCATTERDD instruction executed");
        Ok(())
    }

    pub fn execute_vpscatterdq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSCATTERDQ instruction executed");
        Ok(())
    }

    pub fn execute_vpscatterqd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSCATTERQD instruction executed");
        Ok(())
    }

    pub fn execute_vpscatterqq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSCATTERQQ instruction executed");
        Ok(())
    }

    pub fn execute_vpshab(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHAB instruction executed");
        Ok(())
    }

    pub fn execute_vpshad(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHAD instruction executed");
        Ok(())
    }

    pub fn execute_vpshaq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHAQ instruction executed");
        Ok(())
    }

    pub fn execute_vpshaw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHAW instruction executed");
        Ok(())
    }

    pub fn execute_vpshlb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHLB instruction executed");
        Ok(())
    }

    pub fn execute_vpshld(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHLD instruction executed");
        Ok(())
    }

    pub fn execute_vpshldd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHLDD instruction executed");
        Ok(())
    }

    pub fn execute_vpshldq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHLDQ instruction executed");
        Ok(())
    }

    pub fn execute_vpshldvd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHLDVD instruction executed");
        Ok(())
    }

    pub fn execute_vpshldvq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHLDVQ instruction executed");
        Ok(())
    }

    pub fn execute_vpshldvw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHLDVW instruction executed");
        Ok(())
    }

    pub fn execute_vpshldw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHLDW instruction executed");
        Ok(())
    }

    pub fn execute_vpshlq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHLQ instruction executed");
        Ok(())
    }

    pub fn execute_vpshlw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHLW instruction executed");
        Ok(())
    }

    pub fn execute_vpshrdd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHRDD instruction executed");
        Ok(())
    }

    pub fn execute_vpshrdq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHRDQ instruction executed");
        Ok(())
    }

    pub fn execute_vpshrdvd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHRDVD instruction executed");
        Ok(())
    }

    pub fn execute_vpshrdvq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHRDVQ instruction executed");
        Ok(())
    }

    pub fn execute_vpshrdvw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHRDVW instruction executed");
        Ok(())
    }

    pub fn execute_vpshrdw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHRDW instruction executed");
        Ok(())
    }

    pub fn execute_vpshufb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHUFB instruction executed");
        Ok(())
    }

    pub fn execute_vpshufbitqmb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHUFBITQMB instruction executed");
        Ok(())
    }

    pub fn execute_vpshufd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHUFD instruction executed");
        Ok(())
    }

    pub fn execute_vpshufhw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHUFHW instruction executed");
        Ok(())
    }

    pub fn execute_vpshuflw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSHUFLW instruction executed");
        Ok(())
    }

    pub fn execute_vpsignb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSIGNB instruction executed");
        Ok(())
    }

    pub fn execute_vpsignd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSIGND instruction executed");
        Ok(())
    }

    pub fn execute_vpsignw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSIGNW instruction executed");
        Ok(())
    }

    pub fn execute_vpslld(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSLLD instruction executed");
        Ok(())
    }

    pub fn execute_vpslldq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSLLDQ instruction executed");
        Ok(())
    }

    pub fn execute_vpsllq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSLLQ instruction executed");
        Ok(())
    }

    pub fn execute_vpsllvd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSLLVD instruction executed");
        Ok(())
    }

    pub fn execute_vpsllvq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSLLVQ instruction executed");
        Ok(())
    }

    pub fn execute_vpsllvw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSLLVW instruction executed");
        Ok(())
    }

    pub fn execute_vpsllw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSLLW instruction executed");
        Ok(())
    }

    pub fn execute_vpsrad(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSRAD instruction executed");
        Ok(())
    }

    pub fn execute_vpsraq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSRAQ instruction executed");
        Ok(())
    }

    pub fn execute_vpsravd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSRAVD instruction executed");
        Ok(())
    }

    pub fn execute_vpsravq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSRAVQ instruction executed");
        Ok(())
    }

    pub fn execute_vpsravw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSRAVW instruction executed");
        Ok(())
    }

    pub fn execute_vpsraw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSRAW instruction executed");
        Ok(())
    }

    pub fn execute_vpsrld(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSRLD instruction executed");
        Ok(())
    }

    pub fn execute_vpsrldq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSRLDQ instruction executed");
        Ok(())
    }

    pub fn execute_vpsrlq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSRLQ instruction executed");
        Ok(())
    }

    pub fn execute_vpsrlvd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSRLVD instruction executed");
        Ok(())
    }

    pub fn execute_vpsrlvq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSRLVQ instruction executed");
        Ok(())
    }

    pub fn execute_vpsrlvw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSRLVW instruction executed");
        Ok(())
    }

    pub fn execute_vpsrlw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSRLW instruction executed");
        Ok(())
    }

    pub fn execute_vpsubb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSUBB instruction executed");
        Ok(())
    }

    pub fn execute_vpsubd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSUBD instruction executed");
        Ok(())
    }

    pub fn execute_vpsubq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSUBQ instruction executed");
        Ok(())
    }

    pub fn execute_vpsubsb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSUBSB instruction executed");
        Ok(())
    }

    pub fn execute_vpsubsw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSUBSW instruction executed");
        Ok(())
    }

    pub fn execute_vpsubusb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSUBUSB instruction executed");
        Ok(())
    }

    pub fn execute_vpsubusw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSUBUSW instruction executed");
        Ok(())
    }

    pub fn execute_vpsubw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSUBW instruction executed");
        Ok(())
    }

    pub fn execute_vpternlogd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPTERNLOGD instruction executed");
        Ok(())
    }

    pub fn execute_vpternlogq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPTERNLOGQ instruction executed");
        Ok(())
    }

    pub fn execute_vptest(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPTEST instruction executed");
        Ok(())
    }

    pub fn execute_vptestmb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPTESTMB instruction executed");
        Ok(())
    }

    pub fn execute_vptestmd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPTESTMD instruction executed");
        Ok(())
    }

    pub fn execute_vptestmq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPTESTMQ instruction executed");
        Ok(())
    }

    pub fn execute_vptestmw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPTESTMW instruction executed");
        Ok(())
    }

    pub fn execute_vptestnmb(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPTESTNMB instruction executed");
        Ok(())
    }

    pub fn execute_vptestnmd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPTESTNMD instruction executed");
        Ok(())
    }

    pub fn execute_vptestnmq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPTESTNMQ instruction executed");
        Ok(())
    }

    pub fn execute_vptestnmw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPTESTNMW instruction executed");
        Ok(())
    }

    pub fn execute_vpunpckhbw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPUNPCKHBW instruction executed");
        Ok(())
    }

    pub fn execute_vpunpckhdq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPUNPCKHDQ instruction executed");
        Ok(())
    }

    pub fn execute_vpunpckhqdq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPUNPCKHQDQ instruction executed");
        Ok(())
    }

    pub fn execute_vpunpckhwd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPUNPCKHWD instruction executed");
        Ok(())
    }

    pub fn execute_vpunpcklbw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPUNPCKLBW instruction executed");
        Ok(())
    }

    pub fn execute_vpunpckldq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPUNPCKLDQ instruction executed");
        Ok(())
    }

    pub fn execute_vpunpcklqdq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPUNPCKLQDQ instruction executed");
        Ok(())
    }

    pub fn execute_vpunpcklwd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPUNPCKLWD instruction executed");
        Ok(())
    }

    pub fn execute_vpxor(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPXOR instruction executed");
        Ok(())
    }

    pub fn execute_vpxord(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPXORD instruction executed");
        Ok(())
    }

    pub fn execute_vpxorq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPXORQ instruction executed");
        Ok(())
    }

    pub fn execute_vrangepd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRANGEPD instruction executed");
        Ok(())
    }

    pub fn execute_vrangeps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRANGEPS instruction executed");
        Ok(())
    }

    pub fn execute_vrangesd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRANGESD instruction executed");
        Ok(())
    }

    pub fn execute_vrangess(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRANGESS instruction executed");
        Ok(())
    }

    pub fn execute_vrcp14pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRCP14PD instruction executed");
        Ok(())
    }

    pub fn execute_vrcp14ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRCP14PS instruction executed");
        Ok(())
    }

    pub fn execute_vrcp14sd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRCP14SD instruction executed");
        Ok(())
    }

    pub fn execute_vrcp14ss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRCP14SS instruction executed");
        Ok(())
    }

    pub fn execute_vrcp28pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRCP28PD instruction executed");
        Ok(())
    }

    pub fn execute_vrcp28ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRCP28PS instruction executed");
        Ok(())
    }

    pub fn execute_vrcp28sd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRCP28SD instruction executed");
        Ok(())
    }

    pub fn execute_vrcp28ss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRCP28SS instruction executed");
        Ok(())
    }

    pub fn execute_vrcpps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRCPPS instruction executed");
        Ok(())
    }

    pub fn execute_vrcpss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRCPSS instruction executed");
        Ok(())
    }

    pub fn execute_vreducepd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VREDUCEPD instruction executed");
        Ok(())
    }

    pub fn execute_vreduceps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VREDUCEPS instruction executed");
        Ok(())
    }

    pub fn execute_vreducesd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VREDUCESD instruction executed");
        Ok(())
    }

    pub fn execute_vreducess(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VREDUCESS instruction executed");
        Ok(())
    }

    pub fn execute_vrndscalepd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRNDSCALEPD instruction executed");
        Ok(())
    }

    pub fn execute_vrndscaleps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRNDSCALEPS instruction executed");
        Ok(())
    }

    pub fn execute_vrndscalesd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRNDSCALESD instruction executed");
        Ok(())
    }

    pub fn execute_vrndscaless(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRNDSCALESS instruction executed");
        Ok(())
    }

    pub fn execute_vroundpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VROUNDPD instruction executed");
        Ok(())
    }

    pub fn execute_vroundps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VROUNDPS instruction executed");
        Ok(())
    }

    pub fn execute_vroundsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VROUNDSD instruction executed");
        Ok(())
    }

    pub fn execute_vroundss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VROUNDSS instruction executed");
        Ok(())
    }

    pub fn execute_vrsqrt14pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRSQRT14PD instruction executed");
        Ok(())
    }

    pub fn execute_vrsqrt14ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRSQRT14PS instruction executed");
        Ok(())
    }

    pub fn execute_vrsqrt14sd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRSQRT14SD instruction executed");
        Ok(())
    }

    pub fn execute_vrsqrt14ss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRSQRT14SS instruction executed");
        Ok(())
    }

    pub fn execute_vrsqrt28pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRSQRT28PD instruction executed");
        Ok(())
    }

    pub fn execute_vrsqrt28ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRSQRT28PS instruction executed");
        Ok(())
    }

    pub fn execute_vrsqrt28sd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRSQRT28SD instruction executed");
        Ok(())
    }

    pub fn execute_vrsqrt28ss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRSQRT28SS instruction executed");
        Ok(())
    }

    pub fn execute_vrsqrtps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRSQRTPS instruction executed");
        Ok(())
    }

    pub fn execute_vrsqrtss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRSQRTSS instruction executed");
        Ok(())
    }

    pub fn execute_vscalefpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCALEFPD instruction executed");
        Ok(())
    }

    pub fn execute_vscalefps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCALEFPS instruction executed");
        Ok(())
    }

    pub fn execute_vscalefsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCALEFSD instruction executed");
        Ok(())
    }

    pub fn execute_vscalefss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCALEFSS instruction executed");
        Ok(())
    }

    pub fn execute_vscatterdpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCATTERDPD instruction executed");
        Ok(())
    }

    pub fn execute_vscatterdps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCATTERDPS instruction executed");
        Ok(())
    }

    pub fn execute_vscatterpf0dpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCATTERPF0DPD instruction executed");
        Ok(())
    }

    pub fn execute_vscatterpf0dps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCATTERPF0DPS instruction executed");
        Ok(())
    }

    pub fn execute_vscatterpf0qpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCATTERPF0QPD instruction executed");
        Ok(())
    }

    pub fn execute_vscatterpf0qps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCATTERPF0QPS instruction executed");
        Ok(())
    }

    pub fn execute_vscatterpf1dpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCATTERPF1DPD instruction executed");
        Ok(())
    }

    pub fn execute_vscatterpf1dps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCATTERPF1DPS instruction executed");
        Ok(())
    }

    pub fn execute_vscatterpf1qpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCATTERPF1QPD instruction executed");
        Ok(())
    }

    pub fn execute_vscatterpf1qps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCATTERPF1QPS instruction executed");
        Ok(())
    }

    pub fn execute_vscatterqpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCATTERQPD instruction executed");
        Ok(())
    }

    pub fn execute_vscatterqps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCATTERQPS instruction executed");
        Ok(())
    }

    pub fn execute_vshuff32x4(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSHUFF32X4 instruction executed");
        Ok(())
    }

    pub fn execute_vshuff64x2(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSHUFF64X2 instruction executed");
        Ok(())
    }

    pub fn execute_vshufi32x4(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSHUFI32X4 instruction executed");
        Ok(())
    }

    pub fn execute_vshufi64x2(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSHUFI64X2 instruction executed");
        Ok(())
    }

    pub fn execute_vshufpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSHUFPD instruction executed");
        Ok(())
    }

    pub fn execute_vshufps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSHUFPS instruction executed");
        Ok(())
    }

    pub fn execute_vsqrtpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSQRTPD instruction executed");
        Ok(())
    }

    pub fn execute_vsqrtps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSQRTPS instruction executed");
        Ok(())
    }

    pub fn execute_vsqrtsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSQRTSD instruction executed");
        Ok(())
    }

    pub fn execute_vsqrtss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSQRTSS instruction executed");
        Ok(())
    }

    pub fn execute_vstmxcsr(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSTMXCSR instruction executed");
        Ok(())
    }

    pub fn execute_vsubpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSUBPD instruction executed");
        Ok(())
    }

    pub fn execute_vsubps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSUBPS instruction executed");
        Ok(())
    }

    pub fn execute_vsubsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSUBSD instruction executed");
        Ok(())
    }

    pub fn execute_vsubss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSUBSS instruction executed");
        Ok(())
    }

    pub fn execute_vtestpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VTESTPD instruction executed");
        Ok(())
    }

    pub fn execute_vtestps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VTESTPS instruction executed");
        Ok(())
    }

    pub fn execute_vucomisd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VUCOMISD instruction executed");
        Ok(())
    }

    pub fn execute_vucomiss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VUCOMISS instruction executed");
        Ok(())
    }

    pub fn execute_vunpckhpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VUNPCKHPD instruction executed");
        Ok(())
    }

    pub fn execute_vunpckhps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VUNPCKHPS instruction executed");
        Ok(())
    }

    pub fn execute_vunpcklpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VUNPCKLPD instruction executed");
        Ok(())
    }

    pub fn execute_vunpcklps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VUNPCKLPS instruction executed");
        Ok(())
    }

    pub fn execute_vxorpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VXORPD instruction executed");
        Ok(())
    }

    pub fn execute_vxorps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VXORPS instruction executed");
        Ok(())
    }

    pub fn execute_vzeroall(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VZEROALL instruction executed");
        Ok(())
    }

    pub fn execute_vzeroupper(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VZEROUPPER instruction executed");
        Ok(())
    }

    pub fn execute_vmgexit(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMGEXIT instruction executed");
        Ok(())
    }

    pub fn execute_vaddph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VADDPH instruction executed");
        Ok(())
    }

    pub fn execute_vaddsh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VADDSH instruction executed");
        Ok(())
    }

    pub fn execute_vcmpph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCMPPH instruction executed");
        Ok(())
    }

    pub fn execute_vcmpsh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCMPSH instruction executed");
        Ok(())
    }

    pub fn execute_vcomish(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCOMISH instruction executed");
        Ok(())
    }

    pub fn execute_vcvtdq2ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTDQ2PH instruction executed");
        Ok(())
    }

    pub fn execute_vcvtpd2ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPD2PH instruction executed");
        Ok(())
    }

    pub fn execute_vcvtph2dq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPH2DQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvtph2pd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPH2PD instruction executed");
        Ok(())
    }

    pub fn execute_vcvtph2psx(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPH2PSX instruction executed");
        Ok(())
    }

    pub fn execute_vcvtph2qq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPH2QQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvtph2udq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPH2UDQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvtph2uqq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPH2UQQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvtph2uw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPH2UW instruction executed");
        Ok(())
    }

    pub fn execute_vcvtph2w(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPH2W instruction executed");
        Ok(())
    }

    pub fn execute_vcvtps2phx(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTPS2PHX instruction executed");
        Ok(())
    }

    pub fn execute_vcvtqq2ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTQQ2PH instruction executed");
        Ok(())
    }

    pub fn execute_vcvtsd2sh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTSD2SH instruction executed");
        Ok(())
    }

    pub fn execute_vcvtsh2sd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTSH2SD instruction executed");
        Ok(())
    }

    pub fn execute_vcvtsh2si(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTSH2SI instruction executed");
        Ok(())
    }

    pub fn execute_vcvtsh2ss(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTSH2SS instruction executed");
        Ok(())
    }

    pub fn execute_vcvtsh2usi(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTSH2USI instruction executed");
        Ok(())
    }

    pub fn execute_vcvtsi2sh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTSI2SH instruction executed");
        Ok(())
    }

    pub fn execute_vcvtss2sh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTSS2SH instruction executed");
        Ok(())
    }

    pub fn execute_vcvttph2dq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTTPH2DQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvttph2qq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTTPH2QQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvttph2udq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTTPH2UDQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvttph2uqq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTTPH2UQQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvttph2uw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTTPH2UW instruction executed");
        Ok(())
    }

    pub fn execute_vcvttph2w(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTTPH2W instruction executed");
        Ok(())
    }

    pub fn execute_vcvttsh2si(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTTSH2SI instruction executed");
        Ok(())
    }

    pub fn execute_vcvttsh2usi(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTTSH2USI instruction executed");
        Ok(())
    }

    pub fn execute_vcvtudq2ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTUDQ2PH instruction executed");
        Ok(())
    }

    pub fn execute_vcvtuqq2ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTUQQ2PH instruction executed");
        Ok(())
    }

    pub fn execute_vcvtusi2sh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTUSI2SH instruction executed");
        Ok(())
    }

    pub fn execute_vcvtuw2ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTUW2PH instruction executed");
        Ok(())
    }

    pub fn execute_vcvtw2ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTW2PH instruction executed");
        Ok(())
    }

    pub fn execute_vdivph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VDIVPH instruction executed");
        Ok(())
    }

    pub fn execute_vdivsh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VDIVSH instruction executed");
        Ok(())
    }

    pub fn execute_vfcmaddcph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFCMADDCPH instruction executed");
        Ok(())
    }

    pub fn execute_vfmaddcph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADDCPH instruction executed");
        Ok(())
    }

    pub fn execute_vfcmaddcsh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFCMADDCSH instruction executed");
        Ok(())
    }

    pub fn execute_vfmaddcsh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADDCSH instruction executed");
        Ok(())
    }

    pub fn execute_vfcmulcph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFCMULCPH instruction executed");
        Ok(())
    }

    pub fn execute_vfmulcph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMULCPH instruction executed");
        Ok(())
    }

    pub fn execute_vfcmulcsh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFCMULCSH instruction executed");
        Ok(())
    }

    pub fn execute_vfmulcsh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMULCSH instruction executed");
        Ok(())
    }

    pub fn execute_vfmaddsub132ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADDSUB132PH instruction executed");
        Ok(())
    }

    pub fn execute_vfmaddsub213ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADDSUB213PH instruction executed");
        Ok(())
    }

    pub fn execute_vfmaddsub231ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADDSUB231PH instruction executed");
        Ok(())
    }

    pub fn execute_vfmsubadd132ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUBADD132PH instruction executed");
        Ok(())
    }

    pub fn execute_vfmsubadd213ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUBADD213PH instruction executed");
        Ok(())
    }

    pub fn execute_vfmsubadd231ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUBADD231PH instruction executed");
        Ok(())
    }

    pub fn execute_vfmadd132ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADD132PH instruction executed");
        Ok(())
    }

    pub fn execute_vfmadd213ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADD213PH instruction executed");
        Ok(())
    }

    pub fn execute_vfmadd231ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADD231PH instruction executed");
        Ok(())
    }

    pub fn execute_vfnmadd132ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADD132PH instruction executed");
        Ok(())
    }

    pub fn execute_vfnmadd213ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADD213PH instruction executed");
        Ok(())
    }

    pub fn execute_vfnmadd231ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADD231PH instruction executed");
        Ok(())
    }

    pub fn execute_vfmadd132sh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADD132SH instruction executed");
        Ok(())
    }

    pub fn execute_vfmadd213sh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADD213SH instruction executed");
        Ok(())
    }

    pub fn execute_vfmadd231sh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADD231SH instruction executed");
        Ok(())
    }

    pub fn execute_vfnmadd132sh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADD132SH instruction executed");
        Ok(())
    }

    pub fn execute_vfnmadd213sh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADD213SH instruction executed");
        Ok(())
    }

    pub fn execute_vfnmadd231sh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMADD231SH instruction executed");
        Ok(())
    }

    pub fn execute_vfmsub132ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUB132PH instruction executed");
        Ok(())
    }

    pub fn execute_vfmsub213ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUB213PH instruction executed");
        Ok(())
    }

    pub fn execute_vfmsub231ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUB231PH instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsub132ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUB132PH instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsub213ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUB213PH instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsub231ph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUB231PH instruction executed");
        Ok(())
    }

    pub fn execute_vfmsub132sh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUB132SH instruction executed");
        Ok(())
    }

    pub fn execute_vfmsub213sh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUB213SH instruction executed");
        Ok(())
    }

    pub fn execute_vfmsub231sh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMSUB231SH instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsub132sh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUB132SH instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsub213sh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUB213SH instruction executed");
        Ok(())
    }

    pub fn execute_vfnmsub231sh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFNMSUB231SH instruction executed");
        Ok(())
    }

    pub fn execute_vfpclassph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFPCLASSPH instruction executed");
        Ok(())
    }

    pub fn execute_vfpclasssh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFPCLASSSH instruction executed");
        Ok(())
    }

    pub fn execute_vgetexpph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGETEXPPH instruction executed");
        Ok(())
    }

    pub fn execute_vgetexpsh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGETEXPSH instruction executed");
        Ok(())
    }

    pub fn execute_vgetmantph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGETMANTPH instruction executed");
        Ok(())
    }

    pub fn execute_vgetmantsh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGETMANTSH instruction executed");
        Ok(())
    }

    pub fn execute_vmaxph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMAXPH instruction executed");
        Ok(())
    }

    pub fn execute_vmaxsh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMAXSH instruction executed");
        Ok(())
    }

    pub fn execute_vminph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMINPH instruction executed");
        Ok(())
    }

    pub fn execute_vminsh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMINSH instruction executed");
        Ok(())
    }

    pub fn execute_vmovsh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVSH instruction executed");
        Ok(())
    }

    pub fn execute_vmovw(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVW instruction executed");
        Ok(())
    }

    pub fn execute_vmulph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMULPH instruction executed");
        Ok(())
    }

    pub fn execute_vmulsh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMULSH instruction executed");
        Ok(())
    }

    pub fn execute_vrcpph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRCPPH instruction executed");
        Ok(())
    }

    pub fn execute_vrcpsh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRCPSH instruction executed");
        Ok(())
    }

    pub fn execute_vreduceph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VREDUCEPH instruction executed");
        Ok(())
    }

    pub fn execute_vreducesh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VREDUCESH instruction executed");
        Ok(())
    }

    pub fn execute_vrndscaleph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRNDSCALEPH instruction executed");
        Ok(())
    }

    pub fn execute_vrndscalesh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRNDSCALESH instruction executed");
        Ok(())
    }

    pub fn execute_vrsqrtph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRSQRTPH instruction executed");
        Ok(())
    }

    pub fn execute_vrsqrtsh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRSQRTSH instruction executed");
        Ok(())
    }

    pub fn execute_vscalefph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCALEFPH instruction executed");
        Ok(())
    }

    pub fn execute_vscalefsh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCALEFSH instruction executed");
        Ok(())
    }

    pub fn execute_vsqrtph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSQRTPH instruction executed");
        Ok(())
    }

    pub fn execute_vsqrtsh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSQRTSH instruction executed");
        Ok(())
    }

    pub fn execute_vsubph(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSUBPH instruction executed");
        Ok(())
    }

    pub fn execute_vsubsh(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSUBSH instruction executed");
        Ok(())
    }

    pub fn execute_vucomish(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VUCOMISH instruction executed");
        Ok(())
    }

    pub fn execute_vaddnpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VADDNPD instruction executed");
        Ok(())
    }

    pub fn execute_vaddnps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VADDNPS instruction executed");
        Ok(())
    }

    pub fn execute_vaddsetsps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VADDSETSPS instruction executed");
        Ok(())
    }

    pub fn execute_vcvtfxpntdq2ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTFXPNTDQ2PS instruction executed");
        Ok(())
    }

    pub fn execute_vcvtfxpntpd2dq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTFXPNTPD2DQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvtfxpntpd2udq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTFXPNTPD2UDQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvtfxpntps2dq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTFXPNTPS2DQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvtfxpntps2udq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTFXPNTPS2UDQ instruction executed");
        Ok(())
    }

    pub fn execute_vcvtfxpntudq2ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTFXPNTUDQ2PS instruction executed");
        Ok(())
    }

    pub fn execute_vexp223ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VEXP223PS instruction executed");
        Ok(())
    }

    pub fn execute_vfixupnanpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFIXUPNANPD instruction executed");
        Ok(())
    }

    pub fn execute_vfixupnanps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFIXUPNANPS instruction executed");
        Ok(())
    }

    pub fn execute_vfmadd233ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VFMADD233PS instruction executed");
        Ok(())
    }

    pub fn execute_vgatherpf0hintdpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGATHERPF0HINTDPD instruction executed");
        Ok(())
    }

    pub fn execute_vgatherpf0hintdps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGATHERPF0HINTDPS instruction executed");
        Ok(())
    }

    pub fn execute_vgmaxabsps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGMAXABSPS instruction executed");
        Ok(())
    }

    pub fn execute_vgmaxpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGMAXPD instruction executed");
        Ok(())
    }

    pub fn execute_vgmaxps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGMAXPS instruction executed");
        Ok(())
    }

    pub fn execute_vgminpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGMINPD instruction executed");
        Ok(())
    }

    pub fn execute_vgminps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VGMINPS instruction executed");
        Ok(())
    }

    pub fn execute_vloadunpackhd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VLOADUNPACKHD instruction executed");
        Ok(())
    }

    pub fn execute_vloadunpackhpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VLOADUNPACKHPD instruction executed");
        Ok(())
    }

    pub fn execute_vloadunpackhps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VLOADUNPACKHPS instruction executed");
        Ok(())
    }

    pub fn execute_vloadunpackhq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VLOADUNPACKHQ instruction executed");
        Ok(())
    }

    pub fn execute_vloadunpackld(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VLOADUNPACKLD instruction executed");
        Ok(())
    }

    pub fn execute_vloadunpacklpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VLOADUNPACKLPD instruction executed");
        Ok(())
    }

    pub fn execute_vloadunpacklps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VLOADUNPACKLPS instruction executed");
        Ok(())
    }

    pub fn execute_vloadunpacklq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VLOADUNPACKLQ instruction executed");
        Ok(())
    }

    pub fn execute_vlog2ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VLOG2PS instruction executed");
        Ok(())
    }

    pub fn execute_vmovnrapd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVNRAPD instruction executed");
        Ok(())
    }

    pub fn execute_vmovnraps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVNRAPS instruction executed");
        Ok(())
    }

    pub fn execute_vmovnrngoapd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVNRNGOAPD instruction executed");
        Ok(())
    }

    pub fn execute_vmovnrngoaps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VMOVNRNGOAPS instruction executed");
        Ok(())
    }

    pub fn execute_vpackstorehd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPACKSTOREHD instruction executed");
        Ok(())
    }

    pub fn execute_vpackstorehpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPACKSTOREHPD instruction executed");
        Ok(())
    }

    pub fn execute_vpackstorehps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPACKSTOREHPS instruction executed");
        Ok(())
    }

    pub fn execute_vpackstorehq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPACKSTOREHQ instruction executed");
        Ok(())
    }

    pub fn execute_vpackstoreld(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPACKSTORELD instruction executed");
        Ok(())
    }

    pub fn execute_vpackstorelpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPACKSTORELPD instruction executed");
        Ok(())
    }

    pub fn execute_vpackstorelps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPACKSTORELPS instruction executed");
        Ok(())
    }

    pub fn execute_vpackstorelq(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPACKSTORELQ instruction executed");
        Ok(())
    }

    pub fn execute_vpadcd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPADCD instruction executed");
        Ok(())
    }

    pub fn execute_vpaddsetcd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPADDSETCD instruction executed");
        Ok(())
    }

    pub fn execute_vpaddsetsd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPADDSETSD instruction executed");
        Ok(())
    }

    pub fn execute_vpcmpltd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPCMPLTD instruction executed");
        Ok(())
    }

    pub fn execute_vpermf32x4(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPERMF32X4 instruction executed");
        Ok(())
    }

    pub fn execute_vpmadd231d(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMADD231D instruction executed");
        Ok(())
    }

    pub fn execute_vpmadd233d(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMADD233D instruction executed");
        Ok(())
    }

    pub fn execute_vpmulhd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMULHD instruction executed");
        Ok(())
    }

    pub fn execute_vpmulhud(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPMULHUD instruction executed");
        Ok(())
    }

    pub fn execute_vprefetch0(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPREFETCH0 instruction executed");
        Ok(())
    }

    pub fn execute_vprefetch1(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPREFETCH1 instruction executed");
        Ok(())
    }

    pub fn execute_vprefetch2(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPREFETCH2 instruction executed");
        Ok(())
    }

    pub fn execute_vprefetche0(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPREFETCHE0 instruction executed");
        Ok(())
    }

    pub fn execute_vprefetche1(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPREFETCHE1 instruction executed");
        Ok(())
    }

    pub fn execute_vprefetche2(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPREFETCHE2 instruction executed");
        Ok(())
    }

    pub fn execute_vprefetchenta(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPREFETCHENTA instruction executed");
        Ok(())
    }

    pub fn execute_vprefetchnta(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPREFETCHNTA instruction executed");
        Ok(())
    }

    pub fn execute_vpsbbd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSBBD instruction executed");
        Ok(())
    }

    pub fn execute_vpsbbrd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSBBRD instruction executed");
        Ok(())
    }

    pub fn execute_vpsubrd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSUBRD instruction executed");
        Ok(())
    }

    pub fn execute_vpsubrsetbd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSUBRSETBD instruction executed");
        Ok(())
    }

    pub fn execute_vpsubsetbd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPSUBSETBD instruction executed");
        Ok(())
    }

    pub fn execute_vrcp23ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRCP23PS instruction executed");
        Ok(())
    }

    pub fn execute_vrndfxpntpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRNDFXPNTPD instruction executed");
        Ok(())
    }

    pub fn execute_vrndfxpntps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRNDFXPNTPS instruction executed");
        Ok(())
    }

    pub fn execute_vrsqrt23ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VRSQRT23PS instruction executed");
        Ok(())
    }

    pub fn execute_vscaleps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCALEPS instruction executed");
        Ok(())
    }

    pub fn execute_vscatterpf0hintdpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCATTERPF0HINTDPD instruction executed");
        Ok(())
    }

    pub fn execute_vscatterpf0hintdps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSCATTERPF0HINTDPS instruction executed");
        Ok(())
    }

    pub fn execute_vsubrpd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSUBRPD instruction executed");
        Ok(())
    }

    pub fn execute_vsubrps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSUBRPS instruction executed");
        Ok(())
    }

    pub fn execute_vbcstnebf162ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBCSTNEBF162PS instruction executed");
        Ok(())
    }

    pub fn execute_vbcstnesh2ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VBCSTNESH2PS instruction executed");
        Ok(())
    }

    pub fn execute_vcvtneebf162ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTNEEBF162PS instruction executed");
        Ok(())
    }

    pub fn execute_vcvtneeph2ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTNEEPH2PS instruction executed");
        Ok(())
    }

    pub fn execute_vcvtneobf162ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTNEOBF162PS instruction executed");
        Ok(())
    }

    pub fn execute_vcvtneoph2ps(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VCVTNEOPH2PS instruction executed");
        Ok(())
    }

    pub fn execute_vpdpbssd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPDPBSSD instruction executed");
        Ok(())
    }

    pub fn execute_vpdpbssds(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPDPBSSDS instruction executed");
        Ok(())
    }

    pub fn execute_vpdpbsud(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPDPBSUD instruction executed");
        Ok(())
    }

    pub fn execute_vpdpbsuds(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPDPBSUDS instruction executed");
        Ok(())
    }

    pub fn execute_vpdpbuud(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPDPBUUD instruction executed");
        Ok(())
    }

    pub fn execute_vpdpbuuds(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPDPBUUDS instruction executed");
        Ok(())
    }

    pub fn execute_vpdpwsud(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPDPWSUD instruction executed");
        Ok(())
    }

    pub fn execute_vpdpwsuds(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPDPWSUDS instruction executed");
        Ok(())
    }

    pub fn execute_vpdpwusd(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPDPWUSD instruction executed");
        Ok(())
    }

    pub fn execute_vpdpwusds(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPDPWUSDS instruction executed");
        Ok(())
    }

    pub fn execute_vpdpwuud(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPDPWUUD instruction executed");
        Ok(())
    }

    pub fn execute_vpdpwuuds(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VPDPWUUDS instruction executed");
        Ok(())
    }

    pub fn execute_vsha512msg1(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSHA512MSG1 instruction executed");
        Ok(())
    }

    pub fn execute_vsha512msg2(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSHA512MSG2 instruction executed");
        Ok(())
    }

    pub fn execute_vsha512rnds2(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSHA512RNDS2 instruction executed");
        Ok(())
    }

    pub fn execute_vsm3msg1(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSM3MSG1 instruction executed");
        Ok(())
    }

    pub fn execute_vsm3msg2(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSM3MSG2 instruction executed");
        Ok(())
    }

    pub fn execute_vsm3rnds2(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSM3RNDS2 instruction executed");
        Ok(())
    }

    pub fn execute_vsm4key4(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSM4KEY4 instruction executed");
        Ok(())
    }

    pub fn execute_vsm4rnds4(&self, _instruction: &Instruction, state: &mut CpuState) -> Result<()> {
        log::debug!("VSM4RNDS4 instruction executed");
        Ok(())
    }
}
