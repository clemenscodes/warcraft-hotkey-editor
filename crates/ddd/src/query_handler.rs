use crate::ApplicationLayer;
use crate::Layered;
use crate::Query;

/// Answers a [`Query`]: the read-side counterpart to a [`crate::CommandHandler`].
///
/// It reads from whatever serves reads best — the aggregate, a
/// [`crate::ReadModel`], a projection — and returns the query's declared output,
/// never mutating domain state along the way.
pub trait QueryHandler<TheQuery: Query>: Layered<Layer = ApplicationLayer> {
    /// Answers the query.
    fn handle(&self, query: TheQuery) -> TheQuery::Output;
}
