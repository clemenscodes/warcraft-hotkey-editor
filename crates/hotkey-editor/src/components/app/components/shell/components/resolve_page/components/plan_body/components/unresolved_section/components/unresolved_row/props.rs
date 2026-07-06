use crate::components::app::components::shell::components::resolve_page::logic::UnresolvedView;
use dioxus::prelude::*;

/// One ability the cascade could not place. Its ability icon owns and opens its own
/// carriers dialog.
#[derive(Props, Clone, PartialEq)]
pub struct UnresolvedRowProps {
    pub unresolved_view: UnresolvedView,
}
