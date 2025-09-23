# Project goal
This project has the goal of running a x86_64 virtual machine without using kvm, windows hypervisor or any other hardware virtualization.
It should use crates where applicable to handle things like loading the kernel, creating guest memory, etc.

The vm should use mmio and should not use any pcie. It needs to have network support.

# Recommended crates
vm-memory
vm-device
linux-loader
memfd
iced_x86_64
vm-allocator

# Language
This project should be written using the rust programming language.
