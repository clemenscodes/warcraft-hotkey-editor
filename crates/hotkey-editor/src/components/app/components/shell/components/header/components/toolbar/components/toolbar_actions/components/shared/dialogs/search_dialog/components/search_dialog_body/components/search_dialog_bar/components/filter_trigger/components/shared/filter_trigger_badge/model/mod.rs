use super::view::FilterTriggerBadgeView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FilterTriggerBadgeModel {
    pub count: usize,
}

impl From<&FilterTriggerBadgeView> for FilterTriggerBadgeModel {
    fn from(view: &FilterTriggerBadgeView) -> Self {
        let FilterTriggerBadgeView { count } = view.clone();
        Self { count }
    }
}

impl ddd::Model for FilterTriggerBadgeModel {
    type View = FilterTriggerBadgeView;
}
