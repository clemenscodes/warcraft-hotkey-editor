use super::view::EditorTabsBarView;

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
