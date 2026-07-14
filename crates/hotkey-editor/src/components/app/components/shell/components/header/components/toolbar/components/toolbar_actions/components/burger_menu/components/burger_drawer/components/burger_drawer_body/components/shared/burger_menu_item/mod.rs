pub mod components;
mod model;
mod view;

pub use view::BurgerMenuItemView;
mod state;

use components::active_menu_item::ActiveMenuItem;
use components::idle_menu_item::IdleMenuItem;
use components::primary_menu_item::PrimaryMenuItem;
use dioxus::prelude::*;
use model::BurgerMenuItemModel;
pub use state::BurgerItemState;
use tw_macro::assert_component;

#[component]
pub fn BurgerMenuItem(props: BurgerMenuItemModel) -> Element {
    let icon = props.icon;
    let label = props.label;
    let disabled = props.disabled;
    let role = props.role;
    let aria_haspopup = props.aria_haspopup;
    let aria_expanded = props.aria_expanded;
    let aria_pressed = props.aria_pressed;
    let aria_label = props.aria_label;
    let onclick = props.onclick;
    match props.state {
        BurgerItemState::Idle => rsx! {
            IdleMenuItem {
                icon,
                label,
                disabled,
                role,
                aria_haspopup,
                aria_expanded,
                aria_pressed,
                aria_label,
                onclick,
            }
        },
        BurgerItemState::Active => rsx! {
            ActiveMenuItem {
                icon,
                label,
                disabled,
                role,
                aria_haspopup,
                aria_expanded,
                aria_pressed,
                aria_label,
                onclick,
            }
        },
        BurgerItemState::Primary => rsx! {
            PrimaryMenuItem {
                icon,
                label,
                disabled,
                role,
                aria_haspopup,
                aria_expanded,
                aria_pressed,
                aria_label,
                onclick,
            }
        },
    }
}

assert_component!(BurgerMenuItem);
