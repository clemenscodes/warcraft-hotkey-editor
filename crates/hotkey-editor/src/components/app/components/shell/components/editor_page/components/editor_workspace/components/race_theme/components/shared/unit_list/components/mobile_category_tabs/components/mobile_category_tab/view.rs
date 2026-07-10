use warcraft_api::UnitKind;

/// The published `View` contract mirroring [`MobileCategoryTabProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MobileCategoryTabView {
    pub kind: UnitKind,
}

impl ddd::View for MobileCategoryTabView {}
