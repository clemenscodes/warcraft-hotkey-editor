use super::view::ConflictUnitIconView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictUnitIconModel {
    pub src: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&ConflictUnitIconView> for ConflictUnitIconModel {
    fn from(view: &ConflictUnitIconView) -> Self {
        let ConflictUnitIconView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Model for ConflictUnitIconModel {
    type View = ConflictUnitIconView;
}
