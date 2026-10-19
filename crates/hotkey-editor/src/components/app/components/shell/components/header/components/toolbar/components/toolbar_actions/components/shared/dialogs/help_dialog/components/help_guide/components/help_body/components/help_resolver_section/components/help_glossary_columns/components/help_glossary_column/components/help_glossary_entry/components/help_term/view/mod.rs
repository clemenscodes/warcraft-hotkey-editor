#[derive(Clone, PartialEq)]
pub struct HelpTermView {
    pub term: String,
}

impl ddd::Layered for HelpTermView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HelpTermView {}
