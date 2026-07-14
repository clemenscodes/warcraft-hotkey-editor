#[derive(Clone, PartialEq)]
pub struct UnitPortraitView {
    pub src: Option<String>,
    pub alt: &'static str,
}

impl ddd::View for UnitPortraitView {}
