use super::view::FooterLinkIconView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FooterLinkIconProps {
    pub icon: Option<&'static str>,
}

impl From<&FooterLinkIconView> for FooterLinkIconProps {
    fn from(view: &FooterLinkIconView) -> Self {
        let FooterLinkIconView { icon } = view.clone();
        Self { icon }
    }
}

impl ddd::Props for FooterLinkIconProps {
    type View = FooterLinkIconView;
}
