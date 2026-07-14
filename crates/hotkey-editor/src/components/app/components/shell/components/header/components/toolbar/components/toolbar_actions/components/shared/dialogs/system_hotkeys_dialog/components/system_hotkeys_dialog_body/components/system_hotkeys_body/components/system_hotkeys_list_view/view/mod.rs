use warcraft_api::SystemHotkeysCategory;

#[derive(Clone, PartialEq)]
pub struct SystemHotkeysListViewView {
    pub category: SystemHotkeysCategory,
}

impl ddd::View for SystemHotkeysListViewView {}
