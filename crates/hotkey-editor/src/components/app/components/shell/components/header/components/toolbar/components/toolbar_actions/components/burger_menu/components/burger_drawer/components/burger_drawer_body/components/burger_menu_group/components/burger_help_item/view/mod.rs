use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerHelpItemView {
    pub on_close: EventHandler<()>,
}

impl ddd::Layered for BurgerHelpItemView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BurgerHelpItemView {}
