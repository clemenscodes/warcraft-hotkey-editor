use crate::percent::Percent;

/// The parsed launch intent read from the page query string.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum GalleryMode {
    Shell { story: Option<String> },
    Frame { story: String },
}

impl GalleryMode {
    pub fn from_query(query: &str) -> Option<Self> {
        let trimmed = query.strip_prefix('?').unwrap_or(query);
        let mut has_gallery = false;
        let mut gallery_value: Option<String> = None;
        let mut story: Option<String> = None;
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
                _ => {}
            }
        }
        if !has_gallery {
            return None;
        }
        match gallery_value.as_deref() {
            Some("frame") => story.map(|story| Self::Frame { story }),
            _ => Some(Self::Shell { story }),
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
        assert_eq!(
            GalleryMode::from_query("?gallery"),
            Some(GalleryMode::Shell { story: None })
        );
    }

    #[test]
    fn gallery_with_story_is_shell_with_story() {
        assert_eq!(
            GalleryMode::from_query("?gallery=1&story=Buttons/Primary"),
            Some(GalleryMode::Shell {
                story: Some("Buttons/Primary".to_string())
            })
        );
    }

    #[test]
    fn frame_requires_a_story() {
        assert_eq!(
            GalleryMode::from_query("?gallery=frame&story=Buttons/Primary"),
            Some(GalleryMode::Frame {
                story: "Buttons/Primary".to_string()
            })
        );
        assert_eq!(GalleryMode::from_query("?gallery=frame"), None);
    }

    #[test]
    fn frame_story_is_percent_decoded() {
        assert_eq!(
            GalleryMode::from_query("?gallery=frame&story=Dialog%20header/Default"),
            Some(GalleryMode::Frame {
                story: "Dialog header/Default".to_string()
            })
        );
    }
}
