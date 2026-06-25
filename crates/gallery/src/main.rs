use dioxus::prelude::*;
use gallery::{Gallery, GalleryMode, StoryFrame};
use hotkey_editor::TAILWIND_STYLES;

mod stories;

fn main() {
    dioxus::launch(GalleryApp);
}

#[component]
fn GalleryApp() -> Element {
    let registry = stories::registry();
    let mode = GalleryLocation::mode().unwrap_or(GalleryMode::Shell { story: None });
    match mode {
        GalleryMode::Shell { story } => {
            let base_path = GalleryLocation::path();
            rsx! {
                document::Stylesheet { href: TAILWIND_STYLES }
                Gallery {
                    registry,
                    base_path,
                    initial_story: story,
                    on_select: move |story_id: String| GalleryLocation::set_story(&story_id),
                }
            }
        }
        GalleryMode::Frame { story } => rsx! {
            document::Stylesheet { href: TAILWIND_STYLES }
            StoryFrame { registry, story_id: story }
        },
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

    fn set_story(story_id: &str) {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Ok(history) = window.history() else {
            return;
        };
        let path = Self::path();
        let url = format!("{path}?gallery&story={story_id}");
        let state = wasm_bindgen::JsValue::NULL;
        let _ = history.replace_state_with_url(&state, "", Some(&url));
    }
}
