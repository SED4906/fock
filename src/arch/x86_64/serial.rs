use x86::io::{inb, outb};

const COM1: u16 = 0x3f8;

/// Initializes the COM1 serial port.
/// Silently returns if the port doesn't work.
///
/// # Safety
///
/// Uses port I/O but shouldn't cause problems.
pub unsafe fn serial_init() {
    outb(COM1 + 1, 0);
    outb(COM1 + 3, 0x80);
    outb(COM1, 0x03);
    outb(COM1 + 1, 0);
    outb(COM1 + 3, 0x03);
    outb(COM1 + 2, 0xC7);
    outb(COM1 + 4, 0x03);
    outb(COM1 + 4, 0x1E);
    outb(COM1, 0xAE);
    if inb(COM1) != 0xAE {
        return;
    }
    outb(COM1 + 4, 0x03);
}

/// Sends a byte to COM1.
///
/// # Safety
///
/// Uses port I/O but shouldn't cause problems.
pub unsafe fn serial_send(byte: u8) {
    while inb(COM1 + 5) & 0x20 == 0 {}
    outb(COM1, byte);
}

pub unsafe fn serial_recv() -> u8 {
    while inb(COM1 + 5) & 1 == 0 {}
    let result = inb(COM1);
    serial_send(result);
    result
}

use core::fmt;
use spin::Mutex;

pub struct SerialWriter {}
static SERIAL_WRITER: Mutex<SerialWriter> = Mutex::new(SerialWriter {});

impl fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.as_bytes() {
            unsafe {
                serial_send(*c);
            }
        }
        Ok(())
    }
}

pub fn _serial_print(args: fmt::Arguments) {
    // NOTE: Locking needs to happen around `print_fmt`, not `print_str`, as the former
    // will call the latter potentially multiple times per invocation.
    let mut writer = SERIAL_WRITER.lock();
    fmt::Write::write_fmt(&mut *writer, args).ok();
}

#[macro_export]
macro_rules! print {
    ($($t:tt)*) => { $crate::serial::_serial_print(format_args!($($t)*)) };
}

#[macro_export]
macro_rules! println {
    ()          => { $crate::print!("\n"); };
    // On nightly, `format_args_nl!` could also be used.
    ($($t:tt)*) => { $crate::print!("{}\n", format_args!($($t)*)) };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_println {
    ()          => { $crate::print!("\n"); };
    // On nightly, `format_args_nl!` could also be used.
    ($($t:tt)*) => { $crate::print!("{}\n", format_args!($($t)*)) };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_println {
    () => {};
    ($($t:tt)*) => {};
}
