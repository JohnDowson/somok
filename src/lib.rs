#![deny(missing_docs)]
//! # Somok - postfix Result/Option wrapping
//!
//! ## Usage:
//! Add following to your cargo toml:
//! ```toml
//! somok = "1.0"
//! ```
//! Then use postfix wrapping as follows:
//! ```rust
//! use somok::Somok;
//!
//! fn foo() -> Result<Option<String>> {
//!     String::from("Foobar").some().okay()
//! }
//! ```

/// Postfix wrapping in Result and Option
/// ### Usage:
/// ```rust
/// fn foo() -> Result<Option<String>> {
///     String::from("Foobar").some().okay()
/// }
/// ```
pub trait Somok: Sized {
    /// Returns self wrapped in Ok
    fn okay<E>(self) -> Result<Self, E> {
        Ok(self)
    }
    /// Returns self wrapped in Err
    fn error<T>(self) -> Result<T, Self> {
        Err(self)
    }
    /// Returns self wrapped in Some
    fn some(self) -> Option<Self> {
        Some(self)
    }
    /// Returns boxed self
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
    /// Returns self wrapped in `Either::Left`
    fn left<R>(self) -> Either<Self, R> {
        Either::Left(self)
    }
    /// Returns self wrapped in `Either::Right`
    fn right<L>(self) -> Either<L, Self> {
        Either::Right(self)
    }
}
impl<T: Sized> Somok for T {}

/// Postfix notation for leaking the box
pub trait Leaksome<T: ?Sized> {
    /// Leaks the box
    fn leak(self) -> &'static mut T;
}
impl<T: ?Sized> Leaksome<T> for Box<T> {
    fn leak(self) -> &'static mut T {
        Box::leak(self)
    }
}

/// Fallible `remove()` alternative for Vec
pub trait TryRemove {
    /// Type of collection item
    type Item;
    /// Attempts to `remove` item at index
    fn try_remove(&mut self, i: usize) -> Option<Self::Item>;
}
impl<T> TryRemove for Vec<T> {
    type Item = T;

    fn try_remove(&mut self, i: usize) -> Option<Self::Item> {
        if i < self.len() {
            self.remove(i).some()
        } else {
            None
        }
    }
}

/// Convinience method for conditionally popping from a collection
pub trait CondPop {
    /// Type of collection item
    type Item;
    /// Conditionally pops off an item from collection
    fn cond_pop<F>(&mut self, cond: F) -> Option<Self::Item>
    where
        F: Fn(&Self::Item) -> bool;
}
impl<T> CondPop for Vec<T> {
    type Item = T;

    fn cond_pop<F>(&mut self, cond: F) -> Option<Self::Item>
    where
        F: Fn(&Self::Item) -> bool,
    {
        if let Some(last) = self.last() {
            cond(last).then(|| self.pop()).flatten()
        } else {
            None
        }
    }
}

/// Enum representing either of two options
#[derive(Debug, Clone, Copy)]
pub enum Either<L, R> {
    /// Left variant
    Left(L),
    /// Right variant
    Right(R),
}
