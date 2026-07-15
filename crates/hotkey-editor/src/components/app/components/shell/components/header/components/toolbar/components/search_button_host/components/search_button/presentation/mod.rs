use super::data::ARIA_LABEL;
use super::model::SearchButtonModel;
use crate::components::app::components::shell::components::shared::icons::ICON_SEARCH;
use dioxus::prelude::*;

pub struct SearchButtonPresentation {
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) aria_haspopup: Option<&'static str>,
    pub(super) aria_expanded: Option<bool>,
    pub(super) onclick: EventHandler<MouseEvent>,
}

impl From<&SearchButtonModel> for SearchButtonPresentation {
    fn from(props: &SearchButtonModel) -> Self {
        let aria_expanded = props.aria_expanded;
        let onclick = props.onclick;
        let aria_haspopup = Some("dialog");
        Self {
            icon: ICON_SEARCH,
            aria_label: ARIA_LABEL,
            aria_haspopup,
            aria_expanded,
            onclick,
        }
    }
}

impl ddd::Presentation for SearchButtonPresentation {
    type Model = SearchButtonModel;
}
