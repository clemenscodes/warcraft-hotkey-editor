use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyConflictView;
use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`HotkeyConflictGridModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HotkeyConflictGridView {
    pub conflicts: Vec<HotkeyConflictView>,
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for HotkeyConflictGridView {}
