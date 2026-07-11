/// The corner-radius token a framed icon draws, one step of the shared radius
/// scale. A shared value type callers pass as the `radius` named field, so it is
/// re-exported while `FramedIconModel` stays private.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum IconRadius {
    #[default]
    Tile,
    Control,
    Card,
    Hairline,
}
