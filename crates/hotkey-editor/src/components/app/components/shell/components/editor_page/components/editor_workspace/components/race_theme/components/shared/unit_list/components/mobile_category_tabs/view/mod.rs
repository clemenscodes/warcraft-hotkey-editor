use warcraft_api::UnitKind;

#[derive(Clone, PartialEq)]
pub struct MobileCategoryTabsView {
    pub tabs: Vec<UnitKind>,
}

impl ddd::Layered for MobileCategoryTabsView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MobileCategoryTabsView {}
