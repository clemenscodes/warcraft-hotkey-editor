use super::view::FooterLinkView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FooterLinkProps {
    pub label: &'static str,
    pub href: &'static str,
    pub icon: Option<&'static str>,
}

impl From<&FooterLinkView> for FooterLinkProps {
    fn from(view: &FooterLinkView) -> Self {
        let FooterLinkView { label, href, icon } = view.clone();
        Self { label, href, icon }
    }
}

impl ddd::Props for FooterLinkProps {
    type View = FooterLinkView;
}
