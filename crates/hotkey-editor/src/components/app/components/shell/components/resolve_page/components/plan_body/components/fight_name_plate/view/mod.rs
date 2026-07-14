use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct FightNamePlateView {
    pub name: String,
    pub object_id: WarcraftObjectId,
}

impl ddd::View for FightNamePlateView {}
