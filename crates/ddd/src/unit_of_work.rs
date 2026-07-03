/// A transactional boundary: a set of changes that commit together, all or
/// nothing.
///
/// A unit of work batches mutations and flushes them once, so intermediate steps
/// never persist and a failure leaves nothing half-written. In the editor this
/// is already supplied by [`crate::Service::commit`], which mutates a snapshot
/// and saves once — the n=1 case over a single aggregate. This trait is the shape
/// it takes when a real multi-aggregate transaction appears: several repositories
/// coordinated to succeed or roll back as one.
pub trait UnitOfWork {
    /// Why committing the unit of work failed.
    type Error;

    /// Commits every tracked change atomically, consuming the unit of work.
    fn commit(self) -> Result<(), Self::Error>;
}
