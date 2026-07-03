pub mod components;
mod data;
mod logic;
mod props;

use crate::components::dialogs::dialog::{Dialog, DialogProps};
use dioxus::prelude::*;
pub use props::InfoDialogConfig;

/// The shared shell for the download and import info dialogs: a centered
/// instruction block above a cancel/primary action row. Variants fill in the
/// title, copy, warning, and handlers via `InfoDialogConfig`; this composes the
/// `Dialog` base from them and adds no markup of its own.
#[component]
pub fn InfoDialog(props: InfoDialogConfig) -> Element {
    rsx! {
        Dialog { ..DialogProps::from(&props) }
    }
}
