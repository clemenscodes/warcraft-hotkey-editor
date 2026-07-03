pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::burger_menu::{BurgerMenu, BurgerMenuProps};
use components::collisions_button::{CollisionsButton, CollisionsButtonProps};
use components::header_toolbar::{HeaderToolbar, HeaderToolbarProps};
use dioxus::prelude::*;
pub use props::HeaderActionsProps;
use style::CLASS;
assert_component!(HeaderActions);

#[component]
pub fn HeaderActions(props: HeaderActionsProps) -> Element {
    let collisions = CollisionsButtonProps::from(&props);
    let toolbar = HeaderToolbarProps::from(&props);
    let burger = BurgerMenuProps::from(&props);
    rsx! {
        div { class: CLASS,
            CollisionsButton { ..collisions }
            HeaderToolbar { ..toolbar }
            BurgerMenu { ..burger }
        }
    }
}
