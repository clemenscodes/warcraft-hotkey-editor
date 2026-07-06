pub mod components;
mod props;
mod style;

use components::burger_drawer_body::{BurgerDrawerBody, BurgerDrawerBodyProps};
use components::burger_drawer_header::{BurgerDrawerHeader, BurgerDrawerHeaderProps};
use dioxus::prelude::*;
pub use props::BurgerDrawerProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(BurgerDrawer);

/// The slide-in navigation drawer. Keeps the `burger-drawer` id/class the global
/// scroll-lock and `aria-controls` hook off.
#[component]
pub fn BurgerDrawer(props: BurgerDrawerProps) -> Element {
    let header = BurgerDrawerHeaderProps::from(&props);
    let body = BurgerDrawerBodyProps::from(&props);
    rsx! {
        aside {
            class: CLASS,
            id: "burger-drawer",
            role: "navigation",
            aria_label: "Menu",
            BurgerDrawerHeader { ..header }
            BurgerDrawerBody { ..body }
        }
    }
}
