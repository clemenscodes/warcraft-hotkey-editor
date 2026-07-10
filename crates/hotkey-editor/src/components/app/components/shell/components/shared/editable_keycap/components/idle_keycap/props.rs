use super::view::IdleKeycapView;
use dioxus::prelude::*;

/// The resting editable keycap: the glyph it shows. Rendered by the `EditableKeycap`
/// dispatcher with named fields when the cap is `Idle`. Its corner radius comes from the
/// inherited `--keycap-radius`.
#[derive(Props, Clone, PartialEq)]
pub struct IdleKeycapProps {
    /// The visible glyph — a single letter, "–", "Esc", "Mouse4", etc.
    #[props(into)]
    pub label: String,
}

impl From<&IdleKeycapView> for IdleKeycapProps {
    fn from(view: &IdleKeycapView) -> Self {
        let IdleKeycapView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Props for IdleKeycapProps {
    type View = IdleKeycapView;
}
