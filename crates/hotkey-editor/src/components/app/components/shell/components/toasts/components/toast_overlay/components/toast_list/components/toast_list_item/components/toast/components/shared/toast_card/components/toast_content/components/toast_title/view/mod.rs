#[derive(Clone, PartialEq)]
pub struct ToastTitleView {
    pub title: String,
}

impl ddd::Layered for ToastTitleView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ToastTitleView {}
