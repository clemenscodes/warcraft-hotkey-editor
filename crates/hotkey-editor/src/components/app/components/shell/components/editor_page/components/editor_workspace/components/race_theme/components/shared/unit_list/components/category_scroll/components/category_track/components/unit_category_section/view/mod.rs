use warcraft_api::UnitKind;

#[derive(Clone, PartialEq)]
pub struct UnitCategorySectionView {
    pub category_kind: UnitKind,
}

impl ddd::View for UnitCategorySectionView {}
