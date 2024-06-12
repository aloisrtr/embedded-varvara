//! # embedded-varvara
//! A Varvara emulator build on top of [embedded-hal](https://github.com/rust-embedded/embedded-hal)
//! abstractions.

pub mod console;
pub mod screen;

use baryuxn::{machine::UxnMachine, UxnDeviceBus};
use console::Console;
use embedded_graphics_core::draw_target::DrawTarget;
use screen::{ColourPalette, Screen};

pub struct VarvaraDeviceBus<O, I, E, C> {
    storage: [u8; 0x100],

    console: Console<O, I, E>,
    screen: Screen<C>,
}
impl<O, I, E, C> VarvaraDeviceBus<O, I, E, C> {
    /// Creates a new Varvara device bus.
    pub fn new(console: Console<O, I, E>, canvas: C) -> Self {
        Self {
            storage: [0; 0x100],
            console,
            screen: Screen::new(canvas),
        }
    }

    /// Checks if the Varvara emulator received a halting command.
    pub fn should_quit(&self) -> bool {
        self.storage[0x0f] != 0
    }

    /// Returns the current colour palette of the System device.
    pub fn colour_palette(&self) -> ColourPalette {
        ColourPalette {
            colour0: ((self.storage[0x8] as u16) << 1) | (self.storage[0x9] >> 1) as u16,
            colour1: (((self.storage[0x9] & 0x1) as u16) << 2) | (self.storage[0xa] as u16),
            colour2: ((self.storage[0xa] as u16) << 1) | (self.storage[0xb] >> 1) as u16,
            colour3: (((self.storage[0xb] & 0x1) as u16) << 2) | (self.storage[0xc] as u16),
        }
    }
}
impl<T, O, I, E, C> UxnDeviceBus<T> for VarvaraDeviceBus<O, I, E, C>
where
    C: DrawTarget,
    O: embedded_io::Write,
    I: embedded_io::Read,
    E: embedded_io::Write,
{
    fn read(&mut self, machine: &mut UxnMachine<T>, address: u8) -> u8 {
        let page = address & 0xf0;
        let port = address & 0x0f;

        match page {
            0x00 => match port {
                // System
                0x04 => machine.work_stack.pointer,
                0x05 => machine.return_stack.pointer,
                _ => self.storage[address as usize],
            },
            0x20 => match port {
                // Screen
                0x02 => (self.screen.width() >> 8) as u8,
                0x03 => (self.screen.width() & 0xf) as u8,
                0x04 => (self.screen.height() >> 8) as u8,
                0x05 => (self.screen.height() & 0xf) as u8,
                0x08 => (self.screen.x >> 8) as u8,
                0x09 => (self.screen.x & 0xf) as u8,
                0x0a => (self.screen.y >> 8) as u8,
                0x0b => (self.screen.y & 0xf) as u8,
                0x0c => (self.screen.sprite_address >> 8) as u8,
                0x0d => (self.screen.sprite_address & 0xf) as u8,
                _ => self.storage[address as usize],
            },
            0x30..=0x60 => {
                // Audio
                todo!("Audio operations not implemented")
            }
            #[cfg(feature = "chrono")]
            0xc0 => {
                use chrono::prelude::*;
                // DateTime
                let time = Local::now();
                match port {
                    0x00 => ((time.year() as u16) >> 8) as u8,
                    0x01 => ((time.year() as u16) & 0x00ff) as u8,
                    0x02 => time.month0() as u8,
                    0x03 => time.day0() as u8,
                    0x04 => time.hour() as u8,
                    0x05 => time.minute() as u8,
                    0x06 => time.second() as u8,
                    0x07 => time.weekday() as u8,
                    0x08 => (time.ordinal0() >> 8) as u8,
                    0x09 => (time.ordinal() & 0xff) as u8,
                    0x0a => {
                        defmt::warn!("daytime savings are not supported");
                        self.storage[address as usize]
                    }
                    _ => self.storage[address as usize],
                }
            }
            _ => self.storage[address as usize],
        }
    }
    fn write(&mut self, machine: &mut UxnMachine<T>, address: u8, byte: u8) {
        let page = address & 0xf0;
        let port = address & 0x0f;
        self.storage[address as usize] = byte;
        // Specific actions
        match page {
            0x00 => match port {
                // System
                #[cfg(feature = "defmt")]
                0x03 => defmt::warn!("Expansions not yet implemented"),
                0x04 => machine.work_stack.pointer = byte,
                0x05 => machine.return_stack.pointer = byte,
                #[cfg(feature = "defmt")]
                0x0e if byte != 0 => {
                    // TODO: maybe check the byte and add more functionnality depending
                    // on its value?
                    defmt::debug!(
                        "WST ( {:?} )\nRST ( {:?} )",
                        machine.work_stack,
                        machine.return_stack
                    );
                }
                _ => {}
            },
            #[cfg(feature = "io")]
            0x10 => match port {
                // Console
                0x08 => {
                    self.console.write(&[byte]);
                }
                0x09 => {
                    self.console.write_error(&[byte]);
                }
                _ => {}
            },
            #[cfg(feature = "graphics")]
            0x20 => match port {
                // Screen
                0x06 => {
                    self.screen.auto_x = byte & 0x1 != 0;
                    self.screen.auto_y = byte & 0x2 != 0;
                    self.screen.auto_sprite = byte & 0x4 != 0;
                    self.screen.length = byte >> 4;
                }
                0x08 | 0x09 => {
                    self.screen.x = ((self.storage[0x28] as u16) << 8) | self.storage[0x29] as u16
                }
                0x0a | 0x0b => {
                    self.screen.y = ((self.storage[0x2a] as u16) << 8) | self.storage[0x2b] as u16
                }
                0x0c | 0x0d => {
                    self.screen.sprite_address =
                        ((self.storage[0x2c] as u16) << 8) | self.storage[0x2d] as u16
                }
                0x0e => {
                    let colour = byte & 0x3;
                    let foreground = byte & 0x40 != 0;
                    if byte & 0x80 != 0 {
                        // Fill mode
                        let (x, width) = if byte & 0x10 != 0 {
                            // Flip X
                            (0, self.screen.x)
                        } else {
                            (self.screen.x, self.screen.width() - self.screen.x)
                        };
                        let (y, height) = if byte & 0x20 != 0 {
                            // Flip Y
                            (0, self.screen.y)
                        } else {
                            (self.screen.y, self.screen.height() - self.screen.y)
                        };
                        self.screen
                            .draw_fill(foreground, x, y, width, height, colour);
                    } else {
                        // Pixel mode
                        if self.screen.x < self.screen.width()
                            && self.screen.y < self.screen.height()
                        {
                            self.screen.draw_pixel(colour);
                        }
                    }
                }
            },
            #[cfg(feature = "defmt")]
            0x30..=0x60 => {
                // Audio
                defmt::warn!("Audio operations not yet implemented")
            }
            #[cfg(feature = "defmt")]
            0xa0..=0xb0 => {
                // File
                defmt::warn!("File operations not yet implemented")
            }
            _ => {}
        }
    }
}
