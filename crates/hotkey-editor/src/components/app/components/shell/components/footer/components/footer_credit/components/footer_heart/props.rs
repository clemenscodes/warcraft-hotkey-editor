use super::view::FooterHeartView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FooterHeartProps {
    pub svg: &'static str,
}

impl From<&FooterHeartView> for FooterHeartProps {
    fn from(view: &FooterHeartView) -> Self {
        let FooterHeartView { svg } = view.clone();
        Self { svg }
    }
}

impl ddd::Props for FooterHeartProps {
    type View = FooterHeartView;
}
