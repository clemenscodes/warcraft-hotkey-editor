pub mod components;
mod model;
mod presentation;
mod view;

pub use view::EditorPageView;
mod style;

use components::editor_tabs_bar::EditorTabsBar;
use components::editor_workspace::EditorWorkspace;
use dioxus::prelude::*;
use model::EditorPageModel;
use presentation::use_editor_page;
use style::CLASS;
use tw_macro::assert_component;

/// The editor page: the mode/race tab bar above the workspace (the unit list and the
/// unit detail panel). A route target under the shell layout — it reconciles its URL
/// into the shell's navigation signals, then composes its two children, which read the
/// rest of the editor's state from context themselves. It owns the gap between the two
/// sections, so neither child spaces itself with a margin.
#[component]
pub fn EditorPage(props: EditorPageModel) -> Element {
    use_editor_page(&props);
    rsx! {
        div {
            class: CLASS,
            EditorTabsBar {}
            EditorWorkspace {}
        }
    }
}

assert_component!(EditorPage);
