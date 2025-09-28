use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let kernel_path = out_dir.join("simple_kernel");
    
    // Build the kernel
    let status = Command::new("cargo")
        .args(&["build", "--release", "--target", "x86_64-unknown-none"])
        .status()
        .expect("Failed to build kernel");
    
    if !status.success() {
        panic!("Kernel build failed");
    }
    
    // Convert ELF to binary
    let elf_path = PathBuf::from("target/x86_64-unknown-none/release/simple_kernel");
    let binary_path = out_dir.join("simple_kernel.bin");
    
    let status = Command::new("objcopy")
        .args(&["-O", "binary", &elf_path.to_string_lossy(), &binary_path.to_string_lossy()])
        .status()
        .expect("Failed to convert ELF to binary");
    
    if !status.success() {
        panic!("objcopy failed");
    }
    
    // Pass the binary path to the main build script
    println!("cargo:rustc-env=KERNEL_BINARY={}", binary_path.display());
}
