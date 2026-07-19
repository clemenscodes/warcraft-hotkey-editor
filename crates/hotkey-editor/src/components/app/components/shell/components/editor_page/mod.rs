pub mod components;
mod frame;
mod model;
mod presentation;
mod view;

pub use view::EditorPageView;
mod style;

use components::editor_tabs_bar::EditorTabsBarView;
use components::editor_workspace::EditorWorkspaceView;
use components::mobile_editor::MobileEditor;
use components::mobile_race_nav::MobileRaceNav;
use dioxus::prelude::*;
use dioxus_kit::frame::Page;
use frame::EditorPageFrame;
use model::EditorPageModel;
use crate::services::viewport::use_is_touch_viewport;
use presentation::use_editor_page;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn EditorPage(props: EditorPageModel) -> Element {
    use_editor_page(&props);
    let is_touch = use_is_touch_viewport();
    if is_touch {
        return rsx! {
            MobileRaceNav {}
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
