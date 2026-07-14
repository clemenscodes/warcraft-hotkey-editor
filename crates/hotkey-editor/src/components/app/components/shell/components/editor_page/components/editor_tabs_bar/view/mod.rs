use super::EditorTabsBar;
use super::model::EditorTabsBarModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The editor tabs bar's published `View`. Fieldless: the component is parameterless and
/// sources its state from context, so its contract carries no fields. It is also the editor
/// page frame's header region: it `impl Render` and renders the `EditorTabsBar` once, so the
/// page places the published `View` directly, with no ad-hoc region type.
#[derive(Clone, PartialEq, Default)]
pub struct EditorTabsBarView;

impl ddd::View for EditorTabsBarView {}

impl Render for EditorTabsBarView {
    type Model = EditorTabsBarModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        rsx! {
            EditorTabsBar {
            


            }
        }
    }
}
