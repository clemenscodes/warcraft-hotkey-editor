use warcraft_api::UnitKind;

/// The published `View` contract mirroring [`CategoryScrollModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CategoryScrollView {
    pub sections: Vec<UnitKind>,
}

impl ddd::View for CategoryScrollView {}
