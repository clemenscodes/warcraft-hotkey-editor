use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The moved ability's name + id as a button that deep-links into the editor when
/// the ability has a carrier unit.
#[derive(Props, Clone, PartialEq)]
pub struct FightNameButtonProps {
    #[props(into)]
    pub name: String,
    pub object_id: WarcraftObjectId,
    pub has_unit: bool,
    pub onclick: EventHandler<MouseEvent>,
}
