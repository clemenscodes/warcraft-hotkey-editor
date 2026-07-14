#[derive(Clone, PartialEq)]
pub struct UnitPortraitHostView {
    pub src: Option<String>,
    pub alt: &'static str,
}

impl ddd::View for UnitPortraitHostView {}
