use super::view::PagerCardTitleView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct PagerCardTitleModel {
    #[props(into)]
    pub name: String,
    pub unit_id: WarcraftObjectId,
}

impl From<&PagerCardTitleView> for PagerCardTitleModel {
    fn from(view: &PagerCardTitleView) -> Self {
        let PagerCardTitleView { name, unit_id } = view.clone();
        Self { name, unit_id }
    }
}

impl ddd::Model for PagerCardTitleModel {
    type View = PagerCardTitleView;
}
