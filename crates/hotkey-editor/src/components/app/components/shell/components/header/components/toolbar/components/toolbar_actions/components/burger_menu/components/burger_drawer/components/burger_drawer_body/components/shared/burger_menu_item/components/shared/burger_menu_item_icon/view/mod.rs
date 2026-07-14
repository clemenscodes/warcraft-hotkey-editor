#[derive(Clone, PartialEq)]
pub struct BurgerMenuItemIconView {
    pub svg: &'static str,
}

impl ddd::View for BurgerMenuItemIconView {}
