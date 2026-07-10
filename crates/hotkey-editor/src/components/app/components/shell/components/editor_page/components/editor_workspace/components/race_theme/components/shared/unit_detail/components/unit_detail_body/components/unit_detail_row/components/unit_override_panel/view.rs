use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::logic::UnitOverrideTarget;

/// The published `View` contract mirroring [`UnitOverridePanelProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitOverridePanelView {
    pub(crate) override_target: UnitOverrideTarget,
}

impl ddd::View for UnitOverridePanelView {}
