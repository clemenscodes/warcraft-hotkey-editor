use super::view::ControlPlainIconView;
use dioxus::prelude::*;

/// The control-plain look's props: the optional image source and its alt text. Built by
/// the `FramedIcon` dispatcher from `FramedIconProps`. Absent `source` draws the empty
/// framed square; a present `source` draws the covered image inside the frame.
#[derive(Props, Clone, PartialEq)]
pub struct ControlPlainIconProps {
    pub source: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&ControlPlainIconView> for ControlPlainIconProps {
    fn from(view: &ControlPlainIconView) -> Self {
        let ControlPlainIconView { source, alt } = view.clone();
        Self { source, alt }
    }
}

impl ddd::Props for ControlPlainIconProps {
    type View = ControlPlainIconView;
}
