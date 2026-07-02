use crate::components::views::collisions_page::logic::CarrierDialogData;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// The carriers-dialog mount for the island detail pane: the signal naming the
/// ability whose carriers are shown (empty when closed), and the navigation context
/// its cards deep-link through.
#[derive(Props, Clone, PartialEq)]
pub struct CarriersDialogHostProps {
    pub carrier_dialog: Signal<Option<CarrierDialogData>>,
    pub view_navigation: ViewNavigationContext,
}
