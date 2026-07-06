//! A framework-level toast queue for Dioxus — logic only, bring your own visuals.
//!
//! [`use_toast_provider`] owns a bounded queue and provides a copyable
//! [`Toasts`] dispatch handle as context; any descendant calls [`use_toast`]
//! (or [`consume_toast`] off hook position) to raise a toast. The provider hook
//! returns a [`ToastProviderModel`] — the current [`ToastRecord`]s plus a remove
//! callback — which you render however you like. No visual components, no CSS,
//! no domain types: this module is the state machine.

mod context;
mod hooks;

pub use context::{ToastOptions, ToastRecord, ToastType, Toasts, consume_toast, use_toast};
pub use hooks::{ToastProviderModel, use_toast_provider};
