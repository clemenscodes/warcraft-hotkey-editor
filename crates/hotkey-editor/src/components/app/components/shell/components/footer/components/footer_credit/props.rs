use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FooterCreditProps {
    pub lead: &'static str,
    pub tail: &'static str,
    pub heart: &'static str,
}
