use super::view::CollisionsPageView;
use dioxus::prelude::*;

/// The collisions page's route parameters: the active collision `?kind=` and the
/// selected list `?entry=`. That is the page's entire URL state — the editor
/// selection is the editor's, not the collisions page's, so it is not carried here;
/// it persists in the shell's signals while this page is shown and reappears in the
/// URL when the editor is next active.
///
/// The per-kind selection signals live in the shell (backing the `?entry=` param, one
/// per kind for per-tab memory) so they outlive leaving the page and feed the URL
/// sync; the page reaches them through context, never the router.
#[derive(Props, Clone, PartialEq)]
pub struct CollisionsPageProps {
    pub kind: Option<String>,
    pub entry: Option<String>,
}

impl From<&CollisionsPageView> for CollisionsPageProps {
    fn from(view: &CollisionsPageView) -> Self {
        let CollisionsPageView { kind, entry } = view.clone();
        Self { kind, entry }
    }
}

impl ddd::Props for CollisionsPageProps {
    type View = CollisionsPageView;
}
