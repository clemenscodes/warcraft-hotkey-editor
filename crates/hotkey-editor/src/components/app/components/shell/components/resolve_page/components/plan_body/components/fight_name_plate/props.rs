use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;
/// The rival ability's name + id (non-interactive plate).
#[derive(Props, Clone, PartialEq)]
pub struct FightNamePlateProps {
    #[props(into)]
    pub name: String,
    pub object_id: WarcraftObjectId,
}
