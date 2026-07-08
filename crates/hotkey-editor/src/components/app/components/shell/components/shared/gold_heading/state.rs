/// Which weight of the shared uppercase gold heading look to wear. The variant
/// only selects the look's colour, shadow, and font weight; the heading element
/// that wraps this leaf still owns the font size and any per-band layout, which
/// this leaf inherits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GoldHeadingVariant {
    Section,
    Dialog,
    Grid,
    Toast,
}
