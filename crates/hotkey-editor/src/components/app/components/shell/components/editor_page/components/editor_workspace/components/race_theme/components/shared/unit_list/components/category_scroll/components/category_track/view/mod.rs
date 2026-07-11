use warcraft_api::UnitKind;

/// The published `View` contract mirroring [`CategoryTrackModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CategoryTrackView {
    pub sections: Vec<UnitKind>,
}

impl ddd::View for CategoryTrackView {}
