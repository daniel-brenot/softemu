use crate::memory::mmio::MmioDevice;
use crate::Result;

/// Console device for text output
pub struct ConsoleDevice {
    buffer: Vec<u8>,
    cursor_pos: usize,
    width: usize,
    height: usize,
}

impl ConsoleDevice {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            cursor_pos: 0,
            width,
            height,
        }
    }

    fn put_char(&mut self, ch: u8) {
        if ch == b'\n' {
            // Move to next line
            let current_line = self.cursor_pos / self.width;
            if current_line < self.height - 1 {
                self.cursor_pos = (current_line + 1) * self.width;
            } else {
                // Scroll up
                self.scroll_up();
                self.cursor_pos = (self.height - 1) * self.width;
            }
        } else if ch == b'\r' {
            // Carriage return - move to beginning of line
            let current_line = self.cursor_pos / self.width;
            self.cursor_pos = current_line * self.width;
        } else if ch == b'\t' {
            // Tab - move to next tab stop
            let tab_stop = 8;
            let current_col = self.cursor_pos % self.width;
            let next_tab = ((current_col / tab_stop) + 1) * tab_stop;
            if next_tab < self.width {
                self.cursor_pos = (self.cursor_pos / self.width) * self.width + next_tab;
            }
        } else if ch >= 32 && ch <= 126 {
            // Printable character
            if self.cursor_pos < self.buffer.len() {
                self.buffer[self.cursor_pos] = ch;
                self.cursor_pos += 1;
                
                // Wrap to next line if needed
                if self.cursor_pos % self.width == 0 && self.cursor_pos < self.buffer.len() {
                    // Already at end of line, move to next line
                } else if self.cursor_pos >= self.buffer.len() {
                    // End of buffer, scroll up
                    self.scroll_up();
                    self.cursor_pos = (self.height - 1) * self.width;
                }
            }
        }
    }

    fn scroll_up(&mut self) {
        // Move all lines up by one
        for y in 0..(self.height - 1) {
            for x in 0..self.width {
                let src_idx = (y + 1) * self.width + x;
                let dst_idx = y * self.width + x;
                self.buffer[dst_idx] = self.buffer[src_idx];
            }
        }
        
        // Clear the last line
        let last_line_start = (self.height - 1) * self.width;
        for x in 0..self.width {
            self.buffer[last_line_start + x] = 0;
        }
    }

    fn clear_screen(&mut self) {
        self.buffer.fill(0);
        self.cursor_pos = 0;
    }

    fn get_char_at(&self, x: usize, y: usize) -> u8 {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x]
        } else {
            0
        }
    }

    fn set_cursor_pos(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.cursor_pos = y * self.width + x;
        }
    }
}

impl MmioDevice for ConsoleDevice {
    fn read(&self, offset: u64, _size: u8) -> Result<u64> {
        match offset {
            0 => Ok(self.cursor_pos as u64), // Cursor position
            8 => Ok(self.width as u64), // Screen width
            16 => Ok(self.height as u64), // Screen height
            24 => {
                // Read character at cursor
                if self.cursor_pos < self.buffer.len() {
                    Ok(self.buffer[self.cursor_pos] as u64)
                } else {
                    Ok(0)
                }
            }
            32 => {
                // Read character at cursor position
                if self.cursor_pos < self.buffer.len() {
                    Ok(self.buffer[self.cursor_pos] as u64)
                } else {
                    Ok(0)
                }
            }
            _ => Ok(0),
        }
    }

    fn write(&mut self, offset: u64, value: u64, _size: u8) -> Result<()> {
        match offset {
            0 => {
                // Set cursor position
                let pos = value as usize;
                if pos < self.buffer.len() {
                    self.cursor_pos = pos;
                }
                Ok(())
            }
            8 => {
                // Write character
                let ch = (value & 0xFF) as u8;
                self.put_char(ch);
                Ok(())
            }
            16 => {
                // Clear screen
                if value != 0 {
                    self.clear_screen();
                }
                Ok(())
            }
            24 => {
                // Set cursor position (x, y)
                let x = (value & 0xFFFF) as usize;
                let y = ((value >> 16) & 0xFFFF) as usize;
                self.set_cursor_pos(x, y);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn name(&self) -> &str {
        "Console"
    }

    fn size(&self) -> u64 {
        0x1000 // 4KB
    }
}
