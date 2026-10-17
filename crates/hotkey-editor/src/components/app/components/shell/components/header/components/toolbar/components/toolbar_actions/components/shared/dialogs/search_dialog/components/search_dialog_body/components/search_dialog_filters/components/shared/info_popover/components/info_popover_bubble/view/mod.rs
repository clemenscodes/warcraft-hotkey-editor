#[derive(Clone, PartialEq)]
pub struct InfoPopoverBubbleView {
    pub text: &'static str,
}

impl ddd::Layered for InfoPopoverBubbleView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for InfoPopoverBubbleView {}
