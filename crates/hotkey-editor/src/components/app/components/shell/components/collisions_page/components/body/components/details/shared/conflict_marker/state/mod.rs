use warcraft_keybinds::GridCoordinate;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ConflictMarker {
    Hotkey { label: String },
    Position { coordinate: GridCoordinate },
}
