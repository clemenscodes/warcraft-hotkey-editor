/// `AbilityOff` is intentionally excluded from a unit's *primary* command
/// card — the off-state of a toggle is bound through the override card
/// instead of competing for a second cell next to its on-state. The
/// variant exists for future reuse (a drag-and-drop picker that wires up
/// `CommandGridSection` directly), and is wired through the position /
/// hotkey resolution paths so adding it back to a slot list "just works".
/// Today's picker is click-to-place and doesn't construct this variant.
#[derive(Clone, PartialEq, Eq, Debug)]
#[allow(dead_code)] // see doc-comment above
pub(crate) enum GridSlotId {
    Ability(String),
    AbilityOff(String),
    Command(String),
}

impl GridSlotId {
    pub(crate) fn ability(value: impl Into<String>) -> Self {
        Self::Ability(value.into())
    }

    #[allow(dead_code)] // matches the AbilityOff variant — see GridSlotId doc.
    pub(crate) fn ability_off(value: impl Into<String>) -> Self {
        Self::AbilityOff(value.into())
    }

    pub(crate) fn command(value: impl Into<String>) -> Self {
        Self::Command(value.into())
    }

    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::Ability(value) => value.as_str(),
            Self::AbilityOff(value) => value.as_str(),
            Self::Command(value) => value.as_str(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct DraggingSlot {
    slot_id: GridSlotId,
    source_section: &'static str,
}

impl DraggingSlot {
    pub(crate) fn new(slot_id: GridSlotId, source_section: &'static str) -> Self {
        Self {
            slot_id,
            source_section,
        }
    }

    pub(crate) fn slot_id(&self) -> &GridSlotId {
        &self.slot_id
    }

    pub(crate) fn source_section(&self) -> &'static str {
        self.source_section
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct DropTargetCell {
    section: &'static str,
    column: u8,
    row: u8,
}

impl DropTargetCell {
    pub(crate) fn new(section: &'static str, column: u8, row: u8) -> Self {
        Self {
            section,
            column,
            row,
        }
    }

    pub(crate) fn section(&self) -> &'static str {
        self.section
    }

    pub(crate) fn column(&self) -> u8 {
        self.column
    }

    pub(crate) fn row(&self) -> u8 {
        self.row
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct DragFollowerVisual {
    icon_src: Option<String>,
    label_text: String,
    displayed_letter: Option<String>,
    is_passive_command: bool,
    is_command_cell: bool,
}

impl DragFollowerVisual {
    pub(crate) fn new(
        icon_src: Option<String>,
        label_text: String,
        displayed_letter: Option<String>,
        is_passive_command: bool,
        is_command_cell: bool,
    ) -> Self {
        Self {
            icon_src,
            label_text,
            displayed_letter,
            is_passive_command,
            is_command_cell,
        }
    }

    pub(crate) fn icon_src(&self) -> Option<&str> {
        self.icon_src.as_deref()
    }

    pub(crate) fn label_text(&self) -> &str {
        &self.label_text
    }

    pub(crate) fn displayed_letter(&self) -> Option<&str> {
        self.displayed_letter.as_deref()
    }

    pub(crate) fn is_passive_command(&self) -> bool {
        self.is_passive_command
    }

    pub(crate) fn is_command_cell(&self) -> bool {
        self.is_command_cell
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct DragFollower {
    visual: DragFollowerVisual,
    click_offset_x: f64,
    click_offset_y: f64,
    cursor_x: f64,
    cursor_y: f64,
    tile_width: f64,
    tile_height: f64,
}

impl DragFollower {
    pub(crate) fn new(
        visual: DragFollowerVisual,
        click_offset_x: f64,
        click_offset_y: f64,
        cursor_x: f64,
        cursor_y: f64,
        tile_width: f64,
        tile_height: f64,
    ) -> Self {
        Self {
            visual,
            click_offset_x,
            click_offset_y,
            cursor_x,
            cursor_y,
            tile_width,
            tile_height,
        }
    }

    pub(crate) fn visual(&self) -> &DragFollowerVisual {
        &self.visual
    }

    pub(crate) fn tile_width(&self) -> f64 {
        self.tile_width
    }

    pub(crate) fn tile_height(&self) -> f64 {
        self.tile_height
    }

    pub(crate) fn left(&self) -> f64 {
        self.cursor_x - self.click_offset_x
    }

    pub(crate) fn top(&self) -> f64 {
        self.cursor_y - self.click_offset_y
    }

    pub(crate) fn set_cursor(&mut self, cursor_x: f64, cursor_y: f64) {
        self.cursor_x = cursor_x;
        self.cursor_y = cursor_y;
    }
}
