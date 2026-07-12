use warcraft_keybinds::GridCoordinate;

/// The marker between (or above) a conflict's abilities: the shared hotkey key for a
/// shared-hotkey clash, or the colliding command-card cell for a position clash.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ConflictMarker {
    Hotkey { label: String },
    Position { coordinate: GridCoordinate },
}
