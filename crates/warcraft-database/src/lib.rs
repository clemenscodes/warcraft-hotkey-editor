pub mod catalog;
mod db;
mod test;

pub use catalog::{BuildingTraits, CommandCatalog};
pub use db::WARCRAFT_DATABASE;
pub use db::WARCRAFT_GAMEPLAY_CONSTANTS;
pub use db::WARCRAFT_SYSTEM_KEYBINDS;
