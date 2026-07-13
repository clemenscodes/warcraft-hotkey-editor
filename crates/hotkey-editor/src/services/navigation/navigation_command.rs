use crate::services::navigation::navigation_snapshot::NavigationSnapshot;

/// Whether a navigation writes a **new** browser-history entry or overwrites the current
/// one. The choice is made at the mutation site, not derived after the fact: a page or
/// context change (a view switch, a race/mode/unit change) pushes; an entry pick, or a
/// search keystroke past the first, replaces. Deciding here is what removes the need to
/// diff the old and new routes after a navigation to guess whether it was a push.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NavigationHistoryMode {
    Push,
    Replace,
}

/// A single navigation intent handed from the navigation service to the shell: the
/// fully-typed target state the address bar should show, plus whether to push or replace
/// it. The service builds one of these and invokes the shell-supplied callback with it;
/// the shell — the only place that may name the concrete `Route` — turns the snapshot
/// into a route and pushes or replaces. This keeps the routing type in the component
/// layer while the navigation service stays route-agnostic.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NavigationCommand {
    snapshot: NavigationSnapshot,
    history_mode: NavigationHistoryMode,
}

impl NavigationCommand {
    pub fn new(snapshot: NavigationSnapshot, history_mode: NavigationHistoryMode) -> Self {
        Self {
            snapshot,
            history_mode,
        }
    }

    pub fn snapshot(&self) -> &NavigationSnapshot {
        &self.snapshot
    }

    pub fn history_mode(&self) -> NavigationHistoryMode {
        self.history_mode
    }
}
