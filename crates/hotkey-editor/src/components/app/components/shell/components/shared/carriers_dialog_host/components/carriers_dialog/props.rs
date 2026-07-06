use crate::services::carriers::{CarrierUnitView, InspectedAbility};
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// The carriers dialog: the ability's title, its resolved carriers, the navigation
/// context its cards deep-link through, and the trigger's open-state signal it clears
/// when closed.
#[derive(Props, Clone, PartialEq)]
pub struct CarriersDialogProps {
    #[props(into)]
    pub title: String,
    pub carriers: Vec<CarrierUnitView>,
    pub view_navigation: ViewNavigationContext,
    pub open_state: Signal<Option<InspectedAbility>>,
}
