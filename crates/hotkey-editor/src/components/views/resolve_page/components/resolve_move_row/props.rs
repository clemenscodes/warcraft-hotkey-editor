use crate::components::views::resolve_page::logic::{CarriersDialogData, ResolveMoveView};
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// One planned move to render: its shaped view, the navigation context its name
/// links through, and the carriers-dialog signal its icons open.
#[derive(Props, Clone, PartialEq)]
pub struct ResolveMoveRowProps {
    pub move_view: ResolveMoveView,
    pub view_navigation: ViewNavigationContext,
    pub carriers_dialog: Signal<Option<CarriersDialogData>>,
}
