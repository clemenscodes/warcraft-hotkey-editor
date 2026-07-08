/// Which card wears the shared panel surface: a plan move card (resolved or the
/// orc-tinted stuck variant) or a centered collision conflict card. Selects the
/// gap, padding, alignment, and border colour on top of the common surface.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PanelCardVariant {
    Move,
    MoveStuck,
    Conflict,
}
