use crate::components::app::components::shell::components::resolve_page::presentation::UnresolvedView;

#[derive(Clone, PartialEq)]
pub struct FightRowView {
    pub unresolved_view: UnresolvedView,
}

impl ddd::Layered for FightRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for FightRowView {}
