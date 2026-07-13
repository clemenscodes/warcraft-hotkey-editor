use crate::services::carriers::InspectedAbility;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`CarriersDialogModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CarriersDialogView {
    pub ability: Option<InspectedAbility>,
    pub on_close: Callback<()>,
}

impl ddd::View for CarriersDialogView {}
