use volatile::Volatile;
use core::fmt::Write;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Brown = 0x5,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xa,
    LightCyan = 0xb,
    LightRed = 0xc,
    LightMagenta = 0xd,
    Yellow = 0xe,
    White = 0xf, 
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    character: u8,
    color_code: ColorCode,
}

// vga's just like that afaik
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;


#[repr(transparent)]

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.chars[row][col].write(ScreenChar {
                    character: byte,
                    color_code: self.color_code,
                });
                self.column_position += 1;
            }
        }
    }
    fn new_line(&mut self) {
        // shift all rows up by one, have a new blank row at the bottom
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = &self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col].write(character.read());
            }
        }
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[BUFFER_HEIGHT - 1][col].write(ScreenChar {
                character: b' ',
                color_code: self.color_code,
            });
        }
        self.column_position = 0;
    }

    fn set_color(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }
    fn clear_color(&mut self) {
        self.set_color(ColorCode::new(Color::White, Color::Black));
    }

    fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[row][col].write(ScreenChar {
                    character: b' ',
                    color_code: self.color_code,
                });
            }
        }
        self.column_position = 0;
    }

    fn write_string(&mut self, s: &str) {
        for &b in s.as_bytes() {
        match b {

                0x20..=0x7e | b'\n' | b'\t' => self.write_byte(b),

                _ => self.write_byte(0x3f), // question mark
            }
        }
    }

}

use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}


// singleton
// so this looks like it's really cooked, but lazy static and spin mutex are here for a reason
// lazy static allows us to have runtime statics that still work in no_std environments
// spin mutex is just to "add safe interior mutability", whatever that means
use spin::Mutex;
use lazy_static::lazy_static;
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// copy-pasted from the tutorial
// nearly copy-pasted from the rust source
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
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}