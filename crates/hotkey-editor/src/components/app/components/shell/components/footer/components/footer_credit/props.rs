use super::view::FooterCreditView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FooterCreditProps {
    pub lead: &'static str,
    pub tail: &'static str,
    pub heart: &'static str,
}

impl From<&FooterCreditView> for FooterCreditProps {
    fn from(view: &FooterCreditView) -> Self {
        let FooterCreditView { lead, tail, heart } = view.clone();
        Self { lead, tail, heart }
    }
}

impl ddd::Props for FooterCreditProps {
    type View = FooterCreditView;
}
