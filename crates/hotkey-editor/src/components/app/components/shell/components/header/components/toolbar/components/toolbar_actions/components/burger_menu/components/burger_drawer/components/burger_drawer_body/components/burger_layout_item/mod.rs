mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerMenuItem;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::grid_layout_editor_dialog::GridLayoutEditorDialog;
use dioxus::prelude::*;
use presentation::{BurgerLayoutItemPresentation, use_burger_layout_item};
use style::CLASS;
use tw_macro::assert_component;

/// The compact-layout grid-layout action: the primary drawer row that opens the global hotkey
/// layout editor, and the editor dialog it owns, mounted beneath it. Burger-only, since the
/// full header carries the centered grid-layout button instead. It owns the editor's open
/// signal locally, so the dialog is part of the row and travels with it. Tapping it leaves the
/// drawer open (closing it would unmount this row and its dialog), so the editor opens over the
/// drawer.
#[component]
pub fn BurgerLayoutItem() -> Element {
    let BurgerLayoutItemPresentation {
        icon,
        label,
        state,
        aria_haspopup,
        aria_expanded,
        aria_label,
        open,
        onclick,
        on_open_change,
    } = use_burger_layout_item();
    rsx! {
        div {
            class: CLASS,
            BurgerMenuItem {
                icon,
                label,
                state,
                disabled: false,
                role: None,
                aria_haspopup,
                aria_expanded,
                aria_pressed: None,
                aria_label,
                onclick,
            }
            GridLayoutEditorDialog {
                open,
                on_open_change,
            }
        }
    }
}

assert_component!(BurgerLayoutItem);
