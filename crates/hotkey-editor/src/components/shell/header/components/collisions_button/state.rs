/// The collision button's two visual states: amber warning while collisions
/// remain, gold "all clear" when the config is clean. Chosen from the live
/// collision count in `From<&CollisionsButtonProps>`, never in the body.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum CollisionState {
    Attention,
    #[default]
    Clear,
}
