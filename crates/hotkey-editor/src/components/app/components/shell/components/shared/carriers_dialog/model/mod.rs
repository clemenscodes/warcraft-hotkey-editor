use super::view::CarriersDialogView;
use crate::services::carriers::InspectedAbility;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CarriersDialogModel {
    pub ability: Option<InspectedAbility>,
    pub on_close: Callback<()>,
}

impl From<&CarriersDialogView> for CarriersDialogModel {
    fn from(view: &CarriersDialogView) -> Self {
        let CarriersDialogView { ability, on_close } = view.clone();
        Self { ability, on_close }
    }
}

impl ddd::Model for CarriersDialogModel {
    type View = CarriersDialogView;
}
