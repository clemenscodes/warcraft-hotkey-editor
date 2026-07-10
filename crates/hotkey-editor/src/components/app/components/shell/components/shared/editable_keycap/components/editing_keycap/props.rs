use super::view::EditingKeycapView;
use dioxus::prelude::*;

/// The pulsing editable keycap: the glyph it shows. Rendered by the `EditableKeycap`
/// dispatcher with named fields when the cap is `Editing`. Its corner radius comes from the
/// inherited `--keycap-radius`.
#[derive(Props, Clone, PartialEq)]
pub struct EditingKeycapProps {
    /// The visible glyph — a single letter, "–", "Esc", "Mouse4", etc.
    #[props(into)]
    pub label: String,
}

impl From<&EditingKeycapView> for EditingKeycapProps {
    fn from(view: &EditingKeycapView) -> Self {
        let EditingKeycapView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Props for EditingKeycapProps {
    type View = EditingKeycapView;
}
