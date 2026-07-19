#[derive(Clone, PartialEq)]
pub struct RaceScopeChevronView {
    pub is_open: bool,
}

impl ddd::View for RaceScopeChevronView {}
