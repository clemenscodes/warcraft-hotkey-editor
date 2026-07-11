use super::view::FightNameButtonView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The moved ability's name + id as a button that deep-links into the editor when
/// the ability has a carrier unit.
#[derive(Props, Clone, PartialEq)]
pub struct FightNameButtonModel {
    #[props(into)]
    pub name: String,
    pub object_id: WarcraftObjectId,
    pub has_unit: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&FightNameButtonView> for FightNameButtonModel {
    fn from(view: &FightNameButtonView) -> Self {
        let FightNameButtonView {
            name,
            object_id,
            has_unit,
            onclick,
        } = view.clone();
        Self {
            name,
            object_id,
            has_unit,
            onclick,
        }
    }
}

impl ddd::Model for FightNameButtonModel {
    type View = FightNameButtonView;
}
