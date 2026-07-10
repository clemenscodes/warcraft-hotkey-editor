use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::logic::BurgerMenuRow;

/// The published `View` contract mirroring [`BurgerMenuGroupProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BurgerMenuGroupView {
    pub items: Vec<BurgerMenuRow>,
}

impl ddd::View for BurgerMenuGroupView {}
