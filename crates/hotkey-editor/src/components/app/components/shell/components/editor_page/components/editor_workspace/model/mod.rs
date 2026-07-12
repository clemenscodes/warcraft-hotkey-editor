use super::view::EditorWorkspaceView;

/// The editor workspace's ddd model. The component sources its state from context in its
/// children, so this is the fieldless published contract the editor page's frame body region
/// names as its `Render::Model`.
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
