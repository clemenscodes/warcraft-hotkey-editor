use crate::AggregateRoot;

/// Persistence for a single kind of [`AggregateRoot`]: the boundary between the
/// domain and whatever actually stores it — local storage, a file, a database.
///
/// A repository deals in whole aggregates. It never exposes partial rows and
/// never lets a caller mutate stored state in place: loading yields a fully
/// reconstituted aggregate, and saving replaces the stored aggregate wholesale.
pub trait Repository<Aggregate: AggregateRoot> {
    /// Reads the currently stored aggregate, or `None` if nothing is stored yet.
    fn load(&self) -> Option<Aggregate>;

    /// Persists the aggregate, replacing any previously stored value.
    fn save(&self, aggregate: &Aggregate);
}
