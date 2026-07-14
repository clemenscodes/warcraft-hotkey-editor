use super::EditorWorkspace;
use super::model::EditorWorkspaceModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

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
