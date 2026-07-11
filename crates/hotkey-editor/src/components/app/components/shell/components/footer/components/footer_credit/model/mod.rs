use super::view::FooterCreditView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FooterCreditModel {
    pub lead: &'static str,
    pub tail: &'static str,
    pub heart: &'static str,
}

impl From<&FooterCreditView> for FooterCreditModel {
    fn from(view: &FooterCreditView) -> Self {
        let FooterCreditView { lead, tail, heart } = view.clone();
        Self { lead, tail, heart }
    }
}

impl ddd::Model for FooterCreditModel {
    type View = FooterCreditView;
}
