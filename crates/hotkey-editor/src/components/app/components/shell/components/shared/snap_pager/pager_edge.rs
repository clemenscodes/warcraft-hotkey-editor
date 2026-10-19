/// An end of the mounted cards the scrolling has come near.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(crate) enum PagerEdge {
    #[default]
    Start,
    End,
}
