use warcraft_keybinds::{GridCoordinate, HotkeyToken};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DraggingSlot {
    grid_id: &'static str,
    coordinate: GridCoordinate,
}

impl DraggingSlot {
    pub fn new(grid_id: &'static str, coordinate: GridCoordinate) -> Self {
        Self {
            grid_id,
            coordinate,
        }
    }

    pub fn grid_id(&self) -> &'static str {
        self.grid_id
    }

    pub fn coordinate(&self) -> GridCoordinate {
        self.coordinate
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DropTargetTile {
    grid_id: &'static str,
    coordinate: GridCoordinate,
}

impl DropTargetTile {
    pub fn new(grid_id: &'static str, coordinate: GridCoordinate) -> Self {
        Self {
            grid_id,
            coordinate,
        }
    }

    pub fn grid_id(&self) -> &'static str {
        self.grid_id
    }

    pub fn coordinate(&self) -> GridCoordinate {
        self.coordinate
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct DragFollowerVisual {
    icon_source: String,
    label_text: String,
    displayed_letter: HotkeyToken,
    is_passive_command: bool,
    is_command_cell: bool,
}

impl DragFollowerVisual {
    pub fn new(
        icon_source: String,
        label_text: String,
        displayed_letter: HotkeyToken,
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

    pub fn icon_source(&self) -> &str {
        &self.icon_source
    }

    pub fn label_text(&self) -> &str {
        &self.label_text
    }

    pub fn displayed_letter(&self) -> HotkeyToken {
        self.displayed_letter
    }

    pub fn is_passive_command(&self) -> bool {
        self.is_passive_command
    }

    pub fn is_command_cell(&self) -> bool {
        self.is_command_cell
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct DragFollower {
    visual: DragFollowerVisual,
    click_offset_horizontal: f64,
    click_offset_vertical: f64,
    cursor_horizontal_position: f64,
    cursor_vertical_position: f64,
    tile_width: f64,
    tile_height: f64,
}

impl DragFollower {
    pub fn new(
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

    pub fn visual(&self) -> &DragFollowerVisual {
        &self.visual
    }

    pub fn tile_width(&self) -> f64 {
        self.tile_width
    }

    pub fn tile_height(&self) -> f64 {
        self.tile_height
    }

    pub fn left(&self) -> f64 {
        self.cursor_horizontal_position - self.click_offset_horizontal
    }

    pub fn top(&self) -> f64 {
        self.cursor_vertical_position - self.click_offset_vertical
    }

    pub fn set_cursor_position(
        &mut self,
        cursor_horizontal_position: f64,
        cursor_vertical_position: f64,
    ) {
        self.cursor_horizontal_position = cursor_horizontal_position;
        self.cursor_vertical_position = cursor_vertical_position;
    }
}
