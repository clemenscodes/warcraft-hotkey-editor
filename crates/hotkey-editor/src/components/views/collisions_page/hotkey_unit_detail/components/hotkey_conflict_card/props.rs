use crate::components::views::collisions_page::HotkeyConflictView;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// One shared-hotkey conflict card: the conflict data, the owning unit key, and the
/// navigation context its ability icons deep-link through.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyConflictCardProps {
    pub conflict: HotkeyConflictView,
    #[props(into)]
    pub unit_id: String,
    pub view_navigation: ViewNavigationContext,
}
