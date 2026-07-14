mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerMenuItem;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::HelpDialog;
use dioxus::prelude::*;
use presentation::{BurgerHelpItemPresentation, use_burger_help_item};
use style::CLASS;
use tw_macro::assert_component;

/// The compact-layout help action: the drawer row that opens the onboarding guide, and the
/// guide dialog it owns, mounted beneath it. It owns the guide's open signal locally, so the
/// dialog is part of the row and travels with it. Tapping it leaves the drawer open (closing
/// the drawer would unmount this row and its dialog), so the guide opens over the drawer.
#[component]
pub fn BurgerHelpItem() -> Element {
    let BurgerHelpItemPresentation {
        icon,
        label,
        state,
        role,
        aria_haspopup,
        aria_expanded,
        open,
        onclick,
        on_open_change,
    } = use_burger_help_item();
    rsx! {
        div {
            class: CLASS,
            BurgerMenuItem {
                icon,
                label,
                state,
                disabled: false,
                role,
                aria_haspopup,
                aria_expanded,
                aria_pressed: None,
                aria_label: None,
                onclick,
            }
            HelpDialog {
                open,
                on_open_change,
            }
        }
    }
}

assert_component!(BurgerHelpItem);
