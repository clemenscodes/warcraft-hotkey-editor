use warcraft_api::UnitKind;

/// The published `View` contract mirroring [`MobileCategoryTabsProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MobileCategoryTabsView {
    pub tabs: Vec<UnitKind>,
}

impl ddd::View for MobileCategoryTabsView {}
