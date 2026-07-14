mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::presentation::{use_toolbar_actions, ToolbarActionKind};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The inline download button. It reads its action from the shared toolbar-action set and
/// hides itself until a file is loaded; clicking flips the shared download-info signal.
#[component]
pub fn ExportButton() -> Element {
    let actions = use_toolbar_actions();
    let action = actions.get(ToolbarActionKind::Download);
    if action.hidden {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
            ToolbarButton {
                icon: action.icon,
                aria_label: action.aria_label,
                onclick: action.onclick,
            }
        }
    }
}

assert_component!(ExportButton);
