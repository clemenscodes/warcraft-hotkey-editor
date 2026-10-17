use warcraft_api::UnitCatalogGroup;

#[derive(Clone, PartialEq)]
pub struct UnitCategorySectionView {
    pub group: UnitCatalogGroup,
}

impl ddd::Layered for UnitCategorySectionView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitCategorySectionView {}
