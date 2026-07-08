use super::components::unit_description::UnitDescriptionProps;
use super::components::unit_detail_body::UnitDetailBodyProps;
use super::components::unit_detail_header::UnitDetailHeaderProps;
use super::components::unit_stats_panel::UnitStatsPanelProps;

/// The panel's shaped view: either an empty-state message, or the fully-built child
/// props for the loaded unit.
pub(super) enum UnitDetailView {
    Empty(&'static str),
    Loaded(Box<UnitDetailModel>),
}

/// Every child's finished props for a loaded unit.
pub(super) struct UnitDetailModel {
    pub(super) header: UnitDetailHeaderProps,
    pub(super) description: UnitDescriptionProps,
    pub(super) stats: UnitStatsPanelProps,
    pub(super) body: UnitDetailBodyProps,
}
