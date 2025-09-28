// UART driver for COM1 (0x3F8)
// Implements basic UART functionality for kernel console output

use core::ptr;

const COM1_BASE: u16 = 0x3F8;

// UART Register offsets
const RBR_THR: u16 = 0x00;  // Receiver Buffer Register / Transmitter Holding Register
const IER: u16 = 0x01;      // Interrupt Enable Register
const IIR_FCR: u16 = 0x02;  // Interrupt Identification Register / FIFO Control Register
const LCR: u16 = 0x03;      // Line Control Register
const MCR: u16 = 0x04;      // Modem Control Register
const LSR: u16 = 0x05;      // Line Status Register
const MSR: u16 = 0x06;      // Modem Status Register
const SCR: u16 = 0x07;      // Scratch Register

// LSR bits
const LSR_DATA_READY: u8 = 0x01;
const LSR_OVERRUN_ERROR: u8 = 0x02;
const LSR_PARITY_ERROR: u8 = 0x04;
const LSR_FRAMING_ERROR: u8 = 0x08;
const LSR_BREAK_INTERRUPT: u8 = 0x10;
const LSR_TRANSMITTER_EMPTY: u8 = 0x20;
const LSR_TRANSMITTER_IDLE: u8 = 0x40;
const LSR_FIFO_ERROR: u8 = 0x80;

pub struct Uart {
    base_port: u16,
}

impl Uart {
    pub fn new() -> Self {
        Self {
            base_port: COM1_BASE,
        }
    }

    /// Initialize the UART
    pub fn init(&mut self) {
        // Disable interrupts
        self.outb(IER, 0x00);
        
        // Enable DLAB (Divisor Latch Access Bit)
        self.outb(LCR, 0x80);
        
        // Set divisor to 1 (115200 baud with 1.8432 MHz clock)
        self.outb(RBR_THR, 0x01);  // DLL
        self.outb(IER, 0x00);      // DLM
        
        // 8 data bits, no parity, 1 stop bit, disable DLAB
        self.outb(LCR, 0x03);
        
        // Enable FIFO, clear them, with 14-byte threshold
        self.outb(IIR_FCR, 0xC7);
        
        // Enable interrupts
        self.outb(IER, 0x01);
    }

    /// Check if transmitter is empty
    fn is_transmit_empty(&self) -> bool {
        (self.inb(LSR) & LSR_TRANSMITTER_EMPTY) != 0
    }

    /// Write a byte to the UART
    pub fn write_byte(&mut self, byte: u8) {
        // Wait for transmitter to be empty
        while !self.is_transmit_empty() {
            // Busy wait
        }
        
        // Write the byte
        self.outb(RBR_THR, byte);
    }

    /// Write a string to the UART
    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    /// Read a byte from the UART (non-blocking)
    pub fn read_byte(&self) -> Option<u8> {
        if (self.inb(LSR) & LSR_DATA_READY) != 0 {
            Some(self.inb(RBR_THR))
        } else {
            None
        }
    }

    /// Output a byte to a port
    fn outb(&self, port: u16, value: u8) {
        unsafe {
            ptr::write_volatile((self.base_port + port) as *mut u8, value);
        }
    }

    /// Input a byte from a port
    fn inb(&self, port: u16) -> u8 {
        unsafe {
            ptr::read_volatile((self.base_port + port) as *const u8)
        }
    }
}

// Global UART instance
static mut UART: Option<Uart> = None;

/// Initialize the global UART
pub fn init_uart() {
    unsafe {
        UART = Some(Uart::new());
        if let Some(ref mut uart) = UART {
            uart.init();
        }
    }
}

/// Write a string to the UART console
pub fn uart_print(s: &str) {
    unsafe {
        if let Some(ref mut uart) = UART {
            uart.write_str(s);
        }
    }
}

/// Write a formatted string to the UART console
pub fn uart_println(s: &str) {
    uart_print(s);
    uart_print("\r\n");
}
