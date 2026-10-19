pub mod components;
mod data;
mod model;
mod presentation;
mod view;

pub use view::HelpDialogView;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_footer::DialogFooterView;
use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::help_guide::HelpGuideView;
use dioxus::prelude::*;
use model::HelpDialogModel;
use presentation::HelpDialogPresentation;
use tw_macro::assert_component;

#[component]
pub fn HelpDialog(props: HelpDialogModel) -> Element {
    let HelpDialogPresentation {
        open,
        on_open_change,
        on_dismiss,
    } = HelpDialogPresentation::from(&props);
    let body = HelpGuideView;
    let footer = DialogFooterView {
        label: data::DISMISS_LABEL,
        onclick: on_dismiss,
    };
    rsx! {
        WarcraftDialog::<HelpGuideView,DialogFooterView> {
            title: data::TITLE,
            body,
            footer,
            open,
            on_open_change,
        }
    }
}

assert_component!(HelpDialog);
