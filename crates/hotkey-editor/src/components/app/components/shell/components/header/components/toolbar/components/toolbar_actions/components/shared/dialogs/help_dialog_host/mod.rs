pub mod components;
mod data;
mod presentation;

use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::help_footer_host::HelpFooterHostView;
use components::help_guide_host::HelpGuideHostView;
use dioxus::prelude::*;
use presentation::{HelpDialogHostModel, use_help_dialog_host};
use tw_macro::assert_component;

/// Connects the onboarding help dialog to app state from the always-mounted toolbar, so it
/// opens from either the inline help button or the burger drawer (and auto-opens on first
/// visit). It mounts the reusable `WarcraftDialog` only while the shared open signal is set —
/// the signal is the switch that puts the dialog in the DOM — handing it the isolated guide
/// body region and the pinned footer region.
#[component]
pub fn HelpDialogHost() -> Element {
    let HelpDialogHostModel {
        open,
        on_open_change,
    } = use_help_dialog_host();
    let body = HelpGuideHostView;
    let footer = HelpFooterHostView;
    rsx! {
        if open {
            WarcraftDialog::<HelpGuideHostView, HelpFooterHostView> {
                title: data::TITLE,
                body,
                footer,
                open: true,
                on_open_change,
            }
        }
    }
}

assert_component!(HelpDialogHost);
