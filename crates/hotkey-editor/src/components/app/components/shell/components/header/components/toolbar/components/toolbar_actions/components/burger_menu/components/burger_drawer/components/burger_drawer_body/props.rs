use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::logic::BurgerMenuRow;
use dioxus::prelude::*;

/// The drawer's scrolling content: the primary Grid Layout row on top, then the
/// grouped file actions.
#[derive(Props, Clone, PartialEq)]
pub struct BurgerDrawerBodyProps {
    pub layout: BurgerMenuRow,
    pub items: Vec<BurgerMenuRow>,
}
