use warcraft_api::UnitKind;

/// The published `View` contract mirroring [`UnitCategorySectionModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitCategorySectionView {
    pub category_kind: UnitKind,
}

impl ddd::View for UnitCategorySectionView {}
