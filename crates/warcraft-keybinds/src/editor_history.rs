use std::fmt;
use std::str::FromStr;

/// Maximum number of snapshots kept per stack. Each snapshot is the full
/// canonical state; the persistence layer deflate-compresses the blob, so a deep
/// history still fits localStorage.
const MAX_DEPTH: usize = 40;

/// Separators chosen from the ASCII control range so they can never appear in the
/// INI-style CustomKeys text or the grid-layout storage string. Field separates
/// the two fields of one snapshot; record separates snapshots within a stack;
/// group separates the present/undo/redo sections.
const FIELD_SEPARATOR: char = '\u{1f}';
const RECORD_SEPARATOR: char = '\u{1e}';
const GROUP_SEPARATOR: char = '\u{1d}';

/// One complete, restorable editor state: the canonical keys text plus the grid
/// layout storage string. Because localStorage already holds the entire
/// normalized state as a single string, a snapshot of that string (plus the
/// layout) is the whole app state, so every action is captured uniformly.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct EditorSnapshot {
    keys_text: String,
    grid_layout_text: String,
}

impl EditorSnapshot {
    pub fn new(keys_text: String, grid_layout_text: String) -> Self {
        Self {
            keys_text,
            grid_layout_text,
        }
    }

    pub fn keys_text(&self) -> &str {
        self.keys_text.as_str()
    }

    pub fn grid_layout_text(&self) -> &str {
        self.grid_layout_text.as_str()
    }
}

impl fmt::Display for EditorSnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}{FIELD_SEPARATOR}{}",
            self.keys_text, self.grid_layout_text
        )
    }
}

impl FromStr for EditorSnapshot {
    type Err = ();

    fn from_str(encoded: &str) -> Result<Self, Self::Err> {
        let mut fields = encoded.splitn(2, FIELD_SEPARATOR);
        let keys_text = fields.next().ok_or(())?.to_owned();
        let grid_layout_text = fields.next().ok_or(())?.to_owned();
        let snapshot = Self {
            keys_text,
            grid_layout_text,
        };
        Ok(snapshot)
    }
}

/// A single global undo/redo timeline backed by full-state snapshots. `record`
/// pushes the current present onto the undo stack and clears redo; `undo`/`redo`
/// walk the timeline and return the snapshot that becomes present, for the caller
/// to apply to live state. `present` is the live cursor and is transient across a
/// reload (the persistence round-trip carries it, but the application layer
/// reseats it to the actual boot state), while the two stacks are the persisted
/// history.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct EditorHistory {
    undo_stack: Vec<EditorSnapshot>,
    redo_stack: Vec<EditorSnapshot>,
    present: EditorSnapshot,
}

impl EditorHistory {
    pub fn new(present: EditorSnapshot) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            present,
        }
    }

    pub fn present(&self) -> &EditorSnapshot {
        &self.present
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Records a transition to `current`. A no-op (returns `false`) when `current`
    /// equals the present state — which is exactly what happens right after
    /// undo/redo restores a snapshot, so restores never create new history.
    pub fn record(&mut self, current: EditorSnapshot) -> bool {
        if current == self.present {
            return false;
        }
        let previous = std::mem::replace(&mut self.present, current);
        self.undo_stack.push(previous);
        while self.undo_stack.len() > MAX_DEPTH {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
        true
    }

    /// Steps one entry back, returning the snapshot that is now present (for the
    /// caller to apply to live state), or `None` when there is nothing to undo.
    pub fn undo(&mut self) -> Option<EditorSnapshot> {
        let restored = self.undo_stack.pop()?;
        let previous_present = std::mem::replace(&mut self.present, restored.clone());
        self.redo_stack.push(previous_present);
        Some(restored)
    }

    /// Steps one entry forward, the mirror of [`EditorHistory::undo`].
    pub fn redo(&mut self) -> Option<EditorSnapshot> {
        let restored = self.redo_stack.pop()?;
        let previous_present = std::mem::replace(&mut self.present, restored.clone());
        self.undo_stack.push(previous_present);
        Some(restored)
    }

    /// Replaces the live cursor without touching the stacks. Used after loading a
    /// persisted timeline to seat it on the actual boot state (the editor content
    /// comes from its own storage key, which is authoritative).
    pub fn reseat_present(&mut self, present: EditorSnapshot) {
        self.present = present;
    }

    fn encode_stack(stack: &[EditorSnapshot]) -> String {
        let record_separator = RECORD_SEPARATOR.to_string();
        let encoded: Vec<String> = stack.iter().map(EditorSnapshot::to_string).collect();
        encoded.join(&record_separator)
    }

    fn parse_stack(part: &str) -> Vec<EditorSnapshot> {
        if part.is_empty() {
            return Vec::new();
        }
        part.split(RECORD_SEPARATOR)
            .filter_map(|entry| EditorSnapshot::from_str(entry).ok())
            .collect()
    }
}

impl ddd::Layered for EditorHistory {
    type Layer = ddd::DomainLayer;
}

impl ddd::AggregateRoot for EditorHistory {}

impl fmt::Display for EditorHistory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let undo_text = Self::encode_stack(&self.undo_stack);
        let redo_text = Self::encode_stack(&self.redo_stack);
        write!(
            formatter,
            "{}{GROUP_SEPARATOR}{undo_text}{GROUP_SEPARATOR}{redo_text}",
            self.present
        )
    }
}

