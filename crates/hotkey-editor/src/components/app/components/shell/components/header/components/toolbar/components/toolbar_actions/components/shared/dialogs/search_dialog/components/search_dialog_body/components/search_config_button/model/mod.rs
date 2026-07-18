use super::view::SearchConfigButtonView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SearchConfigButtonModel {
    #[props(into)]
    pub label: String,
    pub open: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&SearchConfigButtonView> for SearchConfigButtonModel {
    fn from(view: &SearchConfigButtonView) -> Self {
        let SearchConfigButtonView {
            label,
            open,
            onclick,
        } = view.clone();
        Self {
            label,
            open,
            onclick,
        }
    }
}

impl ddd::Model for SearchConfigButtonModel {
    type View = SearchConfigButtonView;
}
