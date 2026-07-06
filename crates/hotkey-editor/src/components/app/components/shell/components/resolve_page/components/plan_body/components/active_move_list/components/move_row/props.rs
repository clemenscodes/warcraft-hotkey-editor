use crate::components::app::components::shell::components::resolve_page::logic::MoveView;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// One planned move to render: its shaped view and the navigation context its name
/// links through. Each ability icon owns and opens its own carriers dialog.
#[derive(Props, Clone, PartialEq)]
pub struct MoveRowProps {
    pub move_view: MoveView,
    pub view_navigation: ViewNavigationContext,
}
