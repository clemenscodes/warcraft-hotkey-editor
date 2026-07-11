use super::view::PlaceholderIconView;
use dioxus::prelude::*;

/// The placeholder look's props: the optional image source and its alt text. Built by
/// the `FramedIcon` dispatcher from `FramedIconModel`. Absent `source` draws the empty
/// panel-filled square; a present `source` draws the covered image inside the frame.
#[derive(Props, Clone, PartialEq)]
pub struct PlaceholderIconModel {
    pub source: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&PlaceholderIconView> for PlaceholderIconModel {
    fn from(view: &PlaceholderIconView) -> Self {
        let PlaceholderIconView { source, alt } = view.clone();
        Self { source, alt }
    }
}

impl ddd::Model for PlaceholderIconModel {
    type View = PlaceholderIconView;
}