impl FromStr for EditorHistory {
    type Err = ();

    fn from_str(encoded: &str) -> Result<Self, Self::Err> {
        let mut groups = encoded.splitn(3, GROUP_SEPARATOR);
        let present_part = groups.next().unwrap_or_default();
        let undo_part = groups.next().unwrap_or_default();
        let redo_part = groups.next().unwrap_or_default();
        let present = EditorSnapshot::from_str(present_part).unwrap_or_default();
        let history = Self {
            undo_stack: Self::parse_stack(undo_part),
            redo_stack: Self::parse_stack(redo_part),
            present,
        };
        Ok(history)
    }
}

#[cfg(test)]
mod tests {
    use super::EditorHistory;
    use super::EditorSnapshot;
    use super::MAX_DEPTH;
    use ddd::AggregateRoot;
    use ddd::DomainLayer;
    use ddd::Layered;
    use std::str::FromStr;

    fn snapshot(marker: &str) -> EditorSnapshot {
        EditorSnapshot::new(format!("keys-{marker}"), format!("grid-{marker}"))
    }

    fn assert_domain_aggregate<Aggregate>()
    where
        Aggregate: AggregateRoot + Layered<Layer = DomainLayer>,
    {
    }

    #[test]
    fn editor_history_is_a_domain_aggregate_root() {
        assert_domain_aggregate::<EditorHistory>();
    }

    #[test]
    fn record_pushes_present_and_clears_redo() {
        let mut history = EditorHistory::new(snapshot("a"));
        let recorded = history.record(snapshot("b"));
        assert!(recorded);
        assert!(history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.present(), &snapshot("b"));
    }

    #[test]
    fn recording_the_present_again_is_a_no_op() {
        let mut history = EditorHistory::new(snapshot("a"));
        let recorded = history.record(snapshot("a"));
        assert!(!recorded);
        assert!(!history.can_undo());
    }

    #[test]
    fn undo_then_redo_round_trips_the_present() {
        let mut history = EditorHistory::new(snapshot("a"));
        history.record(snapshot("b"));
        let undone = history.undo();
        assert_eq!(undone, Some(snapshot("a")));
        assert_eq!(history.present(), &snapshot("a"));
        assert!(history.can_redo());
        let redone = history.redo();
        assert_eq!(redone, Some(snapshot("b")));
        assert_eq!(history.present(), &snapshot("b"));
    }

    #[test]
    fn undo_on_empty_stack_returns_none() {
        let mut history = EditorHistory::new(snapshot("a"));
        assert_eq!(history.undo(), None);
    }

    #[test]
    fn undo_stack_is_capped_at_max_depth() {
        let mut history = EditorHistory::new(snapshot("0"));
        for index in 1..=(MAX_DEPTH + 5) {
            history.record(snapshot(&index.to_string()));
        }
        let mut undo_count = 0;
        while history.undo().is_some() {
            undo_count += 1;
        }
        assert_eq!(undo_count, MAX_DEPTH);
    }

    #[test]
    fn serialize_round_trips_the_whole_timeline() {
        let mut history = EditorHistory::new(snapshot("a"));
        history.record(snapshot("b"));
        history.record(snapshot("c"));
        history.undo();
        let encoded = history.to_string();
        let decoded = EditorHistory::from_str(&encoded).expect("history decodes");
        assert_eq!(decoded, history);
    }
}
