use warcraft_api::UnitCatalogGroup;

#[derive(Clone, PartialEq)]
pub struct CategoryScrollView {
    pub groups: Vec<UnitCatalogGroup>,
}

impl ddd::View for CategoryScrollView {}
