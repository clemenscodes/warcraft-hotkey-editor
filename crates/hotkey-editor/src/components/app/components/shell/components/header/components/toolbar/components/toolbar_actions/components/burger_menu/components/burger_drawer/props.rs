use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::logic::BurgerMenuRow;
use dioxus::prelude::*;

/// The slide-in drawer: everything the drawer subtree needs, threaded from the
/// controller — the close handler, the primary Grid Layout row, and the grouped
/// file-action rows.
#[derive(Props, Clone, PartialEq)]
pub struct BurgerDrawerProps {
    pub on_close: EventHandler<MouseEvent>,
    pub layout: BurgerMenuRow,
    pub items: Vec<BurgerMenuRow>,
}
