use crate::icons::IconUrl;

pub(crate) use warcraft_keybinds::GridSlotId;

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
    icon_source: Option<IconUrl>,
    label_text: String,
    displayed_letter: Option<String>,
    is_passive_command: bool,
    is_command_cell: bool,
}

impl DragFollowerVisual {
    pub(crate) fn new(
        icon_source: Option<IconUrl>,
        label_text: String,
        displayed_letter: Option<String>,
        is_passive_command: bool,
        is_command_cell: bool,
    ) -> Self {
        Self {
            icon_source,
            label_text,
            displayed_letter,
            is_passive_command,
            is_command_cell,
        }
    }

    pub(crate) fn icon_source(&self) -> Option<&str> {
        self.icon_source.as_ref().map(|icon| icon.url())
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
    click_offset_horizontal: f64,
    click_offset_vertical: f64,
    cursor_horizontal_position: f64,
    cursor_vertical_position: f64,
    tile_width: f64,
    tile_height: f64,
}

impl DragFollower {
    pub(crate) fn new(
        visual: DragFollowerVisual,
        click_offset_horizontal: f64,
        click_offset_vertical: f64,
        cursor_horizontal_position: f64,
        cursor_vertical_position: f64,
        tile_width: f64,
        tile_height: f64,
    ) -> Self {
        Self {
            visual,
            click_offset_horizontal,
            click_offset_vertical,
            cursor_horizontal_position,
            cursor_vertical_position,
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
        self.cursor_horizontal_position - self.click_offset_horizontal
    }

    pub(crate) fn top(&self) -> f64 {
        self.cursor_vertical_position - self.click_offset_vertical
    }

    pub(crate) fn set_cursor_position(
        &mut self,
        cursor_horizontal_position: f64,
        cursor_vertical_position: f64,
    ) {
        self.cursor_horizontal_position = cursor_horizontal_position;
        self.cursor_vertical_position = cursor_vertical_position;
    }
}
