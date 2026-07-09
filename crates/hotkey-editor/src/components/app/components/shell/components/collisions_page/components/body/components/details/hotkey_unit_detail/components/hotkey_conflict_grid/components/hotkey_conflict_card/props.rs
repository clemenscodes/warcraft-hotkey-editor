use crate::components::app::components::shell::components::collisions_page::logic::HotkeyConflictView;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// One shared-hotkey conflict card: the conflict data, the owning unit id, and the
/// navigation context its ability icons deep-link through.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyConflictCardProps {
    pub conflict: HotkeyConflictView,
    pub unit_id: WarcraftObjectId,
    pub view_navigation: ViewNavigationContext,
}
