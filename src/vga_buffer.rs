use spin::Mutex;
use volatile::Volatile;

use lazy_static::lazy_static;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

lazy_static! {
    pub static ref WRITER:Mutex<Writer> =Mutex::new(Writer{
        column_position:0,
        color_code:ColorCode::new(Color::LightGreen,Color::Black),
        buffer:unsafe{&mut *(0xb8000 as *mut Buffer)}
    });
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    // a two-dimension matrix representing the screen
    // direct memory access to the buffer is protected by a spin-lock
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct Writer {
    column_position: usize,
    // where next byte will be printed to
    color_code: ColorCode,
    // code indicating terminal text color
    buffer: &'static mut Buffer, // vga buffer
}

impl Writer {
    pub fn write_string(&mut self, str: &str) {
        for byte in str.bytes() {
            if (byte >= 0x20 && byte <= 0x7e) || byte == b'\n' {
                self.write_byte(byte)
            } else {
                self.write_byte(0xfe) // unsupported non-ascii chars
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line()
                }

                let row = BUFFER_HEIGHT - 1;
                let column = self.column_position;
                self.buffer.chars[row][column].write(ScreenChar {
                    ascii_char: byte,
                    color_code: self.color_code,
                });
                self.column_position += 1
            }
        }
    }

    // new_line moves the whole lines upwards to create one new line at the bottom of the screen.
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for column in 0..BUFFER_WIDTH {
                let byte = self.buffer.chars[row][column].read();
                self.buffer.chars[row - 1][column].write(byte);
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1); // clear the bottom line
        self.column_position = 0; // reset column position
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ', // fill a 'space' rather than 0x00 will clear this position
            color_code: self.color_code,
        };
        for i in 0..BUFFER_WIDTH {
            self.buffer.chars[row][i].write(blank)
        }
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
struct ColorCode(u8); // ColorCode indicating the foreground and background text color

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
struct ScreenChar {
    // a colored ascii character that can be printed to the screen
    ascii_char: u8,
    color_code: ColorCode,
}