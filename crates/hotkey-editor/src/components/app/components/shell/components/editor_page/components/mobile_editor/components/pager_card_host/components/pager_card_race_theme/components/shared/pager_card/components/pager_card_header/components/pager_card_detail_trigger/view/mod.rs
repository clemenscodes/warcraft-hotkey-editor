use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct PagerCardDetailTriggerView {
    pub icon_url: Option<String>,
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for PagerCardDetailTriggerView {}
