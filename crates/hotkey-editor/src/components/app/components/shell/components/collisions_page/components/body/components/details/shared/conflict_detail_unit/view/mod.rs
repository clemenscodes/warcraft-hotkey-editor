use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct ConflictDetailUnitView {
    pub unit_id: WarcraftObjectId,
    pub icon_url: Option<String>,
    pub name: String,
}

impl ddd::View for ConflictDetailUnitView {}
