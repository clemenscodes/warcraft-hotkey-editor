#[derive(Clone, PartialEq)]
pub struct RaceScopeChevronView {
    pub is_open: bool,
}

impl ddd::Layered for RaceScopeChevronView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for RaceScopeChevronView {}
