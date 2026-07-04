use crate::app::shell::Shell;
use crate::components::views::collisions_page::CollisionsPage;
use crate::components::views::editor_page::EditorPage;
use crate::components::views::resolve_page::ResolvePage;
use dioxus::prelude::*;

/// The application's three client-side pages, each a real route under one shared
/// [`Shell`] layout.
///
/// The view now lives in the URL **path** — `/` is the editor, `/collisions` and
/// `/resolve` its siblings — instead of the old `?view=` query parameter, so the
/// router owns the editor/collisions/resolve distinction and the back/forward
/// buttons step between pages natively. Each route carries **only its own state** as
/// query parameters: the editor its `?race=&mode=&unit=&q=` selection, the collisions
/// page its `?kind=&entry=`, the resolve page its `?entry=`. Every field is an
/// `Option<String>`: an absent parameter arrives as `None`, and each page reconciles
/// its own parameters into the shell's signals.
///
/// The editor selection is not repeated on the collisions/resolve routes — it is the
/// editor's state, not theirs. Because the `Shell` layout stays mounted while the
/// `Outlet` swaps the active page, the editor selection (and the loaded keys, and the
/// grid layout) persists across page navigation in the shell's signals and reappears
/// in the URL when the editor is next shown — the role the old one-page `Workbench`
/// used to fill.
#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(Shell)]
    #[route("/?:race&:mode&:unit&:q", EditorPage)]
    Editor {
        race: Option<String>,
        mode: Option<String>,
        unit: Option<String>,
        q: Option<String>,
    },
    #[route("/collisions?:kind&:entry", CollisionsPage)]
    Collisions {
        kind: Option<String>,
        entry: Option<String>,
    },
    // Any unmatched path redirects to the editor at parse time — the app only ever
    // generates its three known routes, so this is reached solely by a stray or stale
    // URL, and sending it home is friendlier than the router's default error screen.
    // A redirect rule renders nothing, so there is no page component here: routing
    // infrastructure stays in the routing layer, not in a would-be no-op component.
    #[redirect("/:..segments", |segments: Vec<String>| {
        let _ = segments;
        Route::Editor {
            race: None,
            mode: None,
            unit: None,
            q: None,
        }
    })]
    #[route("/resolve?:entry", ResolvePage)]
    Resolve { entry: Option<String> },
}
