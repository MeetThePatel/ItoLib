//! Strongly typed floating-point numbers.
//!
//! Some of the types included in this module are:
//! - [`NonNegativeFloat`]: $\overline{\mathbb{R}}_+ \coloneqq [0, \infty) \cup \lbrace \infty
//!   \rbrace$
//! - [`NonNegativeFiniteFloat`]: $\mathbb{R}_+ \coloneqq [0, \infty)$
//! - [`PositiveFloat`]: $\overline{\mathbb{R}}^*_+ \coloneqq (0, \infty) \cup \lbrace \infty
//!   \rbrace$
//! - [`PositiveFiniteFloat`]: $\overline{\mathbb{R}}^*_+ \coloneqq (0, \infty) \cup \lbrace \infty
//!   \rbrace$

pub mod macros;

mod into_float;
pub use into_float::IntoFloat;

mod finite_float;
pub use finite_float::FiniteFloat;

mod nonnegative_float;
pub use nonnegative_float::NonNegativeFloat;

mod nonnegative_finite_float;
pub use nonnegative_finite_float::NonNegativeFiniteFloat;

mod positive_float;
pub use positive_float::PositiveFloat;

mod positive_finite_float;
pub use positive_finite_float::PositiveFiniteFloat;

/// Error type for typed float conversions.
pub struct ConversionDomainError;

// TODO: Create casting for restricting domain types. For example, FiniteFloat should trivially cast
//       without bounds checking to PositiveFloat.
