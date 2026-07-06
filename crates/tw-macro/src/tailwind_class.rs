/// A single Tailwind utility class. A named-field newtype over the static class
/// string, built only by the [`tw!`](crate::tw) macro (and, internally, by the
/// generated `classes!`/`states!` macros). Making a class list
/// `&[TailwindClass]` rather than a bare `&[&str]` states, in the type, that
/// these strings are Tailwind utilities — an ordinary `&[&str]` of prose is a
/// different type and can never reach the styling machinery. It also gives
/// editor tooling an exact anchor: completion targets `tw![…]` and nothing else.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TailwindClass {
    utility: &'static str,
}

impl TailwindClass {
    /// Wrap a single utility class literal. Only the styling macros call this.
    pub const fn new(utility: &'static str) -> Self {
        Self { utility }
    }

    /// The underlying utility string.
    pub const fn utility(&self) -> &'static str {
        self.utility
    }
}
