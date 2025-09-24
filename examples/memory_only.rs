use anyhow::Result;
use softemu::GuestMemory;

/// Memory-only example that demonstrates basic memory operations
fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    println!("SoftEmu Memory-Only Example");
    println!("===========================");

    // Create guest memory with 512MB
    let mut memory = GuestMemory::new(512 * 1024 * 1024)?;
    println!("Created guest memory with 512MB");

    // Test basic memory operations
    println!("Testing memory operations...");
    
    // Write some test data
    let test_data = vec![0x48, 0xC7, 0xC0, 0x78, 0x56, 0x34, 0x12, 0xF4]; // mov $0x12345678, %rax; hlt
    memory.write_slice(0x100000, &test_data)?;
    println!("✓ Wrote test data to address 0x100000");

    // Read it back
    let read_data = memory.read_slice(0x100000, test_data.len())?;
    if read_data == test_data {
        println!("✓ Memory read/write verification successful");
    } else {
        println!("✗ Memory read/write verification failed");
        return Ok(());
    }

    // Test individual byte operations
    memory.write_u8(0x200000, 0xAB)?;
    let byte = memory.read_u8(0x200000)?;
    if byte == 0xAB {
        println!("✓ Byte read/write verification successful");
    } else {
        println!("✗ Byte read/write verification failed");
    }

    // Test 32-bit operations
    memory.write_u32(0x300000, 0x12345678)?;
    let word = memory.read_u32(0x300000)?;
    if word == 0x12345678 {
        println!("✓ 32-bit read/write verification successful");
    } else {
        println!("✗ 32-bit read/write verification failed");
    }

    // Test 64-bit operations
    memory.write_u64(0x400000, 0x123456789ABCDEF0)?;
    let qword = memory.read_u64(0x400000)?;
    if qword == 0x123456789ABCDEF0 {
        println!("✓ 64-bit read/write verification successful");
    } else {
        println!("✗ 64-bit read/write verification failed");
    }

    // Test bounds checking
    match memory.read_u8(512 * 1024 * 1024) {
        Ok(_) => println!("✗ Bounds checking failed - should have errored"),
        Err(_) => println!("✓ Bounds checking working correctly"),
    }

    println!("All memory operations completed successfully!");
    Ok(())
}
