#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum GhostState {
    #[default]
    Default,
    Command,
}
