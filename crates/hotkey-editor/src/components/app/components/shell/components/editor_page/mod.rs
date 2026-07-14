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

/// The editor page: the mode/race tab bar above the workspace (the unit list and the
/// unit detail panel). A route target under the shell layout — it reconciles its URL
/// into the shell's navigation signals, then composes the headless `Page` frame from its
/// header (the tab bar) and body (the workspace) regions, which read the rest of the
/// editor's state from context themselves. It owns the gap between the two sections via its
/// `CLASS` on the `Page` container, so neither child spaces itself with a margin.
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
