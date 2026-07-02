use crate::components::views::collisions_page::logic::CarrierDialogData;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// The carriers dialog: the shared ability's carriers, the signal it clears when
/// closed, and the navigation context its cards deep-link through.
#[derive(Props, Clone, PartialEq)]
pub struct CarriersDialogProps {
    pub dialog_data: CarrierDialogData,
    pub carrier_dialog: Signal<Option<CarrierDialogData>>,
    pub view_navigation: ViewNavigationContext,
}
