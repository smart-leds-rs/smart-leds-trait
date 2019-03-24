#![no_std]

pub type Color = rgb::RGB8;

pub trait SmartLedsWrite {
    type Error;
    fn write<T>(&mut self, iterator: T) -> Result<(), Self::Error>
    where
        T: Iterator<Item = Color>;
}
