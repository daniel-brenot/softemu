#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod uart;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize UART console
    uart::init_uart();
    
    // Print bootup messages to UART
    uart::uart_println("==========================================");
    uart::uart_println("    SoftEmu Simple Kernel Bootup");
    uart::uart_println("==========================================");
    uart::uart_println("");
    uart::uart_println("Kernel loaded successfully!");
    uart::uart_println("UART console initialized at COM1 (0x3F8)");
    uart::uart_println("Simple kernel is running...");
    uart::uart_println("");
    
    // Simple command loop
    let mut counter = 0;
    loop {
        uart::uart_print("kernel> ");
        
        // Simulate some kernel activity
        counter += 1;
        if counter % 1000000 == 0 {
            uart::uart_print("Kernel heartbeat: ");
            uart::uart_println(&format_number(counter / 1000000));
        }
        
        // Simple delay
        for _ in 0..100000 {
            unsafe { core::arch::asm!("nop") }
        }
    }
}

/// Simple number formatting without std
fn format_number(mut n: usize) -> heapless::String<32> {
    let mut result = heapless::String::new();
    
    if n == 0 {
        result.push('0').ok();
        return result;
    }
    
    let mut digits = heapless::Vec::<u8, 32>::new();
    while n > 0 {
        digits.push((n % 10) as u8 + b'0').ok();
        n /= 10;
    }
    
    for &digit in digits.iter().rev() {
        result.push(digit as char).ok();
    }
    
    result
}
