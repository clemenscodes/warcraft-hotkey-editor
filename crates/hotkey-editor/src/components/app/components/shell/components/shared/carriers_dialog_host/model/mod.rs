use super::view::CarriersDialogHostView;
use crate::services::carriers::InspectedAbility;
use dioxus::prelude::*;

/// Guards the carriers dialog so it is mounted only while an ability is inspected: the
/// ability whose carriers to show (`None` when nothing is open), and the close handler that
/// clears the trigger's open state.
#[derive(Props, Clone, PartialEq)]
pub struct CarriersDialogHostModel {
    pub ability: Option<InspectedAbility>,
    pub on_close: Callback<()>,
}

impl From<&CarriersDialogHostView> for CarriersDialogHostModel {
    fn from(view: &CarriersDialogHostView) -> Self {
        let CarriersDialogHostView { ability, on_close } = view.clone();
        Self { ability, on_close }
    }
}

impl ddd::Model for CarriersDialogHostModel {
    type View = CarriersDialogHostView;
}
