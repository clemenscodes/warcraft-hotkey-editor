pub mod components;
mod props;

use components::editor_tabs_bar::{EditorTabsBar, EditorTabsBarProps};
use components::editor_workspace::{EditorWorkspace, EditorWorkspaceProps};
use dioxus::prelude::*;
pub use props::EditorPageProps;

/// The editor view: the mode/race tab bar above the workspace (the unit list and the
/// unit detail panel). The sibling of `CollisionsPage` and `ResolvePage` for the
/// editor. A pure composition — no class of its own; it places its two children.
#[component]
pub fn EditorPage(props: EditorPageProps) -> Element {
    let tabs = EditorTabsBarProps::from(&props);
    let workspace = EditorWorkspaceProps::from(&props);
    rsx! {
        EditorTabsBar { ..tabs }
        EditorWorkspace { ..workspace }
    }
}
