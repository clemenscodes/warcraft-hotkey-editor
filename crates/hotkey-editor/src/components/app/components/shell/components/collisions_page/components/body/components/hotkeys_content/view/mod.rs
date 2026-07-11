use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyUnitView;

/// The published `View` contract mirroring [`HotkeysContentModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HotkeysContentView {
    pub units: Vec<HotkeyUnitView>,
}

impl ddd::View for HotkeysContentView {}
