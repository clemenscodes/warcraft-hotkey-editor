use crate::components::app::components::shell::components::resolve_page::logic::UnresolvedView;
use dioxus::prelude::*;

/// The stuck ability's column: the unresolved view whose ability it renders as a name
/// plate over an ability icon.
#[derive(Props, Clone, PartialEq)]
pub struct FightColumnProps {
    pub unresolved_view: UnresolvedView,
}
