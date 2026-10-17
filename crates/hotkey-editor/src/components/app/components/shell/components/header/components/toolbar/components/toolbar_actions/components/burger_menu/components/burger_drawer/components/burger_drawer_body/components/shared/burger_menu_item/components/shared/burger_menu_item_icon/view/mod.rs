#[derive(Clone, PartialEq)]
pub struct BurgerMenuItemIconView {
    pub svg: &'static str,
}

impl ddd::Layered for BurgerMenuItemIconView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BurgerMenuItemIconView {}
