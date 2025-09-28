use crate::memory::mmio::MmioDevice;
use crate::Result;
use std::collections::VecDeque;

/// UART Serial Console Device
/// Implements a standard 16550 UART compatible interface
/// 
/// This device can be used as a console for Linux kernels by:
/// 1. Registering it at COM1 address (0x3F8)
/// 2. Configuring Linux kernel with console=ttyS0,115200n8
/// 3. The device supports standard UART registers:
///    - RBR/THR (0x00): Receiver Buffer / Transmitter Holding Register
///    - IER (0x01): Interrupt Enable Register  
///    - IIR/FCR (0x02): Interrupt Identification / FIFO Control Register
///    - LCR (0x03): Line Control Register
///    - MCR (0x04): Modem Control Register
///    - LSR (0x05): Line Status Register
///    - MSR (0x06): Modem Status Register
///    - SCR (0x07): Scratch Register
pub struct SerialConsole {
    // UART Registers
    rbr_thr: u8,        // 0x00: Receiver Buffer Register / Transmitter Holding Register
    ier: u8,            // 0x01: Interrupt Enable Register
    iir_fcr: u8,        // 0x02: Interrupt Identification Register / FIFO Control Register
    lcr: u8,            // 0x03: Line Control Register
    mcr: u8,            // 0x04: Modem Control Register
    lsr: u8,            // 0x05: Line Status Register
    msr: u8,            // 0x06: Modem Status Register
    scr: u8,            // 0x07: Scratch Register
    
    // FIFO buffers
    rx_fifo: VecDeque<u8>,
    tx_fifo: VecDeque<u8>,
    
    // Configuration
    baud_divisor: u16,
    dlab: bool,         // Divisor Latch Access Bit
}

impl SerialConsole {
    pub fn new() -> Self {
        Self {
            rbr_thr: 0,
            ier: 0,
            iir_fcr: 0,
            lcr: 0,
            mcr: 0,
            lsr: 0x60,  // Transmitter empty, no data in holding register
            msr: 0,
            scr: 0,
            rx_fifo: VecDeque::new(),
            tx_fifo: VecDeque::new(),
            baud_divisor: 1,
            dlab: false,
        }
    }
    
    // UART Register Constants
    const LSR_DATA_READY: u8 = 0x01;
    const LSR_OVERRUN_ERROR: u8 = 0x02;
    const LSR_PARITY_ERROR: u8 = 0x04;
    const LSR_FRAMING_ERROR: u8 = 0x08;
    const LSR_BREAK_INTERRUPT: u8 = 0x10;
    const LSR_TRANSMITTER_EMPTY: u8 = 0x20;
    const LSR_TRANSMITTER_IDLE: u8 = 0x40;
    const LSR_FIFO_ERROR: u8 = 0x80;
    
    const IER_RECEIVED_DATA: u8 = 0x01;
    const IER_TRANSMITTER_EMPTY: u8 = 0x02;
    const IER_RECEIVER_LINE_STATUS: u8 = 0x04;
    const IER_MODEM_STATUS: u8 = 0x08;
    
    const IIR_NO_INTERRUPT: u8 = 0x01;
    const IIR_RECEIVED_DATA: u8 = 0x04;
    const IIR_TRANSMITTER_EMPTY: u8 = 0x02;
    const IIR_RECEIVER_LINE_STATUS: u8 = 0x06;
    const IIR_MODEM_STATUS: u8 = 0x00;
    
    const LCR_DLAB: u8 = 0x80;
    
    fn update_lsr(&mut self) {
        self.lsr = 0;
        
        // Data ready in receiver
        if !self.rx_fifo.is_empty() {
            self.lsr |= Self::LSR_DATA_READY;
        }
        
        // Transmitter empty
        if self.tx_fifo.is_empty() {
            self.lsr |= Self::LSR_TRANSMITTER_EMPTY;
        }
        
        // Transmitter idle (always true for our simple implementation)
        self.lsr |= Self::LSR_TRANSMITTER_IDLE;
    }
    
