#[derive(Clone, PartialEq)]
pub struct CarouselDotView {
    pub active: bool,
}

impl ddd::View for CarouselDotView {}
