pub mod components;
mod props;
mod state;
mod style;

use components::burger_menu_item_icon::{BurgerMenuItemIcon, BurgerMenuItemIconProps};
use components::burger_menu_item_label::{BurgerMenuItemLabel, BurgerMenuItemLabelProps};
use dioxus::prelude::*;
pub use props::BurgerMenuItemProps;
pub use state::BurgerItemState;
use tw_macro::assert_component;
assert_component!(BurgerMenuItem);

#[component]
pub fn BurgerMenuItem(props: BurgerMenuItemProps) -> Element {
    let class = style::class(props.state);
    let icon = BurgerMenuItemIconProps { svg: props.icon };
    let label = BurgerMenuItemLabelProps { text: props.label };
    let disabled = props.disabled;
    let role = props.role;
    let data_action = props.data_action;
    let aria_haspopup = props.aria_haspopup;
    let aria_expanded = props.aria_expanded;
    let aria_pressed = props.aria_pressed;
    let aria_label = props.aria_label;
    let onclick = props.onclick;
    rsx! {
        button {
            class,
            r#type: "button",
            role,
            "data-action": data_action,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            aria_label,
            disabled,
            onclick,
            BurgerMenuItemIcon { ..icon }
            BurgerMenuItemLabel { ..label }
        }
    }
}
