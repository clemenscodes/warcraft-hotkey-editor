#[derive(Clone, PartialEq)]
pub struct FooterDisclaimerView {
    pub text: &'static str,
}

impl ddd::Layered for FooterDisclaimerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for FooterDisclaimerView {}
