// Copyright (c) 2017 Stefan Lankes, RWTH Aachen University
//
// MIT License
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use core::fmt;
use spin::Mutex;
use cpuio;

/// A COM serial port.
pub struct ComPort {
    /// COM ports are identified by the base address of their associated
    /// I/O registers.
    base_addr: u16
}

impl ComPort {
    /// Create a new COM port with the specified base address.
    const fn new(base_addr: u16) -> ComPort {
        ComPort { base_addr: base_addr }
    }
}

impl fmt::Write for ComPort {
    /// Output a string to our COM port.  This allows using nice,
    /// high-level tools like Rust's `write!` macro.
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
			let mut uart_port : cpuio::Port<u8> = cpuio::Port::new(self.base_addr);

            // Output each byte of our string.
            for &b in s.as_bytes() {
                // Write our byte.
                uart_port.write(b);
            }
        }
        Ok(())
    }
}

/// Our primary serial port.
pub static COM1: Mutex<ComPort> = Mutex::new(ComPort::new(0x3F8));