    fn update_iir(&mut self) {
        if !self.rx_fifo.is_empty() && (self.ier & Self::IER_RECEIVED_DATA) != 0 {
            self.iir_fcr = Self::IIR_RECEIVED_DATA;
        } else if self.tx_fifo.is_empty() && (self.ier & Self::IER_TRANSMITTER_EMPTY) != 0 {
            self.iir_fcr = Self::IIR_TRANSMITTER_EMPTY;
        } else {
            self.iir_fcr = Self::IIR_NO_INTERRUPT;
        }
    }
    
    fn put_char(&mut self, ch: u8) {
        // For now, just print to stdout
        print!("{}", ch as char);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }
    
    /// Inject a character into the RX FIFO (for host input simulation)
    pub fn inject_char(&mut self, ch: u8) {
        if self.rx_fifo.len() < 16 { // Simple FIFO limit
            self.rx_fifo.push_back(ch);
            self.update_lsr();
            self.update_iir();
        }
    }
    
    /// Consume a character from RX FIFO (called after reading RBR)
    pub fn consume_rx_char(&mut self) -> Option<u8> {
        let ch = self.rx_fifo.pop_front();
        self.update_lsr();
        self.update_iir();
        ch
    }
    
    /// Check if there's data available in RX FIFO
    pub fn has_rx_data(&self) -> bool {
        !self.rx_fifo.is_empty()
    }
}

impl MmioDevice for SerialConsole {
    fn read(&self, offset: u64, _size: u8) -> Result<u64> {
        match offset {
            0 => {
                // RBR (Receiver Buffer Register) or DLL (Divisor Latch Low)
                if self.dlab {
                    Ok((self.baud_divisor & 0xFF) as u64)
                } else {
                    // Return data from RX FIFO (don't remove it here)
                    Ok(self.rx_fifo.front().copied().unwrap_or(0) as u64)
                }
            }
            1 => {
                // IER (Interrupt Enable Register) or DLM (Divisor Latch High)
                if self.dlab {
                    Ok(((self.baud_divisor >> 8) & 0xFF) as u64)
                } else {
                    Ok(self.ier as u64)
                }
            }
            2 => Ok(self.iir_fcr as u64), // IIR (Interrupt Identification Register)
            3 => Ok(self.lcr as u64),     // LCR (Line Control Register)
            4 => Ok(self.mcr as u64),     // MCR (Modem Control Register)
            5 => Ok(self.lsr as u64),     // LSR (Line Status Register)
            6 => Ok(self.msr as u64),     // MSR (Modem Status Register)
            7 => Ok(self.scr as u64),     // SCR (Scratch Register)
            _ => Ok(0),
        }
    }

    fn write(&mut self, offset: u64, value: u64, _size: u8) -> Result<()> {
        let val = (value & 0xFF) as u8;
        
        match offset {
            0 => {
                // THR (Transmitter Holding Register) or DLL (Divisor Latch Low)
                if self.dlab {
                    self.baud_divisor = (self.baud_divisor & 0xFF00) | (val as u16);
                } else {
                    // Write to transmitter
                    self.tx_fifo.push_back(val);
                    self.put_char(val);
                }
            }
            1 => {
                // IER (Interrupt Enable Register) or DLM (Divisor Latch High)
                if self.dlab {
                    self.baud_divisor = (self.baud_divisor & 0x00FF) | ((val as u16) << 8);
                } else {
                    self.ier = val;
                }
            }
            2 => {
                // FCR (FIFO Control Register)
                self.iir_fcr = val;
                // Handle FIFO control (simplified - just acknowledge)
            }
            3 => {
                // LCR (Line Control Register)
                self.lcr = val;
                self.dlab = (val & Self::LCR_DLAB) != 0;
            }
            4 => {
                // MCR (Modem Control Register)
                self.mcr = val;
            }
            5 => {
                // LSR (Line Status Register) - read only, ignore writes
            }
            6 => {
                // MSR (Modem Status Register) - read only, ignore writes
            }
            7 => {
                // SCR (Scratch Register)
                self.scr = val;
            }
            _ => {}
        }
        
        // Update status registers after any write
        self.update_lsr();
        self.update_iir();
        
        Ok(())
    }

    fn name(&self) -> &str {
        "SerialConsole"
    }

    fn size(&self) -> u64 {
        0x10 // 16 bytes for UART registers
    }
}
 