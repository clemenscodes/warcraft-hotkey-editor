mod data;
mod model;
mod presentation;
mod style;
mod view;

pub use view::BurgerRedoItemView;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerMenuItem;
use dioxus::prelude::*;
use model::BurgerRedoItemModel;
use presentation::{BurgerRedoItemPresentation, use_burger_redo_item};
use style::CLASS;
use tw_macro::assert_component;

/// The compact-layout redo action: the drawer row that redoes one edit and then closes the
/// drawer. Unlike the help row it mounts no dialog — it performs the redo command and dismisses
/// the menu.
#[component]
pub fn BurgerRedoItem(props: BurgerRedoItemModel) -> Element {
    let BurgerRedoItemPresentation {
        icon,
        label,
        state,
        disabled,
        role,
        onclick,
    } = use_burger_redo_item(&props);
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

assert_component!(BurgerRedoItem);
