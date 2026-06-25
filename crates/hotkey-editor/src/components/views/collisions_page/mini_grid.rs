use dioxus::prelude::*;
use warcraft_keybinds::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS};

#[derive(Props, Clone, PartialEq)]
pub struct IslandMiniGridProps {
    pub collision_column: u8,
    pub collision_row: u8,
}

/// A tiny empty 4x3 command grid with only the island's conflicting button
/// highlighted.  Shared by the sidebar island cards and the detail header so
/// both render the exact same shape from the exact same markup.
#[component]
pub fn IslandMiniGrid(props: IslandMiniGridProps) -> Element {
    let collision_column = props.collision_column;
    let collision_row = props.collision_row;

    rsx! {
        div { class: "island-mini-grid",
            for row in 0..COMMAND_GRID_ROWS {
                for column in 0..COMMAND_GRID_COLUMNS {
                    {
                        let is_collision = column == collision_column && row == collision_row;
                        let cell_class = if is_collision {
                            "island-mini-cell collision"
                        } else {
                            "island-mini-cell"
                        };
                        rsx! {
                            div { key: "{row}-{column}", class: cell_class }
                        }
                    }
                }
            }
        }
    }
}
