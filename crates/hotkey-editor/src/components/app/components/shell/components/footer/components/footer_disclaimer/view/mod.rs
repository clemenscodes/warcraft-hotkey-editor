#[derive(Clone, PartialEq)]
pub struct FooterDisclaimerView {
    pub text: &'static str,
}

impl ddd::View for FooterDisclaimerView {}
