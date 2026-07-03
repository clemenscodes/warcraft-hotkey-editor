use crate::DomainEvent;

/// Carries [`DomainEvent`]s from where they are raised to whoever subscribed — a
/// driven [`crate::Port`].
///
/// The application publishes through this interface without knowing the
/// transport; the transport is an [`crate::Adapter`] in the infrastructure layer.
/// In a Dioxus frontend the adapter is a `use_coroutine` channel, because events
/// are one-shot occurrences, not state — a `Signal` would be the wrong home.
pub trait EventBus {
    /// Publishes an event to every subscribed handler.
    fn publish<Event: DomainEvent>(&self, event: Event);
}
