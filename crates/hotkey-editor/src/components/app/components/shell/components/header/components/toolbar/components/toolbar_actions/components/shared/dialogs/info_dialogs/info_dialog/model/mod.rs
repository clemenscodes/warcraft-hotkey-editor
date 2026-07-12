use dioxus::prelude::*;

/// The caller-facing configuration every info dialog fills in: the open value and
/// the change handler mirroring the headless dialog's own close, the header title,
/// the centered intro line, an optional warning callout, and the primary action's
/// label and handler, plus the cancel handler. The base `InfoDialog` composes the
/// shell from these; variants build them from their own data.
#[derive(Props, Clone, PartialEq)]
pub struct InfoDialogConfig {
    pub open: bool,
    pub on_open_change: Callback<bool>,
    pub title: &'static str,
    pub intro: &'static str,
    pub warning: Option<&'static str>,
    pub primary_label: &'static str,
    pub on_primary: EventHandler<MouseEvent>,
    pub on_cancel: EventHandler<MouseEvent>,
}
