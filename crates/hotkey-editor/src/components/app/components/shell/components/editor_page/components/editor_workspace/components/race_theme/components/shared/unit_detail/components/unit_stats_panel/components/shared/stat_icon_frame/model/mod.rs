use super::view::StatIconFrameView;
use dioxus::prelude::*;

/// The square frame holding a stat column's icon.
#[derive(Props, Clone, PartialEq)]
pub struct StatIconFrameModel {
    pub src: Asset,
    #[props(into)]
    pub alt: String,
}

impl From<&StatIconFrameView> for StatIconFrameModel {
    fn from(view: &StatIconFrameView) -> Self {
        let StatIconFrameView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Model for StatIconFrameModel {
    type View = StatIconFrameView;
}
