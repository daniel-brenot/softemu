use softemu::devices::SerialConsole;
use softemu::memory::mmio::MmioDevice;

fn main() {
    println!("Serial Console Example");
    
    // Create a new serial console
    let mut console = SerialConsole::new();
    
    // Test basic UART register operations
    println!("\n=== Testing UART Registers ===");
    
    // Test LSR (Line Status Register) - should show transmitter empty
    let lsr = console.read(5, 1).unwrap();
    println!("LSR (Line Status Register): 0x{:02X}", lsr);
    
    // Test writing to THR (Transmitter Holding Register)
    println!("\n=== Testing Character Transmission ===");
    console.write(0, b'H' as u64, 1).unwrap();
    console.write(0, b'e' as u64, 1).unwrap();
    console.write(0, b'l' as u64, 1).unwrap();
    console.write(0, b'l' as u64, 1).unwrap();
    console.write(0, b'o' as u64, 1).unwrap();
    console.write(0, b'\n' as u64, 1).unwrap();
    
    // Test character injection (simulating host input)
    println!("\n=== Testing Character Reception ===");
    console.inject_char(b'W');
    console.inject_char(b'o');
    console.inject_char(b'r');
    console.inject_char(b'l');
    console.inject_char(b'd');
    console.inject_char(b'\n');
    
    // Check if data is available
    if console.has_rx_data() {
        println!("RX data available!");
        
        // Read characters from RX FIFO
        while console.has_rx_data() {
            if let Some(ch) = console.consume_rx_char() {
                print!("{}", ch as char);
            }
        }
        println!();
    }
    
    // Test interrupt enable register
    println!("\n=== Testing Interrupt Configuration ===");
    console.write(1, 0x01, 1).unwrap(); // Enable received data interrupt
    let ier = console.read(1, 1).unwrap();
    println!("IER (Interrupt Enable Register): 0x{:02X}", ier);
    
    // Test line control register
    console.write(3, 0x03, 1).unwrap(); // 8 data bits, no parity, 1 stop bit
    let lcr = console.read(3, 1).unwrap();
    println!("LCR (Line Control Register): 0x{:02X}", lcr);
    
    println!("\nSerial console test completed!");
}
