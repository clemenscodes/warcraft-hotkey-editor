use warcraft_api::SystemHotkeysCategory;

/// The published `View` contract mirroring [`SystemHotkeysListViewProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SystemHotkeysListViewView {
    pub category: SystemHotkeysCategory,
}

impl ddd::View for SystemHotkeysListViewView {}
