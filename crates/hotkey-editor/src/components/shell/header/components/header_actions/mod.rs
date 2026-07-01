mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

use super::burger_menu::{BurgerMenu, BurgerMenuProps};
use super::collisions_button::{CollisionsButton, CollisionsButtonProps};
use super::header_toolbar::{HeaderToolbar, HeaderToolbarProps};

pub use props::HeaderActionsProps;

assert_component!(HeaderActions);

#[component]
pub fn HeaderActions(props: HeaderActionsProps) -> Element {
    let collisions = CollisionsButtonProps::from(&props);
    let toolbar = HeaderToolbarProps::from(&props);
    let burger = BurgerMenuProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            CollisionsButton { ..collisions }
            HeaderToolbar { ..toolbar }
            BurgerMenu { ..burger }
        }
    }
}
