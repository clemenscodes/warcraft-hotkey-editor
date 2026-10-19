#[derive(Clone, PartialEq)]
pub struct BurgerMenuItemLabelView {
    pub text: String,
}

impl ddd::Layered for BurgerMenuItemLabelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BurgerMenuItemLabelView {}
