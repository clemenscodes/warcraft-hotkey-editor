mod props;
mod style;

use super::burger_menu::{BurgerMenu, BurgerMenuProps};
use super::collisions_button::{CollisionsButton, CollisionsButtonProps};
use super::header_toolbar::{HeaderToolbar, HeaderToolbarProps};
use crate::assert_component;
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
