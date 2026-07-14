mod model;
mod view;

pub use view::IdleMenuItemView;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::components::shared::burger_menu_item_icon::BurgerMenuItemIcon;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::components::shared::burger_menu_item_label::BurgerMenuItemLabel;
use dioxus::prelude::*;
use model::IdleMenuItemModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn IdleMenuItem(props: IdleMenuItemModel) -> Element {
    let icon = props.icon;
    let label = props.label;
    let disabled = props.disabled;
    let role = props.role;
    let aria_haspopup = props.aria_haspopup;
    let aria_expanded = props.aria_expanded;
    let aria_pressed = props.aria_pressed;
    let aria_label = props.aria_label;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            role,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            aria_label,
            disabled,
            onclick,
            BurgerMenuItemIcon {
                svg: icon,
            }
            BurgerMenuItemLabel {
                text: label,
            }
        }
    }
}

assert_component!(IdleMenuItem);
