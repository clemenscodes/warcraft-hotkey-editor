use super::view::ResolvePageView;
use dioxus::prelude::*;

/// The resolve page's route parameter: the selected move-category `?entry=`
/// breadcrumb. That is the page's entire URL state — the editor selection is the
/// editor's, not the resolve page's, so it is not carried here; it persists in the
/// shell's signals while this page is shown and reappears in the URL when the editor
/// is next active.
///
/// The selected move-category lives in the shell (backing the `?entry=` param so the
/// viewed section deep-links and survives back/forward); the page reaches it through
/// context, never the router.
#[derive(Props, Clone, PartialEq)]
pub struct ResolvePageProps {
    pub entry: Option<String>,
}

impl From<&ResolvePageView> for ResolvePageProps {
    fn from(view: &ResolvePageView) -> Self {
        let ResolvePageView { entry } = view.clone();
        Self { entry }
    }
}

impl ddd::Props for ResolvePageProps {
    type View = ResolvePageView;
}
