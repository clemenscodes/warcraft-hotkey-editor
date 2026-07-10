use crate::components::app::components::shell::components::collisions_page::logic::HotkeyUnitView;

/// The published `View` contract mirroring [`HotkeysContentProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HotkeysContentView {
    pub units: Vec<HotkeyUnitView>,
}

impl ddd::View for HotkeysContentView {}
