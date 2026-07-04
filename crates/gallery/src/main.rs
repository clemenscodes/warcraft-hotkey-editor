use dioxus::prelude::*;
use gallery::{Gallery, GalleryMode, GalleryView, StoryFrame};
use hotkey_editor::components::app::TAILWIND_STYLES;

mod stories;

fn main() {
    dioxus::launch(GalleryApp);
}

#[component]
fn GalleryApp() -> Element {
    let registry = stories::registry();
    let default_view = GalleryView::default();
    let default_mode = GalleryMode::Shell { view: default_view };
    let mode = GalleryLocation::mode().unwrap_or(default_mode);
    match mode {
        GalleryMode::Shell { view } => {
            let base_path = GalleryLocation::path();
            rsx! {
                document::Stylesheet { href: TAILWIND_STYLES }
                Gallery {
                    registry,
                    base_path,
                    initial_view: view,
                    on_change: move | view : GalleryView |
                                    GalleryLocation::set_view(& view),
                }
            }
        }
        GalleryMode::Frame { story } => {
            rsx! {
                document::Stylesheet { href: TAILWIND_STYLES }
                StoryFrame { registry, story_id: story }
            }
        }
    }
}

/// Reads and writes the gallery app's own page URL. The `gallery` library
/// stays free of browser bindings; this app owns all `web_sys` access.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct GalleryLocation;

impl GalleryLocation {
    fn mode() -> Option<GalleryMode> {
        let window = web_sys::window()?;
        let search = window.location().search().ok()?;
        GalleryMode::from_query(&search)
    }

    fn path() -> String {
        web_sys::window()
            .and_then(|window| window.location().pathname().ok())
            .unwrap_or_default()
    }

    fn set_view(view: &GalleryView) {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Ok(history) = window.history() else {
            return;
        };
        let path = Self::path();
        let body = view.to_query();
        let url = format!("{path}?{body}");
        let state = wasm_bindgen::JsValue::NULL;
        let _ = history.replace_state_with_url(&state, "", Some(&url));
    }
}
