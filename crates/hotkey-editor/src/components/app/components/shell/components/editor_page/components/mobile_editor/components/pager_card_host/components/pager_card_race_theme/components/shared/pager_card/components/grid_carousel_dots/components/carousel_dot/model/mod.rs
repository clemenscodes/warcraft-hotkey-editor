use super::view::CarouselDotView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CarouselDotModel {
    pub active: bool,
}

impl From<&CarouselDotView> for CarouselDotModel {
    fn from(view: &CarouselDotView) -> Self {
        let CarouselDotView { active } = view.clone();
        Self { active }
    }
}

impl ddd::Layered for CarouselDotModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for CarouselDotModel {
    type View = CarouselDotView;
}
