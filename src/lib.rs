//! # Smart Leds Trait
//!
//! Smart leds is a collection of crates to use smart leds on embedded devices with rust.
//!
//! Examples of smart leds include the popular WS2812 (also called Neopixel),
//! APA102 (DotStar) and other leds, which can be individually adressed.
//!
//! This crate is used as a common base, so that breaking changes which would
//! force all other crates to be updated, can be avoided.
//!
//! End users should use the [smart-leds](https://crates.io/crates/smart-leds)
//! crate, which contains various convenience functions.
#![no_std]

pub use {rgb::RGB, rgb::RGB16, rgb::RGB8, rgb::RGBA};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct White<C>(pub C);

/// The RGBW Pixel
///
/// This is used for leds, that in addition to RGB leds also contain a white led
pub type RGBW<ComponentType, WhiteComponentType = ComponentType> =
    RGBA<ComponentType, White<WhiteComponentType>>;

/// A trait that smart led drivers implement
///
/// The amount of time each iteration of `iterator` might take is undefined.
/// Drivers, where this might lead to issues, aren't expected to work in all cases.
pub trait SmartLedsWrite {
    type Error;
    type Color;
    fn write<T, I>(&mut self, iterator: T) -> Result<(), Self::Error>
    where
        T: Iterator<Item = I>,
        I: Into<Self::Color>;
}
