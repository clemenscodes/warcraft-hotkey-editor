use warcraft_api::UnitCatalogGroup;

#[derive(Clone, PartialEq)]
pub struct CategoryTrackView {
    pub groups: Vec<UnitCatalogGroup>,
}

impl ddd::Layered for CategoryTrackView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CategoryTrackView {}
