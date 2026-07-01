use crate::percent::Percent;
use crate::view::{DEFAULT_VIEWPORT_HEIGHT, DEFAULT_VIEWPORT_WIDTH, GalleryView};

/// The parsed launch intent read from the page query string.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum GalleryMode {
    Shell { view: GalleryView },
    Frame { story: String },
}

impl GalleryMode {
    pub fn from_query(query: &str) -> Option<Self> {
        let trimmed = query.strip_prefix('?').unwrap_or(query);
        let mut has_gallery = false;
        let mut gallery_value: Option<String> = None;
        let mut story: Option<String> = None;
        let mut search_query = String::new();
        let mut viewport_width = DEFAULT_VIEWPORT_WIDTH;
        let mut viewport_height = DEFAULT_VIEWPORT_HEIGHT;
        let mut list_hidden = false;
        for pair in trimmed.split('&') {
            if pair.is_empty() {
                continue;
            }
            let mut parts = pair.splitn(2, '=');
            let key = parts.next().unwrap_or_default();
            let value = parts.next();
            match key {
                "gallery" => {
                    has_gallery = true;
                    gallery_value = value.map(str::to_string);
                }
                "story" => {
                    story = value.map(Percent::decode);
                }
                "q" => {
                    if let Some(raw) = value {
                        search_query = Percent::decode(raw);
                    }
                }
                "w" => {
                    let parsed = value.and_then(|raw| raw.parse::<u32>().ok());
                    if let Some(width) = parsed {
                        viewport_width = width;
                    }
                }
                "h" => {
                    let parsed = value.and_then(|raw| raw.parse::<u32>().ok());
                    if let Some(height) = parsed {
                        viewport_height = height;
                    }
                }
                "list" if value == Some("hidden") => {
                    list_hidden = true;
                }
                _ => {}
            }
        }
        if !has_gallery {
            return None;
        }
        match gallery_value.as_deref() {
            Some("frame") => story.map(|story| Self::Frame { story }),
            _ => {
                let view = GalleryView::new(
                    story,
                    search_query,
                    viewport_width,
                    viewport_height,
                    list_hidden,
                );
                Some(Self::Shell { view })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_gallery_param_is_none() {
        assert_eq!(GalleryMode::from_query("?foo=bar"), None);
        assert_eq!(GalleryMode::from_query(""), None);
    }

    #[test]
    fn bare_gallery_is_shell_without_story() {
        let view = GalleryView::default();
        assert_eq!(
            GalleryMode::from_query("?gallery"),
            Some(GalleryMode::Shell { view }),
        );
    }

    #[test]
    fn gallery_with_story_is_shell_with_story() {
        let story = Some("Buttons/Primary".to_string());
        let default_view = GalleryView::default();
        let query = default_view.query().to_string();
        let width = default_view.viewport_width();
        let height = default_view.viewport_height();
        let hidden = default_view.list_hidden();
        let view = GalleryView::new(story, query, width, height, hidden);
        assert_eq!(
            GalleryMode::from_query("?gallery=1&story=Buttons/Primary"),
            Some(GalleryMode::Shell { view }),
        );
    }

    #[test]
    fn gallery_parses_search_viewport_and_list_state() {
        let story = Some("Shell/Header".to_string());
        let query = "head".to_string();
        let view = GalleryView::new(story, query, 1920, 1080, true);
        assert_eq!(
            GalleryMode::from_query("?gallery&story=Shell/Header&q=head&w=1920&h=1080&list=hidden",),
            Some(GalleryMode::Shell { view }),
        );
    }

    #[test]
    fn frame_requires_a_story() {
        assert_eq!(
            GalleryMode::from_query("?gallery=frame&story=Buttons/Primary"),
            Some(GalleryMode::Frame {
                story: "Buttons/Primary".to_string(),
            }),
        );
        assert_eq!(GalleryMode::from_query("?gallery=frame"), None);
    }

    #[test]
    fn frame_story_is_percent_decoded() {
        assert_eq!(
            GalleryMode::from_query("?gallery=frame&story=Dialog%20header/Default"),
            Some(GalleryMode::Frame {
                story: "Dialog header/Default".to_string(),
            }),
        );
    }
}
