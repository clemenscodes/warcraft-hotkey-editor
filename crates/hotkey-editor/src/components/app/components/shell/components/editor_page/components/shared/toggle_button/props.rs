use super::view::ToggleButtonView;
use dioxus::prelude::*;

/// The one labeled pill button, shared by the mode toggle, the search-field toggle,
/// and the catalog-visibility toggle. It is presentational: props in, markup out. It
/// carries no size of its own — the parent sizes the box it fills, and the button
/// scales its whole interior in `cqi` off that box, so the same component draws large
/// (the mode/search pills) or small (the narrower catalog pills) with no variant.
#[derive(Props, Clone, PartialEq)]
pub struct ToggleButtonProps {
    /// The button text.
    pub label: &'static str,
    /// Whether this button is the active one in its group.
    pub active: bool,
    /// An optional tooltip (the catalog toggles explain what they widen; the mode and
    /// search toggles have none).
    #[props(default)]
    pub title: Option<&'static str>,
    /// Activation handler.
    pub onclick: EventHandler<MouseEvent>,
    /// Extra keyboard handling on top of the native button (the mode toggle moves
    /// focus onto the race tabs); the others leave it at the no-op default.
    #[props(default)]
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&ToggleButtonView> for ToggleButtonProps {
    fn from(view: &ToggleButtonView) -> Self {
        let ToggleButtonView {
            label,
            active,
            title,
            onclick,
            onkeydown,
        } = view.clone();
        Self {
            label,
            active,
            title,
            onclick,
            onkeydown,
        }
    }
}

impl ddd::Props for ToggleButtonProps {
    type View = ToggleButtonView;
}
