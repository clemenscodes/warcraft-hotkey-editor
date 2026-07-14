pub mod components;
mod frame;
mod model;
mod presentation;
mod view;

pub use view::EditorPageView;
mod style;

use components::editor_tabs_bar::EditorTabsBarView;
use components::editor_workspace::EditorWorkspaceView;
use dioxus::prelude::*;
use dioxus_kit::frame::Page;
use frame::EditorPageFrame;
use model::EditorPageModel;
use presentation::use_editor_page;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn EditorPage(props: EditorPageModel) -> Element {
    use_editor_page(&props);
    let header = EditorTabsBarView;
    let body = EditorWorkspaceView;
    let frame = EditorPageFrame { header, body };
    rsx! {
        Page {
            class: CLASS,
            frame,
        }
    }
}

assert_component!(EditorPage);
