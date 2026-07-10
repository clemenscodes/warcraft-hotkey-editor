use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TileLabelProps {
    /// The label text, present only when the occupant has no icon.
    pub text: Option<String>,
}
