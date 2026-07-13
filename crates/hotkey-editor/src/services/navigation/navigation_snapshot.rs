use crate::services::navigation::app_view::CollisionKind;
use crate::services::navigation::editor_navigation::DecodedEditorNavigation;

/// The navigable state a URL encodes, decoded per page. Each variant carries **only
/// the state that page owns**: the editor its race/mode/unit/search, the collisions
/// page its active kind and selected entry, the resolve page its selected entry. The
/// editor selection deliberately does not appear on the other two — it is not their
/// state; it persists across a view switch through the shell's signals, which outlive
/// the page, and reappears in the URL only when the editor is the active page.
///
/// It is also the shell's URL guard: a `Signal<NavigationSnapshot>` holds what the address
/// bar currently shows, updated in lockstep by the shell (after it pushes) and by
/// each page (after it reconciles a route the browser navigated to). The shell's push
/// effect diffs the snapshot the live signals describe against this guard — peeked,
/// never subscribed — so a browser back/forward is reconciled into the signals
/// without the effect firing and echoing the navigation straight back.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum NavigationSnapshot {
    Editor(DecodedEditorNavigation),
    Collisions {
        kind: CollisionKind,
        entry: Option<String>,
    },
    Resolve {
        entry: Option<String>,
    },
}
