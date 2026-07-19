use super::view::MobileResolveView;
use crate::components::app::components::shell::components::resolve_page::presentation::{
    MoveSection, UnresolvedView,
};
use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MobileResolveModel {
    #[props(into)]
    pub moves_text: String,
    pub unresolved_count: usize,
    pub running: bool,
    pub on_apply: EventHandler<MouseEvent>,
    pub breadcrumbs: Vec<BreadcrumbView>,
    pub section: Option<MoveSection>,
    pub unresolved: Vec<UnresolvedView>,
}

impl From<&MobileResolveView> for MobileResolveModel {
    fn from(view: &MobileResolveView) -> Self {
        let MobileResolveView {
            moves_text,
            unresolved_count,
            running,
            on_apply,
            breadcrumbs,
            section,
            unresolved,
        } = view.clone();
        Self {
            moves_text,
            unresolved_count,
            running,
            on_apply,
            breadcrumbs,
            section,
            unresolved,
        }
    }
}

impl ddd::Model for MobileResolveModel {
    type View = MobileResolveView;
}
