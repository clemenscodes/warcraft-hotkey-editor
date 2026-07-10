use super::view::FooterDisclaimerView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FooterDisclaimerProps {
    pub text: &'static str,
}

impl From<&FooterDisclaimerView> for FooterDisclaimerProps {
    fn from(view: &FooterDisclaimerView) -> Self {
        let FooterDisclaimerView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for FooterDisclaimerProps {
    type View = FooterDisclaimerView;
}
