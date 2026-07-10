use super::view::SearchFieldButtonView;
use dioxus::prelude::*;

/// One search-field option: its label, whether it is the active field, and the
/// handler that selects it.
#[derive(Props, Clone, PartialEq)]
pub struct SearchFieldButtonProps {
    pub label: &'static str,
    pub is_active: bool,
    pub on_select: EventHandler<MouseEvent>,
}

impl From<&SearchFieldButtonView> for SearchFieldButtonProps {
    fn from(view: &SearchFieldButtonView) -> Self {
        let SearchFieldButtonView {
            label,
            is_active,
            on_select,
        } = view.clone();
        Self {
            label,
            is_active,
            on_select,
        }
    }
}

impl ddd::Props for SearchFieldButtonProps {
    type View = SearchFieldButtonView;
}
