pub mod components;
mod logic;
mod props;
mod state;

use components::active_menu_item::{ActiveMenuItem, ActiveMenuItemProps};
use components::idle_menu_item::{IdleMenuItem, IdleMenuItemProps};
use components::primary_menu_item::{PrimaryMenuItem, PrimaryMenuItemProps};
use dioxus::prelude::*;
pub use props::BurgerMenuItemProps;
pub use state::BurgerItemState;
use tw_macro::assert_component;

/// A single drawer row. A pure dispatcher: from the row's visual weight it renders
/// the matching look — `IdleMenuItem` xor `ActiveMenuItem` xor `PrimaryMenuItem`.
/// Each look owns the same `<button>` markup and its own weight styling; this
/// dispatcher only builds each look's props from the shared `BurgerMenuItemProps`
/// and renders the one the state selects.
#[component]
pub fn BurgerMenuItem(props: BurgerMenuItemProps) -> Element {
    match props.state {
        BurgerItemState::Idle => {
            let item = IdleMenuItemProps::from(&props);
            rsx! {
                IdleMenuItem { ..item }
            }
        }
        BurgerItemState::Active => {
            let item = ActiveMenuItemProps::from(&props);
            rsx! {
                ActiveMenuItem { ..item }
            }
        }
        BurgerItemState::Primary => {
            let item = PrimaryMenuItemProps::from(&props);
            rsx! {
                PrimaryMenuItem { ..item }
            }
        }
    }
}

assert_component!(BurgerMenuItem);
