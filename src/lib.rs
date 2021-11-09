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
}
impl<T: Sized> Somok for T {}
