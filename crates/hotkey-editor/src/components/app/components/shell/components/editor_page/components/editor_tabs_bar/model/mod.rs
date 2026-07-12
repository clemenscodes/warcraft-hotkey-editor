use super::view::EditorTabsBarView;

/// The editor tabs bar's ddd model. The component sources its state from context in its
/// children, so this is the fieldless published contract the editor page's frame header
/// region names as its `Render::Model`.
#[derive(Clone, PartialEq, Default)]
pub struct EditorTabsBarModel;

impl From<&EditorTabsBarView> for EditorTabsBarModel {
    fn from(_view: &EditorTabsBarView) -> Self {
        Self
    }
}

impl ddd::Model for EditorTabsBarModel {
    type View = EditorTabsBarView;
}
