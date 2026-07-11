use super::view::ControlPlainIconView;
use dioxus::prelude::*;

/// The control-plain look's props: the optional image source and its alt text. Built by
/// the `FramedIcon` dispatcher from `FramedIconModel`. Absent `source` draws the empty
/// framed square; a present `source` draws the covered image inside the frame.
#[derive(Props, Clone, PartialEq)]
pub struct ControlPlainIconModel {
    pub source: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&ControlPlainIconView> for ControlPlainIconModel {
    fn from(view: &ControlPlainIconView) -> Self {
        let ControlPlainIconView { source, alt } = view.clone();
        Self { source, alt }
    }
}

impl ddd::Model for ControlPlainIconModel {
    type View = ControlPlainIconView;
}
