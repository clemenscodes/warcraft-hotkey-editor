use crate::components::app::components::shell::components::resolve_page::logic::{
    CarriersDialogData, MoveView,
};
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// One planned move to render: its shaped view, the navigation context its name
/// links through, and the carriers-dialog signal its icons open.
#[derive(Props, Clone, PartialEq)]
pub struct MoveRowProps {
    pub move_view: MoveView,
    pub view_navigation: ViewNavigationContext,
    pub carriers_dialog: Signal<Option<CarriersDialogData>>,
}
