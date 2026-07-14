use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyUnitView;

#[derive(Clone, PartialEq)]
pub struct FilledHotkeyUnitDetailView {
    pub unit_view: HotkeyUnitView,
}

impl ddd::View for FilledHotkeyUnitDetailView {}
