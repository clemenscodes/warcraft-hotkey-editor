use crate::components::app::components::shell::components::collisions_page::logic::HotkeyConflictView;
use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`HotkeyConflictCardProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HotkeyConflictCardView {
    pub conflict: HotkeyConflictView,
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for HotkeyConflictCardView {}
