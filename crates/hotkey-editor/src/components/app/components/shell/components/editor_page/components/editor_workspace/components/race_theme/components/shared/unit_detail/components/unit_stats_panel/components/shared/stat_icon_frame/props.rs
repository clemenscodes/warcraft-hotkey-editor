use super::view::StatIconFrameView;
use dioxus::prelude::*;

/// The square frame holding a stat column's icon.
#[derive(Props, Clone, PartialEq)]
pub struct StatIconFrameProps {
    pub src: Asset,
    #[props(into)]
    pub alt: String,
}

impl From<&StatIconFrameView> for StatIconFrameProps {
    fn from(view: &StatIconFrameView) -> Self {
        let StatIconFrameView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Props for StatIconFrameProps {
    type View = StatIconFrameView;
}
