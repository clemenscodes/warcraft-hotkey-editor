#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum BurgerItemState {
    #[default]
    Idle,
    Active,
    Primary,
}
