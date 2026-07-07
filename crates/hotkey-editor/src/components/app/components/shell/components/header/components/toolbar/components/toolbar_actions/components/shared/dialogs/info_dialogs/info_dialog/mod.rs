pub mod components;
mod data;
mod logic;
mod props;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::dialog::{Dialog, DialogProps};
use dioxus::prelude::*;
pub use props::InfoDialogConfig;

/// The shared shell for the download and import info dialogs: a centered
/// instruction block above a cancel/primary action row. Variants fill in the
/// title, copy, warning, and handlers via `InfoDialogConfig`; this composes the
/// `Dialog` base from them and adds no markup of its own.
use tw_macro::assert_component;
assert_component!(InfoDialog);
#[component]
pub fn InfoDialog(props: InfoDialogConfig) -> Element {
    rsx! {
        Dialog { ..DialogProps::from(&props) }
    }
}
