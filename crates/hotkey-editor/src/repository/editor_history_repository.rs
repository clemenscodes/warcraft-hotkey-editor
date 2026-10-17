use ddd::Adapter;
use ddd::InfrastructureLayer;
use ddd::Layered;
use ddd::SynchronousRepository;
use std::convert::Infallible;
use std::str::FromStr;
use warcraft_keybinds::EditorHistory;
use warcraft_keybinds::EditorHistoryIdentity;

use crate::persistence::editor_history_persistence;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditorHistoryRepository;

impl Layered for EditorHistoryRepository {
    type Layer = InfrastructureLayer;
}

impl Adapter for EditorHistoryRepository {}

impl SynchronousRepository<EditorHistory> for EditorHistoryRepository {
    type Failure = Infallible;

    fn find_by_id(
        &self,
        _identity: &EditorHistoryIdentity,
    ) -> Result<Option<EditorHistory>, Infallible> {
        let Some(stored_text) = editor_history_persistence::load_text() else {
            return Ok(None);
        };
        let parsed_history = EditorHistory::from_str(stored_text.as_str()).ok();
        Ok(parsed_history)
    }

    fn find_many(
        &self,
        identities: &[EditorHistoryIdentity],
    ) -> Result<Vec<EditorHistory>, Infallible> {
        let mut found_histories = Vec::new();
        for identity in identities {
            let Ok(stored_history) = self.find_by_id(identity);
            found_histories.extend(stored_history);
        }
        Ok(found_histories)
    }

    fn save(&self, aggregate: &EditorHistory) -> Result<(), Infallible> {
        let canonical_text = aggregate.to_string();
        editor_history_persistence::save_text(&canonical_text);
        Ok(())
    }

    fn delete(
        &self,
        identity: &EditorHistoryIdentity,
    ) -> Result<Option<EditorHistory>, Infallible> {
        let Ok(stored_history) = self.find_by_id(identity);
        editor_history_persistence::remove_text();
        Ok(stored_history)
    }
}
