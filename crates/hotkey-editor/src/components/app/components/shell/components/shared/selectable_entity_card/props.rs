use super::state::CardAccent;
use dioxus::prelude::*;

/// A selectable entity card: its accent (race colour or the fixed collision gold),
/// its selected flag, the click handler, and the leading-visual-plus-meta content
/// its caller nests inside. The optional keyboard and mount handlers let the editor
/// unit card add Space/Enter selection and register the focusable button with the
/// focus coordinator; the collision cards leave both unset.
#[derive(Props, Clone, PartialEq)]
pub struct SelectableEntityCardProps {
    pub accent: CardAccent,
    pub is_selected: bool,
    pub onclick: EventHandler<MouseEvent>,
    #[props(default)]
    pub onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(default)]
    pub onmounted: Option<EventHandler<Event<MountedData>>>,
    pub children: Element,
}
