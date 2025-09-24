use anyhow::Result;
use clap::Parser;
use log::info;
use softemu::vm::VirtualMachine;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "softemu")]
#[command(about = "A software-based x86_64 emulator")]
struct Cli {
    /// Path to the kernel image to load
    #[arg(short, long)]
    kernel: PathBuf,

    /// Path to the initrd (optional)
    #[arg(short, long)]
    initrd: Option<PathBuf>,

    /// Amount of RAM to allocate (in MB)
    #[arg(short, long, default_value = "512")]
    memory: u64,

    /// Number of CPU cores to emulate
    #[arg(short, long, default_value = "1")]
    cores: u32,

    /// Enable debug output
    #[arg(short, long)]
    debug: bool,

    /// Network interface name (optional)
    #[arg(short, long)]
    network: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(if cli.debug {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init();

    info!("Starting SoftEmu x86_64 emulator");
    info!("Kernel: {:?}", cli.kernel);
    info!("Memory: {} MB", cli.memory);
    info!("Cores: {}", cli.cores);

    // Create and configure the virtual machine
    let mut vm = VirtualMachine::new(cli.memory * 1024 * 1024, cli.cores)?;

    // Load the kernel
    vm.load_kernel(&cli.kernel)?;

    // Load initrd if provided
    if let Some(initrd_path) = cli.initrd {
        let initrd_data = std::fs::read(&initrd_path)?;
        vm.load_initrd(&initrd_data)?;
    }

    // Configure network if requested
    if let Some(network_interface) = cli.network {
        // Network functionality is commented out in the VM
        info!("Network interface {} would be configured", network_interface);
    }

    // Start the virtual machine
    info!("Starting virtual machine execution...");
    vm.run()?;

    Ok(())
}
