/// A category tab is either the selected category (gold, highlighted) or one of the
/// others (dimmed gold). Chosen in the hook.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum SystemHotkeysCategoryTabState {
    #[default]
    Inactive,
    Active,
}
