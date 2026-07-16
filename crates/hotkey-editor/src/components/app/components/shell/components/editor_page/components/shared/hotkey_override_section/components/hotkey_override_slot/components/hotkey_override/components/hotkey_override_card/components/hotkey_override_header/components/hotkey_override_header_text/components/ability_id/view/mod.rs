use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct AbilityIdView {
    pub object_id: WarcraftObjectId,
}

impl ddd::View for AbilityIdView {}
