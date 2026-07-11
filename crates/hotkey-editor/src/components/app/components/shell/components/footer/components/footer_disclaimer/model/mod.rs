use super::view::FooterDisclaimerView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FooterDisclaimerModel {
    pub text: &'static str,
}

impl From<&FooterDisclaimerView> for FooterDisclaimerModel {
    fn from(view: &FooterDisclaimerView) -> Self {
        let FooterDisclaimerView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for FooterDisclaimerModel {
    type View = FooterDisclaimerView;
}
