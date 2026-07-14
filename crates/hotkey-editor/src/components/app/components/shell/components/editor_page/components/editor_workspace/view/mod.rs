use super::EditorWorkspace;
use super::model::EditorWorkspaceModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The editor workspace's published `View`. Fieldless: the component is parameterless and
/// sources its state from context, so its contract carries no fields. It is also the editor
/// page frame's body region: it `impl Render` and renders the `EditorWorkspace` once, so the
/// page places the published `View` directly, with no ad-hoc region type.
#[derive(Clone, PartialEq, Default)]
pub struct EditorWorkspaceView;

impl ddd::View for EditorWorkspaceView {}

impl Render for EditorWorkspaceView {
    type Model = EditorWorkspaceModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        rsx! {
            EditorWorkspace {
            


            }
        }
    }
}
