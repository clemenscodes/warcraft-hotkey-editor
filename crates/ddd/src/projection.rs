use crate::DomainEvent;

/// Builds a [`crate::ReadModel`] by folding [`DomainEvent`]s into it — the bridge
/// from the write side to a read-optimised shape.
///
/// A projection subscribes to events and updates a denormalised view: fold every
/// `CollisionIntroduced` / `CollisionResolved` into a running "current
/// collisions" list, so the collisions view reads one ready structure instead of
/// recomputing over all bindings each render.
pub trait Projection<Event: DomainEvent> {
    /// Folds one event into the read model.
    fn apply(&mut self, event: &Event);
}
