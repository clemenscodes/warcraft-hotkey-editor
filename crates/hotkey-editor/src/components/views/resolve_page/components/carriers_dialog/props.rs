use crate::components::views::resolve_page::logic::CarriersDialogData;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// The carriers dialog: an ability's carriers, the signal it clears when closed,
/// and the navigation context its cards deep-link through.
#[derive(Props, Clone, PartialEq)]
pub struct CarriersDialogProps {
    pub dialog_data: CarriersDialogData,
    pub carriers_dialog: Signal<Option<CarriersDialogData>>,
    pub view_navigation: ViewNavigationContext,
}
