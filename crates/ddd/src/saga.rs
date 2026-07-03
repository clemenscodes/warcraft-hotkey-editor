use crate::ApplicationLayer;
use crate::Layered;

/// A long-running process that coordinates several aggregates by reacting to
/// events and issuing commands — also called a process manager.
///
/// A saga is the multi-aggregate counterpart to a single command: it advances a
/// workflow across consistency boundaries that cannot be changed in one
/// transaction, keeping them eventually consistent. It is exactly where a real
/// multi-aggregate [`crate::UnitOfWork`] and event-driven [`crate::Policy`]
/// reactions come together — and exactly what the editor does *not* need while it
/// mutates one aggregate. An intent marker in the application layer.
pub trait Saga: Layered<Layer = ApplicationLayer> {}
