use warcraft_api::UnitKind;

#[derive(Clone, PartialEq)]
pub struct CategoryTrackView {
    pub sections: Vec<UnitKind>,
}

impl ddd::View for CategoryTrackView {}
