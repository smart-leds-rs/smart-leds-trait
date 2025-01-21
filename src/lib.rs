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

/// A trait that Smart Led Drivers implement
///
/// The amount of time each iteration of `iterator` might take is undefined.
/// Drivers, where this might lead to issues, aren't expected to work in all cases.
pub trait SmartLedsWrite {
    type Error;
    type Color;
    fn write<T, I>(&mut self, iterator: T) -> Result<(), Self::Error>
    where
        T: IntoIterator<Item = I>,
        I: Into<Self::Color>;
}

/// An async trait that Smart Led Drivers implement
///
/// The amount of time each iteration of `iterator` might take is undefined.
/// Drivers, where this might lead to issues, aren't expected to work in all cases.
pub trait SmartLedsWriteAsync {
    type Error;
    type Color;
    // The async_fn_in_trait warning doesn't really matter for embedded cases because
    // no_std async executors don't require futures to be Send. Also, embedded-hal-async
    // does not have Send bounds in its traits, so the HAL functions called in
    // implementations of this trait wouldn't return Send futures anyway. It's
    // questionable if it would be desirable for embedded HALs to return a Send future
    // for the write function for a peripheral because you probably don't want to
    // write data to the same peripheral from multiple threads simultaneously and have
    // the data get interleaved, nor have the embedded HAL implement a synchronization
    // mechanism with a run time cost to avoid that.
    // https://github.com/rust-embedded/embedded-hal/pull/515#issuecomment-1763525962
    #[allow(async_fn_in_trait)]
    async fn write<T, I>(&mut self, iterator: T) -> Result<(), Self::Error>
    where
        T: IntoIterator<Item = I>,
        I: Into<Self::Color>;
}
