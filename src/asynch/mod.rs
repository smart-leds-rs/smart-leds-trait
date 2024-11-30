use core::future::Future;

/// An async trait that smart led drivers implement
///
/// The amount of time each iteration of `iterator` might take is undefined.
/// Drivers, where this might lead to issues, aren't expected to work in all cases.
pub trait SmartLedsWrite {
    type Error;
    type Color;
    fn write<T, I>(&mut self, iterator: T) -> impl Future<Output = Result<(), Self::Error>>
    where
        T: IntoIterator<Item = I>,
        I: Into<Self::Color>;
}
