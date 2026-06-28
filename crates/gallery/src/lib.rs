use dioxus::prelude::*;

mod frame;
mod frame_path;
mod mode;
mod percent;
mod registry;
mod shell;
mod story;
mod view;
mod viewport;

pub(crate) const GALLERY_STYLES: Asset = asset!("/src/gallery.css");

pub use frame::StoryFrame;
pub use frame_path::FramePath;
pub use mode::GalleryMode;
pub use registry::{StoryComponent, StoryGroup, StoryRegistry};
pub use shell::Gallery;
pub use story::Story;
pub use view::GalleryView;
pub use viewport::ViewportPreset;
