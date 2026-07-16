use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct PagerCardView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for PagerCardView {}
