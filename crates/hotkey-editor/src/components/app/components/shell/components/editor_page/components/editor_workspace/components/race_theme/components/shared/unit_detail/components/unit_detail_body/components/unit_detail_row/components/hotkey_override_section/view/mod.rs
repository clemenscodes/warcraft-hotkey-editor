use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::presentation::UnitOverrideTarget;

/// The published `View` contract mirroring [`HotkeyOverrideSectionModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HotkeyOverrideSectionView {
    pub(crate) override_target: UnitOverrideTarget,
}

impl ddd::View for HotkeyOverrideSectionView {}
