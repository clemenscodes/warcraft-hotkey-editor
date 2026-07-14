#[derive(Clone, PartialEq)]
pub struct UnitNameView {
    pub text: &'static str,
}

impl ddd::View for UnitNameView {}
