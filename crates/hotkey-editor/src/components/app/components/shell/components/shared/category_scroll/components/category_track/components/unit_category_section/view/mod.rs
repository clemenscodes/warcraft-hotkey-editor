use warcraft_api::UnitCatalogGroup;

#[derive(Clone, PartialEq)]
pub struct UnitCategorySectionView {
    pub group: UnitCatalogGroup,
}

impl ddd::View for UnitCategorySectionView {}
