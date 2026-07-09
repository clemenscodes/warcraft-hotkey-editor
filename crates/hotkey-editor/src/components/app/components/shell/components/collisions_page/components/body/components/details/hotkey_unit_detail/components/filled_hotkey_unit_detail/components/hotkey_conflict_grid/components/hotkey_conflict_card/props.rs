use crate::components::app::components::shell::components::collisions_page::logic::HotkeyConflictView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// One shared-hotkey conflict card: the conflict data and the owning unit id. Its
/// ability icons read the navigation from context to deep-link, so no navigation is
/// threaded through this card.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyConflictCardProps {
    pub conflict: HotkeyConflictView,
    pub unit_id: WarcraftObjectId,
}
