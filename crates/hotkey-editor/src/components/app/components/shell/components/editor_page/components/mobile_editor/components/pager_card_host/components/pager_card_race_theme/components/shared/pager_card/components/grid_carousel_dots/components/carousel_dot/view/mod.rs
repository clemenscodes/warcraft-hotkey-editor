#[derive(Clone, PartialEq)]
pub struct CarouselDotView {
    pub active: bool,
}

impl ddd::Layered for CarouselDotView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CarouselDotView {}
