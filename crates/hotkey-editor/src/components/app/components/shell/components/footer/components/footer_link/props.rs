use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FooterLinkProps {
    pub label: &'static str,
    pub href: &'static str,
    pub icon: Option<&'static str>,
}
