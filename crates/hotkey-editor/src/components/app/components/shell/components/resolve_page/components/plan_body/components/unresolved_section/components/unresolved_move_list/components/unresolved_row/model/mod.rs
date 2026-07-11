use super::view::UnresolvedRowView;
use crate::components::app::components::shell::components::resolve_page::presentation::UnresolvedView;
use dioxus::prelude::*;

/// One ability the cascade could not place. Its ability icon owns and opens its own
/// carriers dialog.
#[derive(Props, Clone, PartialEq)]
pub struct UnresolvedRowModel {
    pub unresolved_view: UnresolvedView,
}

impl From<&UnresolvedRowView> for UnresolvedRowModel {
    fn from(view: &UnresolvedRowView) -> Self {
        let UnresolvedRowView { unresolved_view } = view.clone();
        Self { unresolved_view }
    }
}

impl ddd::Model for UnresolvedRowModel {
    type View = UnresolvedRowView;
}
