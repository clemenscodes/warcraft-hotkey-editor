#[derive(Clone, PartialEq)]
pub struct ToastContentView {
    pub title: String,
    pub description: Option<String>,
}

impl ddd::View for ToastContentView {}
