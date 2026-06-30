use dioxus::prelude::*;

mod styling;

mod app;
mod components;
mod model;
mod services;

pub use app::App;
pub use components::actions::export_button::ExportButton;
pub use components::actions::grid_layout_button::GridLayoutButton;
pub use components::actions::help_button::HelpButton;
pub use components::actions::preview_button::PreviewButton;
pub use components::actions::redo_button::RedoButton;
pub use components::actions::resolve_button::ResolveButton;
pub use components::actions::system_hotkeys_button::SystemHotkeysButton;
pub use components::actions::templates_button::TemplatesButton;
pub use components::actions::undo_button::UndoButton;
pub use components::actions::upload_button::UploadButton;
pub use components::dialogs::dialog_header::DialogHeader;
pub use components::dialogs::download_info_dialog::DownloadInfoDialog;
pub use components::dialogs::help_dialog::HelpDialog;
pub use components::dialogs::layout_editor::LayoutEditor;
pub use components::dialogs::preview_dialog::PreviewDialog;
pub use components::dialogs::templates_dialog::TemplatesDialog;
pub use components::dialogs::templates_dialog::components::template_gallery::components::template_card::TemplateCard;
pub use components::dialogs::upload_info_dialog::UploadInfoDialog;
pub use components::grid_editors::command_grid_editor::CommandGridEditor;
pub use components::grid_editors::grid_editor::components::drag_follower_overlay::DragFollowerOverlay;
pub use components::grid_editors::grid_editor::components::headed_grid::HeadedGrid;
pub use components::grid_editors::grid_editor::components::headed_grid::components::grid::Grid;
pub use components::grid_editors::grid_editor::components::headed_grid::components::grid_heading::GridHeading;
pub use components::grid_editors::grid_editor::components::headed_grid::components::grid::components::grid_tile::GridTile;
pub use components::grid_editors::grid_editor::components::headed_grid::components::grid::components::grid_tile::GridTileProps;
pub use components::grid_editors::grid_editor::components::headed_grid::components::grid::components::grid_tile::GridTileState;
pub use components::grid_editors::grid_editor::components::headed_grid::components::grid::components::grid_tile::components::hotkey_badge::HotkeyBadge;
pub use components::grid_editors::grid_editor::components::headed_grid::components::grid::components::grid_tile::components::hotkey_badge::HotkeyBadgeProps;
pub use components::grid_editors::grid_editor::components::headed_grid::components::grid::components::grid_tile::components::hotkey_badge::HotkeyBadgeState;
pub use components::grid_editors::research_grid_editor::ResearchGridEditor;
pub use components::grid_editors::uprooted_grid_editor::UprootedGridEditor;
pub use components::shared::key_picker::KeyPicker;
pub use components::shared::key_picker::KeyPickerCell;
pub use components::shared::key_picker::KeyPickerCellState;
pub use components::shared::toolbar_button::ToolbarButton;
pub use components::shell::footer::Footer;
pub use components::shell::header::BurgerMenu;
pub use components::shell::header::CollisionsButton;
pub use components::shell::header::Header;
pub use components::shell::header::HeaderBrand;
pub use components::shell::header::HeaderToolbar;
pub use components::shell::toasts::ToastMount;
pub use components::shell::tooltips::TooltipMount;
pub use components::system_hotkeys::control_groups::ControlGroupsHotkeysView;
pub use components::system_hotkeys::dialog::SystemHotkeysDialog;
pub use components::system_hotkeys::dialog::breadcrumbs::SystemHotkeysBreadcrumbs;
pub use components::system_hotkeys::dialog::category_tab::SystemHotkeysCategoryTab;
pub use components::system_hotkeys::dialog::dialog_header::SystemHotkeysHeader;
pub use components::system_hotkeys::dialog::inventory_drag_overlay::InventoryDragOverlay;
pub use components::system_hotkeys::hero_selection::HeroSelectionHotkeysView;
pub use components::system_hotkeys::inventory::InventoryHotkeysView;
pub use components::system_hotkeys::inventory_grid::InventoryCell;
pub use components::system_hotkeys::inventory_grid::InventoryDragFollower;
pub use components::system_hotkeys::inventory_grid::InventoryDragSource;
pub use components::system_hotkeys::inventory_grid::InventoryGrid;
pub use components::system_hotkeys::key_cell::KeyCaptureCell;
pub use components::system_hotkeys::key_picker_dialog::SystemKeyPickerDialog;
pub use components::system_hotkeys::list_view::SystemHotkeysListEntry;
pub use components::system_hotkeys::list_view::SystemHotkeysListView;
pub use components::system_hotkeys::slot_button::SlotButton;
pub use components::tabs::mode_and_race_tabs::ModeAndRaceTabs;
pub use components::tabs::race_tabs::RaceTabs;
pub use components::tabs::race_tabs::tab::RaceTab;
pub use components::tile_override::AltPositionPicker;
pub use components::tile_override::TileOverridePanel;
pub use components::tile_override::UpgradePositionPicker;
pub use components::tile_override::alt_state_section::AltStateSection;
pub use components::tile_override::description::AbilityDescription;
pub use components::tile_override::key_field::OverrideKeyField;
pub use components::tile_override::upgrade_section::UpgradeSection;
pub use components::tile_override::upgrade_tier::UpgradeTierSelector;
pub use components::unit_detail::UnitDetailPanel;
pub use components::unit_detail::grids::UnitCommandGrids;
pub use components::unit_detail::header::UnitDetailHeader;
pub use components::unit_detail::header::hero_level_option::HeroLevelOption;
pub use components::unit_detail::stats_panel::UnitStatsPanel;
pub use components::unit_detail::stats_panel::attack_matchup_cell::AttackMatchupCell;
pub use components::unit_detail::stats_panel::attribute_row::AttributeRow;
pub use components::unit_detail::stats_panel::attributes_column::{
    AttributesColumn, HeroDisplayData,
};
pub use components::unit_detail::stats_panel::combat_column::{AttackDisplayData, CombatColumn};
pub use components::unit_detail::stats_panel::damage_matchup_row::DamageMatchupRow;
pub use components::unit_detail::stats_panel::defense_matchup_cell::DefenseMatchupCell;
pub use components::unit_detail::stats_panel::defense_matchup_row::DefenseMatchupRow;
pub use components::unit_detail::stats_panel::stat_icon::StatIcon;
pub use components::unit_list::UnitListPanel;
pub use components::unit_list::category::UnitCategorySection;
pub use components::unit_list::mobile_category_tab::MobileCategoryTab;
pub use components::unit_list::unit_card::UnitCard;
pub use components::unit_list::unit_card::icon::UnitCardIcon;
pub use components::unit_list::unit_card::info::UnitCardInfo;
pub use components::views::collisions_page::CollisionsPage;
pub use components::views::collisions_page::{
    AbilityIconView, ConflictAbilityView, ConflictView, HotkeyConflictView, HotkeyUnitView,
    IslandView, UnitIconView, UnitPositionConflictView, UnitPositionUnitView,
};
pub use components::views::collisions_page::{
    HotkeyUnitDetail, HotkeyUnitSidebar, IslandDetail, IslandMiniGrid, IslandSidebar,
    UnitPositionDetail, UnitPositionSidebar,
};
pub use components::views::resolve_page::ResolvePage;
pub use model::grid::{DragFollower, DragFollowerVisual, DraggingSlot, DropTargetTile};
pub use model::icons::IconUrl;
pub use services::customkeys::upload_status::UploadStatus;
pub use services::navigation::app_view::AppView;
pub use services::navigation::app_view::CollisionKind;
pub use services::navigation::view_navigation::ViewNavigationContext;
pub use services::undo::UndoHistory;

/// The editor's compiled Tailwind stylesheet, exposed so a consumer (the
/// component gallery) can inject it and render the editor's components with
/// their real styling.
pub const TAILWIND_STYLES: Asset = asset!("/assets/tailwind.css");
