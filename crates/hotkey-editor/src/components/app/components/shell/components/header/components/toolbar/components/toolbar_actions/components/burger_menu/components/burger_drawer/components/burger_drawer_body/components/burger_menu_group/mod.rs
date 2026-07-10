mod props;
mod view;

pub use view::BurgerMenuGroupView;
mod style;

use super::shared::burger_menu_item::BurgerMenuItem;
use dioxus::prelude::*;
use props::BurgerMenuGroupProps;
use style::CLASS;
use tw_macro::assert_component;

/// The scrolling list of file actions inside the drawer.
#[component]
pub fn BurgerMenuGroup(props: BurgerMenuGroupProps) -> Element {
    let items = props.items;
    rsx! {
        div { class: CLASS, role: "menu", aria_label: "File actions",
            for row in items.into_iter() {
                BurgerMenuItem {
                    icon: row.icon,
                    label: row.label,
                    state: row.state,
                    disabled: row.disabled,
                    role: row.role,
                    aria_haspopup: row.aria_haspopup,
                    aria_expanded: row.aria_expanded,
                    aria_pressed: row.aria_pressed,
                    aria_label: row.aria_label,
                    onclick: row.onclick,
                }
            }
        }
    }
}

assert_component!(BurgerMenuGroup);
