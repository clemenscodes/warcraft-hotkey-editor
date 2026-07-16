pub mod components;
mod frame;
mod model;
mod presentation;
mod view;
mod viewport;

pub use view::EditorPageView;
mod style;

use components::editor_tabs_bar::EditorTabsBarView;
use components::editor_workspace::EditorWorkspaceView;
use components::mobile_editor::MobileEditor;
use dioxus::prelude::*;
use dioxus_kit::frame::Page;
use frame::EditorPageFrame;
use model::EditorPageModel;
use presentation::use_editor_page;
use style::CLASS;
use tw_macro::assert_component;
use viewport::use_is_mobile_viewport;

#[component]
pub fn EditorPage(props: EditorPageModel) -> Element {
    use_editor_page(&props);
    let is_mobile = use_is_mobile_viewport();
    if is_mobile {
        return rsx! {
            MobileEditor {}
        };
    }
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
