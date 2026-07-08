mod props;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::components::shared::burger_menu_item_icon::BurgerMenuItemIcon;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::components::shared::burger_menu_item_label::BurgerMenuItemLabel;
use dioxus::prelude::*;
pub use props::PrimaryMenuItemProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(PrimaryMenuItem);

/// The primary (call-to-action) look of a drawer row: the emphasised gold panel button
/// carrying the drawer's headline action, composing the icon and label. Presentational
/// — the dispatcher builds its props and renders it when the row's visual weight is
/// primary.
#[component]
pub fn PrimaryMenuItem(props: PrimaryMenuItemProps) -> Element {
    let icon = props.icon;
    let label = props.label;
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
            class: CLASS,
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
