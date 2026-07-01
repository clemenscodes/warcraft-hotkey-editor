use dioxus::prelude::*;

use super::components::footer_link_icon::FooterLinkIconProps;

#[derive(Props, Clone, PartialEq)]
pub struct FooterLinkProps {
    pub label: &'static str,
    pub href: &'static str,
    pub icon: Option<&'static str>,
}

impl From<&FooterLinkProps> for FooterLinkIconProps {
    fn from(props: &FooterLinkProps) -> Self {
        let icon = props.icon;
        Self { icon }
    }
}
