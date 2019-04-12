#![no_std]

pub use {rgb::RGB, rgb::RGB16, rgb::RGB8, rgb::RGBA};

pub struct White<C>(pub C);

pub type RGBW<ComponentType, WhiteComponentType = ComponentType> =
    RGBA<ComponentType, White<WhiteComponentType>>;

pub trait SmartLedsWrite {
    type Error;
    type Color;
    fn write<T, I>(&mut self, iterator: T) -> Result<(), Self::Error>
    where
        T: Iterator<Item = I>,
        I: Into<Self::Color>;
}
