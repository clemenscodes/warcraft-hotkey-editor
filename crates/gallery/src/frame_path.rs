use crate::percent::Percent;

/// Builds the iframe source URL for a story. Pure, so the gallery never reads
/// the browser to construct it; the host supplies the page path.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FramePath {
    base_path: String,
}

impl FramePath {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }

    pub fn src(&self, story_id: &str) -> String {
        let encoded = Percent::encode(story_id);
        format!("{}?gallery=frame&story={}", self.base_path, encoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn src_appends_frame_query() {
        let path = FramePath::new("/warcraft-hotkey-editor/".to_string());
        assert_eq!(
            path.src("Buttons/Primary"),
            "/warcraft-hotkey-editor/?gallery=frame&story=Buttons/Primary",
        );
    }

    #[test]
    fn src_escapes_spaces_in_story_id() {
        let path = FramePath::new("/base/".to_string());
        assert_eq!(
            path.src("Dialog header/Default"),
            "/base/?gallery=frame&story=Dialog%20header/Default",
        );
    }
}
