use super::view::FightRowView;
use crate::components::app::components::shell::components::resolve_page::presentation::UnresolvedView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FightRowModel {
    pub unresolved_view: UnresolvedView,
}

impl From<&FightRowView> for FightRowModel {
    fn from(view: &FightRowView) -> Self {
        let FightRowView { unresolved_view } = view.clone();
        Self { unresolved_view }
    }
}

impl ddd::Model for FightRowModel {
    type View = FightRowView;
}
