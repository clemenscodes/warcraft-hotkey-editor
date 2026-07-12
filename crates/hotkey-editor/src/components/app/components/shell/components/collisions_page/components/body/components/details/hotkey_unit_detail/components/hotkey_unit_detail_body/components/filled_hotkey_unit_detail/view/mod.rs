use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyUnitView;

/// The published `View` contract mirroring [`FilledHotkeyUnitDetailModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FilledHotkeyUnitDetailView {
    pub unit_view: HotkeyUnitView,
}

impl ddd::View for FilledHotkeyUnitDetailView {}
