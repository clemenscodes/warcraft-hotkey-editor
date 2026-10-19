mod data;
mod model;
mod presentation;
mod style;
mod view;

pub use view::BurgerResolveItemView;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerMenuItem;
use dioxus::prelude::*;
use model::BurgerResolveItemModel;
use presentation::{BurgerResolveItemPresentation, use_burger_resolve_item};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BurgerResolveItem(props: BurgerResolveItemModel) -> Element {
    let BurgerResolveItemPresentation {
        icon,
        label,
        state,
        disabled,
        role,
        onclick,
    } = use_burger_resolve_item(&props);
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

assert_component!(BurgerResolveItem);
