use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::logic::BurgerMenuRow;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`BurgerDrawerProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BurgerDrawerView {
    pub on_close: EventHandler<MouseEvent>,
    pub layout: BurgerMenuRow,
    pub items: Vec<BurgerMenuRow>,
}

impl ddd::View for BurgerDrawerView {}
