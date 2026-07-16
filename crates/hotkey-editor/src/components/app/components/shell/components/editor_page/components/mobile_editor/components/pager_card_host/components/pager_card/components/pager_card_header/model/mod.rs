use super::view::PagerCardHeaderView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct PagerCardHeaderModel {
    pub icon_url: Option<String>,
    #[props(into)]
    pub name: String,
    pub unit_id: WarcraftObjectId,
}

impl From<&PagerCardHeaderView> for PagerCardHeaderModel {
    fn from(view: &PagerCardHeaderView) -> Self {
        let PagerCardHeaderView {
            icon_url,
            name,
            unit_id,
        } = view.clone();
        Self {
            icon_url,
            name,
            unit_id,
        }
    }
}

impl ddd::Model for PagerCardHeaderModel {
    type View = PagerCardHeaderView;
}
