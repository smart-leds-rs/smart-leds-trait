#![no_std]

#[derive(Copy, Clone, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<(u8, u8, u8)> for Color {
    fn from(color_tuple: (u8, u8, u8)) -> Self {
        Self {
            r: color_tuple.0,
            g: color_tuple.1,
            b: color_tuple.2,
        }
    }
}
pub trait SmartLedsWrite {
    type Error;
    fn write<T>(&mut self, iterator: T) -> Result<(), Self::Error>
    where
        T: Iterator<Item = Color>;
}
