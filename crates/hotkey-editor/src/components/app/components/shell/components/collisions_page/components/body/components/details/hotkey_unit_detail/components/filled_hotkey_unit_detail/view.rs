use crate::components::app::components::shell::components::collisions_page::logic::HotkeyUnitView;

/// The published `View` contract mirroring [`FilledHotkeyUnitDetailProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FilledHotkeyUnitDetailView {
    pub unit_view: HotkeyUnitView,
}

impl ddd::View for FilledHotkeyUnitDetailView {}
