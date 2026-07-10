use super::view::ConflictUnitIconView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictUnitIconProps {
    pub src: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&ConflictUnitIconView> for ConflictUnitIconProps {
    fn from(view: &ConflictUnitIconView) -> Self {
        let ConflictUnitIconView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Props for ConflictUnitIconProps {
    type View = ConflictUnitIconView;
}
