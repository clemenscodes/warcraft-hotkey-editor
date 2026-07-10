use dioxus::prelude::*;

/// The square frame holding a stat column's icon.
#[derive(Props, Clone, PartialEq)]
pub struct StatIconFrameProps {
    pub src: Asset,
    #[props(into)]
    pub alt: String,
}
