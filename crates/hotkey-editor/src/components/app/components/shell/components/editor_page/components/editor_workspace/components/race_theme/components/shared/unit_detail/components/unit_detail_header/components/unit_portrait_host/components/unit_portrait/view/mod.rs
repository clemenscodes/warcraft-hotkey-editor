#[derive(Clone, PartialEq)]
pub struct UnitPortraitView {
    pub src: String,
    pub alt: &'static str,
}

impl ddd::View for UnitPortraitView {}
