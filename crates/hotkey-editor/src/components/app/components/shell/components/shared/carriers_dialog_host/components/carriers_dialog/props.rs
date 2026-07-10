use super::view::CarriersDialogView;
use crate::services::carriers::{CarrierUnitView, InspectedAbility};
use dioxus::prelude::*;

/// The carriers dialog: the ability's title, its resolved carriers, and the trigger's
/// open-state signal it clears when closed. Its cards read the navigation they deep-link
/// through from context, so no navigation is threaded here.
#[derive(Props, Clone, PartialEq)]
pub struct CarriersDialogProps {
    #[props(into)]
    pub title: String,
    pub carriers: Vec<CarrierUnitView>,
    pub open_state: Signal<Option<InspectedAbility>>,
}

impl From<&CarriersDialogView> for CarriersDialogProps {
    fn from(view: &CarriersDialogView) -> Self {
        let CarriersDialogView {
            title,
            carriers,
            open_state,
        } = view.clone();
        Self {
            title,
            carriers,
            open_state,
        }
    }
}

impl ddd::Props for CarriersDialogProps {
    type View = CarriersDialogView;
}
