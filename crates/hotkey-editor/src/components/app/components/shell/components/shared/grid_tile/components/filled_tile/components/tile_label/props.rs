use super::view::TileLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TileLabelProps {
    /// The label text, present only when the occupant has no icon.
    pub text: Option<String>,
}

impl From<&TileLabelView> for TileLabelProps {
    fn from(view: &TileLabelView) -> Self {
        let TileLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for TileLabelProps {
    type View = TileLabelView;
}
