use crate::services::navigation::navigation_snapshot::NavigationSnapshot;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NavigationHistoryMode {
    Push,
    Replace,
}

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
