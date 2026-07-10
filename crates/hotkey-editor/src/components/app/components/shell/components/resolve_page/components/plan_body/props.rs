use crate::components::app::components::shell::components::resolve_page::logic::{
    MoveSection, UnresolvedView,
};
use dioxus::prelude::*;

/// The scrollable plan body: the active move section (absent when the plan has only
/// unresolved abilities) and every unresolved ability.
#[derive(Props, Clone, PartialEq)]
pub struct PlanBodyProps {
    pub section: Option<MoveSection>,
    pub unresolved: Vec<UnresolvedView>,
}
