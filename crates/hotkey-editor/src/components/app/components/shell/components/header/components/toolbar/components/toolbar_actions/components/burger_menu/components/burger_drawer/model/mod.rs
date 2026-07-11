use super::view::BurgerDrawerView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::presentation::BurgerMenuRow;
use dioxus::prelude::*;

/// The slide-in drawer: everything the drawer subtree needs, threaded from the
/// controller — the close handler, the primary Grid Layout row, and the grouped
/// file-action rows.
#[derive(Props, Clone, PartialEq)]
pub struct BurgerDrawerModel {
    pub on_close: EventHandler<MouseEvent>,
    pub layout: BurgerMenuRow,
    pub items: Vec<BurgerMenuRow>,
}

impl From<&BurgerDrawerView> for BurgerDrawerModel {
    fn from(view: &BurgerDrawerView) -> Self {
        let BurgerDrawerView {
            on_close,
            layout,
            items,
        } = view.clone();
        Self {
            on_close,
            layout,
            items,
        }
    }
}

impl ddd::Model for BurgerDrawerModel {
    type View = BurgerDrawerView;
}
