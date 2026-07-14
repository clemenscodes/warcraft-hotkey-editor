#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum ColorKeyState {
    #[default]
    Available,
    Current,
    Conflict,
}
