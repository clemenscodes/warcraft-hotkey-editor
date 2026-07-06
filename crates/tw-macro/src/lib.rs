//! Compile-time assembly of a component's Tailwind class list, with a
//! consumer-declared responsive band vocabulary.
//!
//! A consumer declares its bands once with [`define_styling!`], which generates
//! the keyed `classes!` and `states!` macros bound to that vocabulary. A
//! component then writes one keyed list per band it uses, inline, in its own
//! `style.rs`; `classes!` joins them into a single `pub(super) const CLASS` at
//! compile time. The caller never writes the identity class: the macro derives
//! it from the component directory (`module_path!()` ends in `…::<dir>::style`).
//!
//! Enforced, all at compile time:
//! - **The identity is derived, not given** (`help_top_row` → `help-top-row`),
//!   so it can never drift from `dir == component == class`.
//! - **`base` is always-on** and may carry no declared band prefix; a width
//!   style like `mobile:flex` in `base` is a compile error, while variant
//!   prefixes like `after:`/`hover:` are fine.
//! - **Every band key names a declared band** — a typo (`moble:`) is a compile
//!   error.
//! - **Every class carries its band's prefix** (`uhd:flex` can never land under
//!   `mobile:`).
//!
//! `CLASS` is a [`ClassList`], not a `&str`: it can be handed to a `class:`
//! attribute but implements neither `Display` nor `Debug` nor a public
//! accessor, so it can never be string-interpolated (`class: "{CLASS} other"`
//! does not compile) and its inner string cannot be extracted. Styling coupling
//! between components is therefore impossible to express.
//!
//! The `dioxus` feature adds `ClassList`'s `IntoAttributeValue` impl and the
//! `to_library_class` bridge; without it the crate is a pure, native-testable
//! engine usable by any framework.

mod class_list;
#[macro_use]
mod macros;
mod tailwind_class;

#[doc(hidden)]
pub mod internal;

pub use class_list::ClassList;
pub use tailwind_class::TailwindClass;

#[cfg(test)]
mod tests;
