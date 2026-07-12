pub mod components;
mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::help_footer_host::HelpFooterHostView;
use components::help_guide_host::HelpGuideHostView;
use dioxus::prelude::*;
use presentation::{HelpDialogHostModel, use_help_dialog_host};
use style::CLASS;
use tw_macro::assert_component;

/// Connects the onboarding help dialog to app state and places it in the always-mounted
/// toolbar, so it opens from either the inline help button or the burger drawer (and
/// auto-opens on first visit). It renders the reusable `WarcraftDialog` directly, handing it
/// the isolated guide body region and the pinned footer region; the headless dialog gates
/// itself on the shared open signal.
#[component]
pub fn HelpDialogHost() -> Element {
    let HelpDialogHostModel {
        open,
        on_open_change,
    } = use_help_dialog_host();
    let body = HelpGuideHostView;
    let footer = HelpFooterHostView;
    rsx! {
        div {
            class: CLASS,
            WarcraftDialog::<HelpGuideHostView, HelpFooterHostView> {
                title: data::TITLE,
                body,
                footer,
                open,
                on_open_change,
            }
        }
    }
}

assert_component!(HelpDialogHost);
