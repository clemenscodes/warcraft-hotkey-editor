#[derive(Clone, PartialEq)]
pub struct ToastDescriptionView {
    pub description: Option<String>,
}

impl ddd::View for ToastDescriptionView {}
