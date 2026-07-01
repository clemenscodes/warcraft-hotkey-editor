/// The visual state of the drag follower ghost, driving its background.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum GhostState {
    #[default]
    Default,
    Command,
}
