use super::view::FooterHeartView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FooterHeartModel {
    pub svg: &'static str,
}

impl From<&FooterHeartView> for FooterHeartModel {
    fn from(view: &FooterHeartView) -> Self {
        let FooterHeartView { svg } = view.clone();
        Self { svg }
    }
}

impl ddd::Model for FooterHeartModel {
    type View = FooterHeartView;
}
