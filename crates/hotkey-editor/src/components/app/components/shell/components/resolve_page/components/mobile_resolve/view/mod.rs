use crate::components::app::components::shell::components::resolve_page::presentation::{
    MoveSection, UnresolvedView,
};
use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbView;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct MobileResolveView {
    pub moves_text: String,
    pub unresolved_count: usize,
    pub running: bool,
    pub on_apply: EventHandler<MouseEvent>,
    pub breadcrumbs: Vec<BreadcrumbView>,
    pub section: Option<MoveSection>,
    pub unresolved: Vec<UnresolvedView>,
}

impl ddd::View for MobileResolveView {}
