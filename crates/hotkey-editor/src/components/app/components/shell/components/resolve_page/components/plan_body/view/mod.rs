use crate::components::app::components::shell::components::resolve_page::presentation::{
    MoveSection, UnresolvedView,
};

/// The published `View` contract mirroring [`PlanBodyModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct PlanBodyView {
    pub section: Option<MoveSection>,
    pub unresolved: Vec<UnresolvedView>,
}

impl ddd::View for PlanBodyView {}
