use dioxus::prelude::*;

use crate::registry::StoryRegistry;

#[derive(Props, Clone, PartialEq)]
pub struct StoryFrameProps {
    pub registry: StoryRegistry,
    pub story_id: String,
}

#[component]
pub fn StoryFrame(props: StoryFrameProps) -> Element {
    let registry = props.registry;
    let story_id = props.story_id;
    match registry.find(&story_id) {
        Some(story) => {
            let render = story.render();
            rsx! {
                div { class: "gallery-frame-root", {render()} }
            }
        }
        None => rsx! {
            div { class: "gallery-frame-root gallery-frame-missing",
                "Unknown story: {story_id}"
            }
        },
    }
}
