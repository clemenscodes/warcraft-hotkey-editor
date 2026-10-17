#[derive(Clone, PartialEq)]
pub struct ToastContentView {
    pub title: String,
    pub description: Option<String>,
}

impl ddd::Layered for ToastContentView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ToastContentView {}
