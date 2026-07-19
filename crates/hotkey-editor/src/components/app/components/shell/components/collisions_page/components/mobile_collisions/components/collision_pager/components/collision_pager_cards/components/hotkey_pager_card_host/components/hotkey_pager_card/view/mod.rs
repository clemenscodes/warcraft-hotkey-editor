use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyUnitView;

#[derive(Clone, PartialEq)]
pub struct HotkeyPagerCardView {
    pub unit: HotkeyUnitView,
}

impl ddd::View for HotkeyPagerCardView {}
