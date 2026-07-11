use super::view::FooterLinkIconView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FooterLinkIconModel {
    pub icon: Option<&'static str>,
}

impl From<&FooterLinkIconView> for FooterLinkIconModel {
    fn from(view: &FooterLinkIconView) -> Self {
        let FooterLinkIconView { icon } = view.clone();
        Self { icon }
    }
}

impl ddd::Model for FooterLinkIconModel {
    type View = FooterLinkIconView;
}
