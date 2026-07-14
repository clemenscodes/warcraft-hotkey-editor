use super::view::ConflictUnitIconHostView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictUnitIconHostModel {
    pub src: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&ConflictUnitIconHostView> for ConflictUnitIconHostModel {
    fn from(view: &ConflictUnitIconHostView) -> Self {
        let ConflictUnitIconHostView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Model for ConflictUnitIconHostModel {
    type View = ConflictUnitIconHostView;
}
