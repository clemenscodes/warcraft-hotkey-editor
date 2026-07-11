/// Which of the picker's two columns a key belongs to. It carries the only two
/// differences between the columns: the keyboard anchors its caps by position
/// (leftmost, rightmost, or interior) and marks its oversized caps wide, while the
/// numpad right-anchors every cap and never marks one wide.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum BoardSection {
    Keyboard,
    Numpad,
}
