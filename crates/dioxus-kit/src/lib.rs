//! General-purpose, app-agnostic utilities for Dioxus web apps.
//!
//! Each concern is its own module. Currently:
//! - [`toast`] — a framework-level toast queue (a context-provided dispatch
//!   handle, a bounded queue, and a render model; bring your own visuals).

pub mod toast;
