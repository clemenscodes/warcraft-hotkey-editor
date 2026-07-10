use super::view::ConflictDetailUnitIconView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictDetailUnitIconProps {
    pub src: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&ConflictDetailUnitIconView> for ConflictDetailUnitIconProps {
    fn from(view: &ConflictDetailUnitIconView) -> Self {
        let ConflictDetailUnitIconView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Props for ConflictDetailUnitIconProps {
    type View = ConflictDetailUnitIconView;
}
