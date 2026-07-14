use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyUnitView;

#[derive(Clone, PartialEq)]
pub struct HotkeyUnitDetailView {
    pub units: Vec<HotkeyUnitView>,
}

impl ddd::View for HotkeyUnitDetailView {}
