mod data;
mod model;
mod presentation;
mod style;
mod view;

pub use view::BurgerUndoItemView;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerMenuItem;
use dioxus::prelude::*;
use model::BurgerUndoItemModel;
use presentation::{BurgerUndoItemPresentation, use_burger_undo_item};
use style::CLASS;
use tw_macro::assert_component;

/// The compact-layout undo action: the drawer row that undoes one edit and then closes the
/// drawer. Unlike the help row it mounts no dialog — it performs the undo command and dismisses
/// the menu.
#[component]
pub fn BurgerUndoItem(props: BurgerUndoItemModel) -> Element {
    let BurgerUndoItemPresentation {
        icon,
        label,
        state,
        disabled,
        role,
        onclick,
    } = use_burger_undo_item(&props);
    rsx! {
        div {
            class: CLASS,
            BurgerMenuItem {
                icon,
                label,
                state,
                disabled,
                role,
                aria_haspopup: None,
                aria_expanded: None,
                aria_pressed: None,
                aria_label: None,
                onclick,
            }
        }
    }
}

assert_component!(BurgerUndoItem);
