#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum IconRadius {
    #[default]
    Tile,
    Control,
    Card,
    Hairline,
}
