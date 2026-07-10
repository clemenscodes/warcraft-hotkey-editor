use crate::components::app::components::shell::components::resolve_page::logic::UnresolvedView;
use dioxus::prelude::*;

/// The stuck ability's row: the single centered column holding the stuck ability derived
/// from the unresolved view.
#[derive(Props, Clone, PartialEq)]
pub struct FightRowProps {
    pub unresolved_view: UnresolvedView,
}
