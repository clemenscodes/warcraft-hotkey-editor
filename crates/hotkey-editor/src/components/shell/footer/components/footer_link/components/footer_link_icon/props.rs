use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FooterLinkIconProps {
    pub icon: Option<&'static str>,
}
