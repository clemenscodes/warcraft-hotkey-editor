use crate::ApplicationLayer;
use crate::DomainEvent;
use crate::Layered;

/// Reacts to a [`DomainEvent`] that has been published — the subscriber side of
/// the event flow.
///
/// One event can have many independent handlers, each blind to the others: when
/// `CollisionsResolved` fires, one handler refreshes a read model, another logs.
/// Handlers coordinate the application; they never live inside the aggregate that
/// raised the event.
pub trait EventHandler<Event: DomainEvent>: Layered<Layer = ApplicationLayer> {
    /// Handles an event that has occurred.
    fn handle(&self, event: &Event);
}
