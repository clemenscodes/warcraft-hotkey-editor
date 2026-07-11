pub mod components;
mod model;
mod view;

pub use view::BurgerDrawerBodyView;
mod style;

use components::burger_menu_group::BurgerMenuGroup;
use components::shared::burger_menu_item::BurgerMenuItem;
use dioxus::prelude::*;
use model::BurgerDrawerBodyModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BurgerDrawerBody(props: BurgerDrawerBodyModel) -> Element {
    let layout = props.layout;
    let items = props.items;
    let icon = layout.icon;
    let label = layout.label;
    let state = layout.state;
    let disabled = layout.disabled;
    let role = layout.role;
    let aria_haspopup = layout.aria_haspopup;
    let aria_expanded = layout.aria_expanded;
    let aria_pressed = layout.aria_pressed;
    let aria_label = layout.aria_label;
    let onclick = layout.onclick;
    rsx! {
        div { class: CLASS,
            BurgerMenuItem {
                icon,
                label,
                state,
                disabled,
                role,
                aria_haspopup,
                aria_expanded,
                aria_pressed,
                aria_label,
                onclick,
            }
            BurgerMenuGroup { items }
        }
    }
}

assert_component!(BurgerDrawerBody);
