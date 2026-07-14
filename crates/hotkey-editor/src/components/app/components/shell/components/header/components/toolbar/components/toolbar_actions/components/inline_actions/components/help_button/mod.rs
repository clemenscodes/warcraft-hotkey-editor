mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::presentation::{use_toolbar_actions, ToolbarActionKind};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// One inline file-action button. It reads its action from the shared toolbar-action set
/// and renders the button; the action's icon, label, and behaviour live once in that set.
#[component]
pub fn HelpButton() -> Element {
    let actions = use_toolbar_actions();
    let action = actions.get(ToolbarActionKind::Help);
    rsx! {
        div {
            class: CLASS,
            ToolbarButton {
                icon: action.icon,
                aria_label: action.aria_label,
                disabled: action.disabled,
                aria_haspopup: action.aria_haspopup,
                aria_expanded: action.expanded,
                aria_pressed: action.pressed,
                onclick: action.onclick,
            }
        }
    }
}

assert_component!(HelpButton);
