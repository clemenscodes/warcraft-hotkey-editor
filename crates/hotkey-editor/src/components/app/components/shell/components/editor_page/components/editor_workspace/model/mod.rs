use super::view::EditorWorkspaceView;

#[derive(Clone, PartialEq, Default)]
pub struct EditorWorkspaceModel;

impl From<&EditorWorkspaceView> for EditorWorkspaceModel {
    fn from(_view: &EditorWorkspaceView) -> Self {
        Self
    }
}

impl ddd::Model for EditorWorkspaceModel {
    type View = EditorWorkspaceView;
}
