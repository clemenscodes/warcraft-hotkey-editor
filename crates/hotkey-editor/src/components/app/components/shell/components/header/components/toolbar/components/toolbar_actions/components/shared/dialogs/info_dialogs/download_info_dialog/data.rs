//! The download dialog's copy: the header title, the intro line, the amber
//! warning callout, and the primary button's label.

/// The dialog title.
pub(super) const TITLE: &str = "Download CustomKeys.txt";

/// The lead-in line above the filename chip.
pub(super) const INTRO: &str = "Place the file in your Documents folder, inside Warcraft III, then CustomKeyBindings. The filename must be exactly:";

/// The amber callout about the fixed filename and saved positions.
pub(super) const WARNING: &str = "Any other filename will not be detected by Warcraft III. Note: button positions in saved custom games are fixed at save time and will not update, even if hotkeys change.";

/// The primary button's label.
pub(super) const PRIMARY_LABEL: &str = "Download";
