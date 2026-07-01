use dioxus::prelude::*;

use super::components::footer_heart::FooterHeartProps;

#[derive(Props, Clone, PartialEq)]
pub struct FooterCreditProps {
    pub lead: &'static str,
    pub tail: &'static str,
    pub heart: &'static str,
}

impl From<&FooterCreditProps> for FooterHeartProps {
    fn from(props: &FooterCreditProps) -> Self {
        let svg = props.heart;
        Self { svg }
    }
}
