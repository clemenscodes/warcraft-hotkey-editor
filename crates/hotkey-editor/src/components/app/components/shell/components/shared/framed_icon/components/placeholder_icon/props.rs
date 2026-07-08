use dioxus::prelude::*;

/// The placeholder look's props: the optional image source and its alt text. Built by
/// the `FramedIcon` dispatcher from `FramedIconProps`. Absent `source` draws the empty
/// panel-filled square; a present `source` draws the covered image inside the frame.
#[derive(Props, Clone, PartialEq)]
pub struct PlaceholderIconProps {
    pub source: Option<String>,
    #[props(into)]
    pub alt: String,
}
