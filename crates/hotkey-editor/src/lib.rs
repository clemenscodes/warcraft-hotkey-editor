use dioxus::prelude::*;

mod styling;

mod app;
pub mod components;
mod model;
mod services;

pub use app::App;

// Shared, non-component vocabulary a consumer (the gallery, the binary) needs.
// Components are reached by their full module path under `components::` rather
// than re-exported flat here, so adding a component never touches this file.
pub use model::grid::{DragFollower, DragFollowerVisual, DraggingSlot, DropTargetTile};
pub use model::icons::IconUrl;
pub use services::customkeys::upload_status::UploadStatus;
pub use services::navigation::app_view::AppView;
pub use services::navigation::app_view::CollisionKind;
pub use services::navigation::view_navigation::ViewNavigationContext;
pub use services::overlay_state::OverlayState;
pub use services::undo::UndoHistory;

/// The editor's compiled Tailwind stylesheet, exposed so a consumer (the
/// component gallery) can inject it and render the editor's components with
/// their real styling.
pub const TAILWIND_STYLES: Asset = asset!("/assets/tailwind.css");
