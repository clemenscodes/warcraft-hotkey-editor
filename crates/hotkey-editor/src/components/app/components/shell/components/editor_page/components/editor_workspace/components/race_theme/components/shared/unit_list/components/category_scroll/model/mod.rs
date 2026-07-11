use super::view::CategoryScrollView;
use dioxus::prelude::*;
use warcraft_api::UnitKind;

/// The scroll region's props: the kinds of the category sections to lay out, in
/// display order. Passed straight through to the inner track.
#[derive(Props, Clone, PartialEq)]
pub struct CategoryScrollModel {
    pub sections: Vec<UnitKind>,
}

impl From<&CategoryScrollView> for CategoryScrollModel {
    fn from(view: &CategoryScrollView) -> Self {
        let CategoryScrollView { sections } = view.clone();
        Self { sections }
    }
}

impl ddd::Model for CategoryScrollModel {
    type View = CategoryScrollView;
}
