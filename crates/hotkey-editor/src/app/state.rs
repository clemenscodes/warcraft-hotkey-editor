/// The app shell's layout state. The collisions view is a single full-bleed page and
/// drops the inter-section gap; every other view keeps the standard section gaps.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum AppLayout {
    #[default]
    Standard,
    Collisions,
}
