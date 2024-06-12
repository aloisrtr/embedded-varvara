//! # Console Device
//! Abstracts a console-like interface, with text input, output, and error.

use embedded_io::ErrorType;

pub struct Console<O, I, E> {
    output: O,
    input: I,
    error: E,
}
impl<O, I, E> Console<O, I, E>
where
    O: embedded_io::Write,
{
    pub fn write(&self, bytes: &[u8]) -> Result<usize, <O as ErrorType>::Error> {
        self.output.write(bytes)
    }
}
impl<O, I, E> Console<O, I, E>
where
    E: embedded_io::Write,
{
    pub fn write_error(&self, bytes: &[u8]) -> Result<usize, <E as ErrorType>::Error> {
        self.error.write(bytes)
    }
}
impl<O, I, E> Console<O, I, E>
where
    I: embedded_io::Read,
{
    pub fn read(&self, dest: &mut [u8]) -> Result<usize, <I as ErrorType>::Error> {
        self.input.read(dest)
    }

    pub fn read_exact(
        &self,
        dest: &mut [u8],
    ) -> Result<(), embedded_io::ReadExactError<<I as ErrorType>::Error>> {
        self.input.read_exact(dest)
    }
}
