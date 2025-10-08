use crate::{cpu::{fault::Fault, CpuState}, VirtualMachine};

#[cfg(feature = "x16_real_mode")]
pub mod x16_real_mode;
#[cfg(feature = "x16_protected_mode")]
pub mod x16_protected_mode;
#[cfg(feature = "x32_mode")]
pub mod x32_mode;
#[cfg(feature = "x64_mode")]
pub mod x64_mode;
#[cfg(feature = "virtual8086_mode")]
pub mod virtual8086_mode;

/// Wraps calling instructions with interrupt handling
/// that supports double fault recovery
pub fn execute_instruction_with_interrupt_handler(state: &mut CpuState, vm: &VirtualMachine) {
    if let Err(e) = execute_instruction(state, vm) {
        if let Err(_) = state.handle_interrupt(&vm.memory_manager, e as u8) {
            if let Err(_) = state.handle_interrupt(&vm.memory_manager, Fault::DoubleFault as u8) {
                log::error!("Triple fault detected, halting virtual machine");
                // TODO: find some way to signal we are in a triple fault state
                return;
            }
        }
    }
}

pub fn execute_instruction(state: &mut CpuState, vm: &VirtualMachine) -> Result<(), Fault> {
    Ok(())
}

