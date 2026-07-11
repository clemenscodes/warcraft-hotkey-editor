use super::view::ConflictDetailUnitIconView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictDetailUnitIconModel {
    pub src: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&ConflictDetailUnitIconView> for ConflictDetailUnitIconModel {
    fn from(view: &ConflictDetailUnitIconView) -> Self {
        let ConflictDetailUnitIconView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Model for ConflictDetailUnitIconModel {
    type View = ConflictDetailUnitIconView;
}
