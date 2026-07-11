use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::presentation::BurgerMenuRow;

/// The published `View` contract mirroring [`BurgerDrawerBodyModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BurgerDrawerBodyView {
    pub layout: BurgerMenuRow,
    pub items: Vec<BurgerMenuRow>,
}

impl ddd::View for BurgerDrawerBodyView {}
