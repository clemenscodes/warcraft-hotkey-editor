mod model;
mod view;

pub use view::ActiveMenuItemView;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::components::shared::burger_menu_item_icon::BurgerMenuItemIcon;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::components::shared::burger_menu_item_label::BurgerMenuItemLabel;
use dioxus::prelude::*;
use model::ActiveMenuItemModel;
use style::CLASS;
use tw_macro::assert_component;

/// The active (open-toggle) look of a drawer row: the gold panel button lit to show
/// its dialog/preview is open, composing the icon and label. Presentational — the
/// dispatcher names its fields and renders it when the row's visual weight is active.
#[component]
pub fn ActiveMenuItem(props: ActiveMenuItemModel) -> Element {
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

assert_component!(ActiveMenuItem);
