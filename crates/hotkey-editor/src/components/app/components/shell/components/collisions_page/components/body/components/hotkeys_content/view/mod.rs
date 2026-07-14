use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyUnitView;

#[derive(Clone, PartialEq)]
pub struct HotkeysContentView {
    pub units: Vec<HotkeyUnitView>,
}

impl ddd::View for HotkeysContentView {}
