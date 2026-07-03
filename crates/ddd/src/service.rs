use crate::AggregateRoot;
use crate::Command;
use crate::Repository;

/// The application-layer orchestrator for one [`AggregateRoot`]: it holds the
/// live aggregate, owns its [`Repository`], and is the sole place mutations
/// happen.
///
/// A service exposes no way to change the aggregate except [`Service::commit`]
/// (for named methods) or [`Service::dispatch`] (for reified [`Command`]s). Both
/// paths write through to the repository in the same call, so stored state can
/// never drift from the live aggregate. Implementors provide only the three
/// primitives — [`Service::repository`], [`Service::snapshot`], and
/// [`Service::replace`]; the write-through discipline is supplied here and
/// cannot be bypassed.
pub trait Service<Aggregate: AggregateRoot> {
    /// The concrete repository this service persists through.
    type Repository: Repository<Aggregate>;

    /// Returns a handle to the repository backing this service.
    fn repository(&self) -> Self::Repository;

    /// Returns a clone of the current live aggregate.
    fn snapshot(&self) -> Aggregate;

    /// Overwrites the live aggregate with a new value.
    fn replace(&self, aggregate: Aggregate);

    /// Runs a mutation against the aggregate and persists the result in the same
    /// call — the single write-through path every mutation funnels through.
    fn commit<Outcome>(&self, change: impl FnOnce(&mut Aggregate) -> Outcome) -> Outcome {
        let mut aggregate = self.snapshot();
        let outcome = change(&mut aggregate);
        let repository = self.repository();
        repository.save(&aggregate);
        self.replace(aggregate);
        outcome
    }

    /// Executes a named [`Command`] against the aggregate and persists the
    /// result. Sugar over [`Service::commit`] for the reified-command path.
    fn dispatch<CommandType>(&self, command: CommandType) -> CommandType::Outcome
    where
        CommandType: Command<Aggregate>,
    {
        self.commit(|aggregate| command.execute(aggregate))
    }
}
