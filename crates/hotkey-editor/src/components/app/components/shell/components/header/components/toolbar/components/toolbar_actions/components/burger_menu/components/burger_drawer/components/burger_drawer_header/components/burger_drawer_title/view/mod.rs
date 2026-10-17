#[derive(Clone, PartialEq)]
pub struct BurgerDrawerTitleView {
    pub title: &'static str,
}

impl ddd::Layered for BurgerDrawerTitleView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BurgerDrawerTitleView {}
