use super::view::TileOverrideInfoOnlyView;
use dioxus::prelude::*;

/// The note shown for a passive ability that has no hotkey field.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideInfoOnlyProps {
    #[props(into)]
    pub text: String,
}

impl From<&TileOverrideInfoOnlyView> for TileOverrideInfoOnlyProps {
    fn from(view: &TileOverrideInfoOnlyView) -> Self {
        let TileOverrideInfoOnlyView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for TileOverrideInfoOnlyProps {
    type View = TileOverrideInfoOnlyView;
}
