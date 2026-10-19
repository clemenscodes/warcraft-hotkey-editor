#[derive(Clone, PartialEq)]
pub struct ToastDescriptionView {
    pub description: Option<String>,
}

impl ddd::Layered for ToastDescriptionView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ToastDescriptionView {}
