#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum SurfaceState {
    #[default]
    Interactive,
    Attention,
    Clear,
}
