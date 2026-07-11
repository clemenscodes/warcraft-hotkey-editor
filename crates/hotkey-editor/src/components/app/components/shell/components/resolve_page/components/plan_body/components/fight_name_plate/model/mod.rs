use super::view::FightNamePlateView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;
/// The rival ability's name + id (non-interactive plate).
#[derive(Props, Clone, PartialEq)]
pub struct FightNamePlateModel {
    #[props(into)]
    pub name: String,
    pub object_id: WarcraftObjectId,
}

impl From<&FightNamePlateView> for FightNamePlateModel {
    fn from(view: &FightNamePlateView) -> Self {
        let FightNamePlateView { name, object_id } = view.clone();
        Self { name, object_id }
    }
}

impl ddd::Model for FightNamePlateModel {
    type View = FightNamePlateView;
}
