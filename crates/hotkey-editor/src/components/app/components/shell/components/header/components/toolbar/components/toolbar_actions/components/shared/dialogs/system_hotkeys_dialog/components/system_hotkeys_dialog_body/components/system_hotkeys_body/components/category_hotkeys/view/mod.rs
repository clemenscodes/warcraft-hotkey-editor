use warcraft_api::SystemHotkeysCategory;

#[derive(Clone, PartialEq)]
pub struct CategoryHotkeysView {
    pub category: SystemHotkeysCategory,
}

impl ddd::Layered for CategoryHotkeysView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CategoryHotkeysView {}
