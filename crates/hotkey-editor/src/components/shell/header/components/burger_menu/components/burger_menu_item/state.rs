/// A drawer row's visual weight: a plain idle action, an active toggle (its
/// dialog/preview is open), or the primary call-to-action (Grid Layout). Chosen
/// by the composed hook that builds the row, never in the body.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum BurgerItemState {
    #[default]
    Idle,
    Active,
    Primary,
}
