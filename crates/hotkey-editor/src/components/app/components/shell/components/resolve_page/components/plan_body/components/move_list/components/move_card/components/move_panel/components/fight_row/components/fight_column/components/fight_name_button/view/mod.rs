use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct FightNameButtonView {
    pub name: String,
    pub object_id: WarcraftObjectId,
    pub has_unit: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for FightNameButtonView {}
