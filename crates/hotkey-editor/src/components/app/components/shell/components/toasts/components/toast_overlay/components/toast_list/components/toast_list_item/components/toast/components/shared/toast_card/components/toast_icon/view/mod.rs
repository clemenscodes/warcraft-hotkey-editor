#[derive(Clone, PartialEq)]
pub struct ToastIconView {
    pub icon: &'static str,
}

impl ddd::Layered for ToastIconView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ToastIconView {}
