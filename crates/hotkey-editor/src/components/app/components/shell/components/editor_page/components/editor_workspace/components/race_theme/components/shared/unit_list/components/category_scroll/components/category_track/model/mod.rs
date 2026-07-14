use super::view::CategoryTrackView;
use dioxus::prelude::*;
use warcraft_api::UnitKind;

#[derive(Props, Clone, PartialEq)]
pub struct CategoryTrackModel {
    pub sections: Vec<UnitKind>,
}

impl From<&CategoryTrackView> for CategoryTrackModel {
    fn from(view: &CategoryTrackView) -> Self {
        let CategoryTrackView { sections } = view.clone();
        Self { sections }
    }
}

impl ddd::Model for CategoryTrackModel {
    type View = CategoryTrackView;
}
