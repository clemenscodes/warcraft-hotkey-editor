/// The capture pulse of an editable keycap: resting, or lit gold while its key
/// picker is open. The pulse look is identical wherever the keycap is used, so it
/// lives here rather than on each host.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum EditableKeycapState {
    #[default]
    Idle,
    Editing,
}
