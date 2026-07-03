use crate::DomainEvent;
use crate::DomainLayer;
use crate::Layered;

/// A domain rule that reacts to something that happened: given a
/// [`DomainEvent`], it decides what should follow.
///
/// "When a slot is moved and hotkey rewriting is on, reassign the hotkey" is a
/// policy — a when/then rule expressed as domain logic rather than buried in a
/// handler. Its reaction is usually a set of follow-up commands; the associated
/// type keeps that open, because the commands are aggregate-specific.
pub trait Policy<Event: DomainEvent>: Layered<Layer = DomainLayer> {
    /// What the policy decides should happen — typically follow-up commands.
    type Reaction;

    /// Decides the reaction to an event that has occurred.
    fn react(&self, event: &Event) -> Self::Reaction;
}
