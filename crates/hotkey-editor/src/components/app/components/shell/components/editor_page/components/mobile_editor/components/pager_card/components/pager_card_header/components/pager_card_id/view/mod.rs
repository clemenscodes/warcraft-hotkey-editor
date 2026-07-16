use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct PagerCardIdView {
    pub unit_id: WarcraftObjectId,
}

impl ddd::View for PagerCardIdView {}
