use crate::components::app::components::shell::Shell;
use crate::components::app::components::shell::components::collisions_page::CollisionsPage;
use crate::components::app::components::shell::components::editor_page::EditorPage;
use crate::components::app::components::shell::components::resolve_page::ResolvePage;
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(Shell)]
    #[redirect("/:..segments", |segments: Vec<String>| {
        let _ = segments;
        Self::default()
    })]
    #[route("/?:race&:mode&:unit&:search_query", EditorPage)]
    Editor {
        race: Option<String>,
        mode: Option<String>,
        unit: Option<String>,
        search_query: Option<String>,
    },
    #[route("/collisions?:kind&:entry", CollisionsPage)]
    Collisions {
        kind: Option<String>,
        entry: Option<String>,
    },
    #[route("/resolve?:entry", ResolvePage)]
    Resolve { entry: Option<String> },
}

impl Default for Route {
    fn default() -> Self {
        Self::Editor {
            race: None,
            mode: None,
            unit: None,
            search_query: None,
        }
    }
}
