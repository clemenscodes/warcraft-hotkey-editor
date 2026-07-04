use crate::registry::StoryRegistry;
use dioxus::prelude::*;
use hotkey_editor::components::app::components::shell::components::toasts::ToastMount;

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
                ToastMount {
                    div { class: "min-h-dvh flex flex-col items-center", {render()} }
                }
            }
        }
        None => {
            rsx! {
                div {
                    class: "min-h-dvh flex flex-col items-center p-8 [font-family:system-ui,sans-serif]",
                    "Unknown story: {story_id}"
                }
            }
        }
    }
}
