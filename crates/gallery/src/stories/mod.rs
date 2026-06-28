use gallery::{Story, StoryRegistry};

mod buttons;
mod command_grid;
mod dialogs;
pub mod fixtures;
mod shared;
mod shell;
mod system_hotkeys;
mod tabs;
mod tile_override;
mod unit_detail;
mod unit_list;
mod views;

/// Builds the full set of stories the gallery shows, one module per editor
/// component group.
pub fn registry() -> StoryRegistry {
    let mut stories: Vec<Story> = Vec::new();
    stories.extend(buttons::stories());
    stories.extend(command_grid::stories());
    stories.extend(dialogs::stories());
    stories.extend(shared::stories());
    stories.extend(shell::stories());
    stories.extend(system_hotkeys::stories());
    stories.extend(tabs::stories());
    stories.extend(tile_override::stories());
    stories.extend(unit_detail::stories());
    stories.extend(unit_list::stories());
    stories.extend(views::stories());
    stories.into_iter().collect()
}
