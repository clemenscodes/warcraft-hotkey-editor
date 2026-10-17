#[derive(Clone, PartialEq)]
pub struct InfoPopoverView {
    pub text: &'static str,
}

impl ddd::Layered for InfoPopoverView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for InfoPopoverView {}
