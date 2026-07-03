use crate::DomainLayer;
use crate::Layered;

/// A record that something meaningful has already happened in the domain.
///
/// Domain events are named in the past tense — `SlotMoved`, `HotkeyRebound`,
/// `LayoutApplied` — and are immutable facts, not intentions: a
/// [`crate::Command`] can be refused, whereas an event has already occurred.
/// Events are what a command reports through its [`crate::Command::Outcome`] and
/// what an [`crate::EventBus`] carries to handlers.
///
/// An intent marker with cheap true bounds: an event lives in the domain layer
/// and can always be inspected for logging and replay.
pub trait DomainEvent: std::fmt::Debug + Layered<Layer = DomainLayer> {}
