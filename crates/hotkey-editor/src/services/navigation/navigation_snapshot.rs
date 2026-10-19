use crate::services::navigation::app_view::CollisionKind;
use crate::services::navigation::editor_navigation::DecodedEditorNavigation;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum NavigationSnapshot {
    Editor(DecodedEditorNavigation),
    Collisions {
        kind: CollisionKind,
        entry: Option<String>,
    },
    Resolve {
        entry: Option<String>,
    },
}
