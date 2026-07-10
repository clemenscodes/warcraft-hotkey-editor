use super::view::UnresolvedRowView;
use crate::components::app::components::shell::components::resolve_page::logic::UnresolvedView;
use dioxus::prelude::*;

/// One ability the cascade could not place. Its ability icon owns and opens its own
/// carriers dialog.
#[derive(Props, Clone, PartialEq)]
pub struct UnresolvedRowProps {
    pub unresolved_view: UnresolvedView,
}

impl From<&UnresolvedRowView> for UnresolvedRowProps {
    fn from(view: &UnresolvedRowView) -> Self {
        let UnresolvedRowView { unresolved_view } = view.clone();
        Self { unresolved_view }
    }
}

impl ddd::Props for UnresolvedRowProps {
    type View = UnresolvedRowView;
}
