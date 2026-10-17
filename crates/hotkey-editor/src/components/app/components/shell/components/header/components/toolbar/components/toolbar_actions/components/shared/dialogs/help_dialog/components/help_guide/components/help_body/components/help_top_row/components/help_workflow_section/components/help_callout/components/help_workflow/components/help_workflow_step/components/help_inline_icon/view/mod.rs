#[derive(Clone, PartialEq)]
pub struct HelpInlineIconView {
    pub icon: &'static str,
}

impl ddd::Layered for HelpInlineIconView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HelpInlineIconView {}
