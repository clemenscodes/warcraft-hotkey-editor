pub mod components;
mod hooks;
mod props;

use components::editor_tabs_bar::EditorTabsBar;
use components::editor_workspace::EditorWorkspace;
use dioxus::prelude::*;
use hooks::{EditorPageModel, use_editor_page};
pub use props::EditorPageProps;

/// The editor page: the mode/race tab bar above the workspace (the unit list and the
/// unit detail panel). A route target under the shell layout — it reconciles its URL
/// into the shell's navigation signals and reads the rest of the editor's state from
/// context, then composes its two children. A pure composition — no class of its own.
use tw_macro::assert_component;
assert_component!(EditorPage);
#[component]
pub fn EditorPage(props: EditorPageProps) -> Element {
    let EditorPageModel { tabs, workspace } = use_editor_page(&props);
    rsx! {
        EditorTabsBar { ..tabs }
        EditorWorkspace { ..workspace }
    }
}
