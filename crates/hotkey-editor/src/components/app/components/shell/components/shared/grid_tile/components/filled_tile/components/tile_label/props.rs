use super::super::super::props::FilledTileProps;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TileLabelProps {
    /// The label text, present only when the occupant has no icon.
    pub text: Option<String>,
}

impl From<&FilledTileProps> for TileLabelProps {
    fn from(props: &FilledTileProps) -> Self {
        let text = if props.icon.is_none() {
            Some(props.label.clone())
        } else {
            None
        };
        Self { text }
    }
}
