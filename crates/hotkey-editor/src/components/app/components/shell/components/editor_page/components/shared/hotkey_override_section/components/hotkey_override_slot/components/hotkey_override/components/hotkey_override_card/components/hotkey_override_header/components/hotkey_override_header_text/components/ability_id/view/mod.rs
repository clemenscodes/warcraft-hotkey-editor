use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct AbilityIdView {
    pub object_id: WarcraftObjectId,
}

impl ddd::Layered for AbilityIdView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for AbilityIdView {}
