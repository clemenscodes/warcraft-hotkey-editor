use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerTemplatesItemView {
    pub on_close: EventHandler<()>,
}

impl ddd::Layered for BurgerTemplatesItemView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BurgerTemplatesItemView {}
