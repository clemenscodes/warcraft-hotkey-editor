use super::view::EditorPageView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct EditorPageModel {
    pub race: Option<String>,
    pub mode: Option<String>,
    pub unit: Option<String>,
    pub search_query: Option<String>,
}

impl From<&EditorPageView> for EditorPageModel {
    fn from(view: &EditorPageView) -> Self {
        let EditorPageView {
            race,
            mode,
            unit,
            search_query,
        } = view.clone();
        Self {
            race,
            mode,
            unit,
            search_query,
        }
    }
}

impl ddd::Model for EditorPageModel {
    type View = EditorPageView;
}
