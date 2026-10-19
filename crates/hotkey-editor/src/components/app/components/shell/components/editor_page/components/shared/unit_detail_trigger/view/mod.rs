use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct UnitDetailTriggerView {
    pub icon_url: Option<String>,
    pub unit_id: WarcraftObjectId,
}

impl ddd::Layered for UnitDetailTriggerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitDetailTriggerView {}
