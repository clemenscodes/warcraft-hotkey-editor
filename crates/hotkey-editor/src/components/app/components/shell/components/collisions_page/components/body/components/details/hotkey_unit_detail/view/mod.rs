use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyUnitView;

/// The published `View` contract mirroring [`HotkeyUnitDetailModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HotkeyUnitDetailView {
    pub units: Vec<HotkeyUnitView>,
}

impl ddd::View for HotkeyUnitDetailView {}
