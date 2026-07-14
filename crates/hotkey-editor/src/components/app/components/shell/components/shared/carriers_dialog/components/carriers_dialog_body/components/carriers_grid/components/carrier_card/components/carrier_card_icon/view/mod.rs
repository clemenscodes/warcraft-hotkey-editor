#[derive(Clone, PartialEq)]
pub struct CarrierCardIconView {
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::View for CarrierCardIconView {}
