use super::view::FooterLinkView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FooterLinkModel {
    pub label: &'static str,
    pub href: &'static str,
    pub icon: Option<&'static str>,
}

impl From<&FooterLinkView> for FooterLinkModel {
    fn from(view: &FooterLinkView) -> Self {
        let FooterLinkView { label, href, icon } = view.clone();
        Self { label, href, icon }
    }
}

impl ddd::Model for FooterLinkModel {
    type View = FooterLinkView;
}
