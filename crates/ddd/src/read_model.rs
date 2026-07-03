/// A denormalised structure shaped for reading, not for enforcing invariants.
///
/// The write model (the [`crate::AggregateRoot`]) is optimised for consistency; a
/// read model is optimised for a specific view — the collisions list, the resolve
/// worklist. It is derived, disposable, and rebuildable from the write model or
/// from events via a [`crate::Projection`], so it holds no authority of its own.
/// An intent marker: it labels a type as read-side, derived state.
pub trait ReadModel {}
