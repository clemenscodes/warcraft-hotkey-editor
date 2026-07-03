use crate::AggregateRoot;
use crate::ApplicationLayer;
use crate::Layered;

/// A named, self-contained intention to change one [`AggregateRoot`].
///
/// A command carries everything the change needs and applies it through the
/// aggregate root, so the root can enforce its invariants. Commands are the only
/// vocabulary a [`crate::Service`] accepts: no "set this field" back door, only
/// named use-cases. Executing a command consumes it, because an intention is
/// spent once carried out. A command is an application-layer input.
///
/// This trait fuses the intention with its execution for ergonomics. Strict CQRS
/// keeps them apart — the command as data, a [`crate::CommandHandler`] as the
/// executor — which the vocabulary also offers.
pub trait Command<Aggregate: AggregateRoot>: Layered<Layer = ApplicationLayer> {
    /// What executing the command reports back — often the [`crate::DomainEvent`]s
    /// it produced.
    type Outcome;

    /// Applies this command to the aggregate and returns its outcome.
    fn execute(self, aggregate: &mut Aggregate) -> Self::Outcome;
}
