//! Framework-agnostic browser-platform primitives for Rust web apps, native-testable.
//!
//! - [`storage`] — a keyed `localStorage` wrapper and string codecs.
//! - [`dom`] — blob download, file-input picker, and roving focus.
//!
//! No framework dependency: usable from any Rust web app (Dioxus, Leptos, Yew, …)
//! or none. Every browser entry point is a no-op on non-wasm targets, so a
//! consuming app's logic stays `cargo test`-able off the browser.

pub mod dom;
pub mod storage;
