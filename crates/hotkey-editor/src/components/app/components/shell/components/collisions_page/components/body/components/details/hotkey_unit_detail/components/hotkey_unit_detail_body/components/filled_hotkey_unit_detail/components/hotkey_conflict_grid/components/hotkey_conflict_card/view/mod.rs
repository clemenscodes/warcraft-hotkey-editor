use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyConflictView;
use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct HotkeyConflictCardView {
    pub conflict: HotkeyConflictView,
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for HotkeyConflictCardView {}
