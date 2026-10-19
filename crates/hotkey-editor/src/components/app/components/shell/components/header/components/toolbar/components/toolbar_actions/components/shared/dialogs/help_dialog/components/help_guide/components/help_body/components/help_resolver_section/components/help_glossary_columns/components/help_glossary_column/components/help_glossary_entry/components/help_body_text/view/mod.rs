#[derive(Clone, PartialEq)]
pub struct HelpBodyTextView {
    pub text: String,
}

impl ddd::Layered for HelpBodyTextView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HelpBodyTextView {}
