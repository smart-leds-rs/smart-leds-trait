#![no_std]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub trait SmartLedsWrite {
    type Error;
    fn write<T>(&mut self, iterator: T) -> Result<(), Self::Error>
    where
        T: Iterator<Item = Color>;
}
