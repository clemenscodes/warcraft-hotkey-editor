pub mod components;
mod data;
mod model;
mod presentation;
mod view;

pub use view::HelpDialogView;

use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::help_footer::HelpFooterView;
use components::help_guide::HelpGuideView;
use dioxus::prelude::*;
use model::HelpDialogModel;
use presentation::HelpDialogPresentation;
use tw_macro::assert_component;

/// The onboarding help guide, opened by the trigger that owns its open signal (the inline
/// help button or the burger drawer, each with its own instance). It mounts the reusable
/// `WarcraftDialog` only while `open` is set — the trigger's signal is the switch that puts
/// the dialog in the DOM — handing it the isolated guide body region and the pinned dismiss
/// footer region, whose dismiss records the onboarding as seen before closing.
#[component]
pub fn HelpDialog(props: HelpDialogModel) -> Element {
    let HelpDialogPresentation {
        open,
        on_open_change,
        on_dismiss,
    } = HelpDialogPresentation::from(&props);
    let body = HelpGuideView;
    let footer = HelpFooterView { on_dismiss };
    rsx! {
        if open {
            WarcraftDialog::<HelpGuideView,HelpFooterView> {
                title: data::TITLE,
                body,
                footer,
                open: true,
                on_open_change,
            }
        }
    }
}

assert_component!(HelpDialog);
