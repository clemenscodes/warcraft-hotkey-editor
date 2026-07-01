use crate::components::views::resolve_page::logic::CarriersDialogData;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// The carriers-dialog mount: the signal that names the ability whose carriers are
/// being shown (empty when the dialog is closed), and the navigation context the
/// dialog's cards deep-link through.
#[derive(Props, Clone, PartialEq)]
pub struct CarriersDialogHostProps {
    pub carriers_dialog: Signal<Option<CarriersDialogData>>,
    pub view_navigation: ViewNavigationContext,
}
