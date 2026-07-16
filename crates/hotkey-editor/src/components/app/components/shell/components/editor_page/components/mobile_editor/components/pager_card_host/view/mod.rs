use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct PagerCardHostView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for PagerCardHostView {}
