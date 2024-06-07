//! # What is Refcon?
//!
//! This crate provides the [`Refcon`] enum, which is used to wrap either a
//! concrete instance of a type `T`, or a reference to one.
//!
//! By implementing [`AsRef<T>`], [`Refcon`] lets either variant seamlessly
//! behave as a reference to `T`.
//!
//! ```rust
//! const ZERO = ABigStruct::costly_initializer();
//! let v: Vec<ABigStruct> = (0..10_000)
//!     .map(|i|
//!          if i < 1_000 {
//!              Refcon::from(&ZERO)
//!          } else {
//!              Refcon::from(ABigStruct::new(i))
//!          })
//!     .collect();
//!
//! // [...]
//!
//! for x in {
//!   x.do_something();
//! }
//! ```
//!
//! ## When **not** to use Refcon
//!   - `T` is smaller than two machine words – you should just copy everything
//!   in this case anyways;
//!   - you can not afford/do not want to pay for the double indirection price
//!   while accessing the inner value;
//!   - you do not want to pay for the price of the additional memory copy while
//!   constructing the concrete variant;
//!
//! ## When to use Refcon
//!   - You want to mix values & references: _e.g._ if you have a vector
//!   containing either reference to pre-computed values or ad-hoc ones or if an
//!   iterator in a trait may iterate over immediate or reference values
//!   depending on the implementation.

#[cfg(test)]
mod tests;

pub enum Refcon<'a, T> {
    Ref(&'a T),
    Concrete(T),
}

impl<'a, T> Refcon<'a, T> {
    /// Return true if the wrapped value is a reference to `T`.
    pub fn is_ref(&self) -> bool {
        matches!(self, Refcon::Ref(_))
    }

    /// Return true if the wrapped value is a concrete instance of `T`.
    pub fn is_concrete(&self) -> bool {
        matches!(self, Refcon::Concrete(_))
    }
}

impl<T> AsRef<T> for Refcon<'_, T> {
    fn as_ref(&self) -> &T {
        match self {
            Refcon::Concrete(t) => t,
            Refcon::Ref(t) => t,
        }
    }
}

impl<T> From<T> for Refcon<'_, T> {
    fn from(value: T) -> Self {
        Refcon::Concrete(value)
    }
}

impl<'a, T> From<&'a T> for Refcon<'a, T> {
    fn from(value: &'a T) -> Self {
        Refcon::Ref(value)
    }
}
