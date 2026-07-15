pub mod components;
mod model;
mod presentation;
mod view;

pub use view::EditorPageView;
mod style;

use super::shared::warcraft_page::WarcraftPage;
use components::editor_tabs_bar::EditorTabsBarView;
use components::editor_workspace::EditorWorkspaceView;
use dioxus::prelude::*;
use model::EditorPageModel;
use presentation::use_editor_page;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn EditorPage(props: EditorPageModel) -> Element {
    use_editor_page(&props);
    let header = EditorTabsBarView;
    let body = EditorWorkspaceView;
    rsx! {
        div {
            class: CLASS,
            WarcraftPage::<EditorTabsBarView, EditorWorkspaceView> {
                header,
                body,
            }
        }
    }
}

assert_component!(EditorPage);
