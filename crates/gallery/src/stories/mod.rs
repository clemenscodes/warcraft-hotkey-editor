use dioxus_gallery::{Story, StoryRegistry};

mod buttons;
mod carriers;
mod dialogs;
mod editor_mount;
pub mod fixtures;
mod grid;
mod keys_mount;
mod shell;
mod system_hotkeys;
mod system_hotkeys_state_mount;
mod tabs;
mod tile_override;
mod toast_mount;
mod toggle_button;
mod unit_detail;
mod unit_list;
mod views;

/// Builds the full set of stories the gallery shows, one module per editor
/// component group.
pub fn registry() -> StoryRegistry {
    let mut stories: Vec<Story> = Vec::new();
    stories.extend(buttons::stories());
    stories.extend(grid::stories());
    stories.extend(dialogs::stories());
    stories.extend(carriers::stories());
    stories.extend(shell::stories());
    stories.extend(system_hotkeys::stories());
    stories.extend(tabs::stories());
    stories.extend(tile_override::stories());
    stories.extend(toggle_button::stories());
    stories.extend(unit_detail::stories());
    stories.extend(unit_list::stories());
    stories.extend(views::stories());
    stories.into_iter().collect()
}
