use super::view::BurgerDrawerBodyView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::presentation::BurgerMenuRow;
use dioxus::prelude::*;

/// The drawer's scrolling content: the primary Grid Layout row on top, then the
/// grouped file actions.
#[derive(Props, Clone, PartialEq)]
pub struct BurgerDrawerBodyModel {
    pub layout: BurgerMenuRow,
    pub items: Vec<BurgerMenuRow>,
}

impl From<&BurgerDrawerBodyView> for BurgerDrawerBodyModel {
    fn from(view: &BurgerDrawerBodyView) -> Self {
        let BurgerDrawerBodyView { layout, items } = view.clone();
        Self { layout, items }
    }
}

impl ddd::Model for BurgerDrawerBodyModel {
    type View = BurgerDrawerBodyView;
}
