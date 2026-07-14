use super::view::StatIconFrameHostView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StatIconFrameHostModel {
    pub src: Asset,
    #[props(into)]
    pub alt: String,
}

impl From<&StatIconFrameHostView> for StatIconFrameHostModel {
    fn from(view: &StatIconFrameHostView) -> Self {
        let StatIconFrameHostView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Model for StatIconFrameHostModel {
    type View = StatIconFrameHostView;
}
