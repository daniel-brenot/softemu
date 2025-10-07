use crate::cpu::CpuState;
use crate::cpu::fault::Fault;

pub fn execute_bndcl_bnd_rm32(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_bndcu_bnd_rm32(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_bndmk_bnd_m32(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_bndmov_bndm64_bnd(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_bndmov_bnd_bndm64(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_bndstx_mib_bnd(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_clgi(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_clrssbsy_m64(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_encls(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_enclu(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_enclv(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_endbr32(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_endbr64(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_enqcmds_r16_m512(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_enqcmds_r32_m512(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_enqcmd_r16_m512(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_enqcmd_r32_m512(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_hreset_imm8(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_invlpgad(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_invlpgaw(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_invlpgbd(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_invlpgbw(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_mcommit(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_monitord(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_monitorw(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_monitorxd(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_monitorxw(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_movdir64b_r16_m512(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_movdir64b_r32_m512(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_movdiri_m32_r32(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_mwait(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_mwaitx(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_pconfig(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_ptwrite_rm32(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_pvalidated(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_pvalidatew(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_rdpkru(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_rdshr_rm32(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_rstorssp_m64(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_saveprevssp(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_setssbsy(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_stgi(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_tlbsync(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_vmclear_m64(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_vmfunc(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_vmloadd(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_vmloadw(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_vmptrld_m64(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_vmptrst_m64(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_vmread_rm32_r32(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_vmsaved(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_vmsavew(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_vmwrite_r32_rm32(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_vmxoff(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_vmxon_m64(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_wrpkru(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_wrshr_rm32(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_wrssd_m32_r32(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_wrussd_m32_r32(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_xabort_imm8(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

pub fn execute_xtest(state: &mut CpuState) -> Result<(), Fault> {
    unimplemented!();
}

