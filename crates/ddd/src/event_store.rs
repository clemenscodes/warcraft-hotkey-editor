use crate::DomainEvent;

/// Append-only persistence for [`DomainEvent`]s — the backbone of event sourcing,
/// and a driven [`crate::Port`].
///
/// Instead of storing current state, an event-sourced system stores the full
/// ordered history of what happened and rebuilds state by replaying it. The
/// editor does not need this — localStorage holds the materialised `CustomKeys` —
/// but it is the natural home for a complete, replayable undo history should
/// snapshots ever become too heavy.
pub trait EventStore<Event: DomainEvent + Clone> {
    /// Appends an event to the end of the stream.
    fn append(&self, event: &Event);

    /// Loads the full event stream in the order it was appended.
    fn load(&self) -> Vec<Event>;
}
