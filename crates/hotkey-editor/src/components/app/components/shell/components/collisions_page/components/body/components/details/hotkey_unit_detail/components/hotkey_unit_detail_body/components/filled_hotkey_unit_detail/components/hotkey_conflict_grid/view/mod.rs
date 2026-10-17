use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyConflictView;
use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct HotkeyConflictGridView {
    pub conflicts: Vec<HotkeyConflictView>,
    pub unit_id: WarcraftObjectId,
}

impl ddd::Layered for HotkeyConflictGridView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HotkeyConflictGridView {}
