use super::view::FightColumnView;
use crate::components::app::components::shell::components::resolve_page::presentation::UnresolvedView;
use dioxus::prelude::*;

/// The stuck ability's column: the unresolved view whose ability it renders as a name
/// plate over an ability icon.
#[derive(Props, Clone, PartialEq)]
pub struct FightColumnModel {
    pub unresolved_view: UnresolvedView,
}

impl From<&FightColumnView> for FightColumnModel {
    fn from(view: &FightColumnView) -> Self {
        let FightColumnView { unresolved_view } = view.clone();
        Self { unresolved_view }
    }
}

impl ddd::Model for FightColumnModel {
    type View = FightColumnView;
}
