/// The color look of a picker key: a free key, the one currently bound here, or a key
/// already taken elsewhere. Chosen from the cell by `key_picker_key`; the key only
/// renders the color it is given. Orthogonal to its width, which its slot owns.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum ColorKeyState {
    #[default]
    Available,
    Current,
    Conflict,
}
