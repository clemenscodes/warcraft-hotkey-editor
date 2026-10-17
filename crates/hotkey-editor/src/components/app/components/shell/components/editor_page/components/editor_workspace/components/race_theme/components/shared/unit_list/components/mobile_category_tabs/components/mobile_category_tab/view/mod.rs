use warcraft_api::UnitKind;

#[derive(Clone, PartialEq)]
pub struct MobileCategoryTabView {
    pub kind: UnitKind,
}

impl ddd::Layered for MobileCategoryTabView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MobileCategoryTabView {}
