use crate::services::carriers::InspectedAbility;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct CarriersDialogView {
    pub ability: Option<InspectedAbility>,
    pub on_close: Callback<()>,
}

impl ddd::View for CarriersDialogView {}
