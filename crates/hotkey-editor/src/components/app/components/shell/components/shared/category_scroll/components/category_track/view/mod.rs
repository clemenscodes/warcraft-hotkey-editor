use warcraft_api::UnitCatalogGroup;

#[derive(Clone, PartialEq)]
pub struct CategoryTrackView {
    pub groups: Vec<UnitCatalogGroup>,
}

impl ddd::View for CategoryTrackView {}
