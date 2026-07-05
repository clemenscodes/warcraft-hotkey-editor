use ddd::Adapter;
use ddd::InfrastructureLayer;
use ddd::Layered;
use ddd::Repository;
use std::str::FromStr;
use warcraft_keybinds::EditorHistory;

use crate::persistence::editor_history_persistence::EditorHistoryPersistence;

/// Infrastructure adapter that persists the [`EditorHistory`] aggregate to
/// localStorage, bridging the aggregate's canonical text form and the compressed
/// blob the persistence layer stores.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditorHistoryRepository;

impl Layered for EditorHistoryRepository {
    type Layer = InfrastructureLayer;
}

impl Adapter for EditorHistoryRepository {}

impl Repository<EditorHistory> for EditorHistoryRepository {
    fn load(&self) -> Option<EditorHistory> {
        let stored_text = EditorHistoryPersistence::load_text()?;
        EditorHistory::from_str(stored_text.as_str()).ok()
    }

    fn save(&self, aggregate: &EditorHistory) {
        let canonical_text = aggregate.to_string();
        EditorHistoryPersistence::save_text(&canonical_text);
    }
}
