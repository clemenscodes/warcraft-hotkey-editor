//! Compile-time assembly of a component's Tailwind class list.
//!
//! A component declares one utility list per responsive band as locked consts in
//! its own `style.rs`, then `classes!` joins them into a single
//! `pub(super) const CLASS: &str` at compile time. The caller never writes the
//! identity class: the macro derives it from the component directory.
//!
//! Enforced, all at compile time:
//! - **The identity is derived, not given.** `module_path!()` ends in
//!   `…::<dir>::style`, so the macro reads the component directory and emits the
//!   kebab identity (`help_top_row` → `help-top-row`). It can never drift from
//!   `dir == component == class`, and the caller can never get it wrong.
//! - **Every band is present.** The arity is fixed to the six app bands, so a
//!   component can never silently miss `MOBILE` or `UHD`; an unused band is an
//!   explicit empty `&[]`.
//! - **Every band const is named correctly** (`MOBILE`, `TABLET`, `LAPTOP`,
//!   `DESKTOP`, `QHD`, `UHD`), so `grep MOBILE` finds every component's mobile
//!   styles.
//! - **Every class carries its band's prefix** (`uhd:flex` can never land in
//!   `MOBILE`).
//!
//! `pub(super)` keeps `CLASS` visible only to the component's own module: with
//! the private `mod style;`, no other component can name the path, so styles
//! never leak across components. Every class stays a literal in the band arrays,
//! so Tailwind's source scanner still sees every token.
//!
//! `CLASS` is a [`ClassList`], not a `&str`: it can be handed to a `class:`
//! attribute but deliberately implements neither `Display` nor a public
//! accessor, so it can never be string-interpolated (`class: "{CLASS} other"`
//! does not compile) and its inner string cannot be extracted. A component can
//! only ever wear exactly its own `CLASS` — never another component's class, and
//! nothing can be appended — so styling coupling between components is
//! impossible to express.
//!
//! The pieces are split across this module:
//! - [`class_list`] and [`tailwind_class`] hold the two opaque types.
//! - [`compile_checks`] holds the `const fn` machinery the macros expand into.
//! - [`macros`] defines `tw!`, `classes!`, `states!` and `assert_component!`,
//!   each `#[macro_export]`ed to the crate root.

mod class_list;
mod compile_checks;
mod macros;
mod tailwind_class;

pub use class_list::ClassList;
pub use compile_checks::{
    assert_band, assert_base, assert_component_name, assert_flat, assert_named, build_identity,
    identity_len, join_into, joined_len,
};
pub use tailwind_class::TailwindClass;
