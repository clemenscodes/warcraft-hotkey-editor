use warcraft_api::SystemHotkeysCategory;

#[derive(Clone, PartialEq)]
pub struct SystemHotkeysListViewView {
    pub category: SystemHotkeysCategory,
}

impl ddd::Layered for SystemHotkeysListViewView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SystemHotkeysListViewView {}
