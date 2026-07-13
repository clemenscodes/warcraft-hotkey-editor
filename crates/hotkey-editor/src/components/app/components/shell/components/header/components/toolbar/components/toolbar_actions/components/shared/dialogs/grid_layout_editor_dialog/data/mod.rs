//! Static content for the layout editor: the key picker's QWERTY rows and the
//! two instruction lines shown above the grid.

/// The QWERTY rows the key picker offers when editing a grid cell.
pub(super) const QWERTY_ROWS: &[&[char]] = &[
    &['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'],
    &['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L'],
    &['Z', 'X', 'C', 'V', 'B', 'N', 'M'],
];

/// The two instruction lines rendered above the grid.
pub(super) const INTRO_LINES: &[&str] = &[
    "Define a hotkey letter for each button position.",
    "Click apply to rewrite every ability hotkey to match this grid layout.",
];
