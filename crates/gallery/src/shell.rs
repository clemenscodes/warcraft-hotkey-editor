use dioxus::prelude::*;

use crate::GALLERY_STYLES;
use crate::frame_path::FramePath;
use crate::registry::StoryRegistry;
use crate::viewport::ViewportPreset;

#[derive(Props, Clone, PartialEq)]
pub struct GalleryProps {
    pub registry: StoryRegistry,
    pub base_path: String,
    pub initial_story: Option<String>,
    pub on_select: EventHandler<String>,
}

#[component]
pub fn Gallery(props: GalleryProps) -> Element {
    let registry = props.registry;
    let base_path = props.base_path;
    let on_select = props.on_select;

    let starting = props.initial_story.or_else(|| registry.first_id());
    let mut selected = use_signal::<Option<String>>(move || starting.clone());
    let mut width = use_signal::<u32>(|| 1440);
    let mut height = use_signal::<u32>(|| 900);
    let mut sidebar_width = use_signal::<f64>(|| 256.0);
    let mut dragging = use_signal::<bool>(|| false);

    let groups = registry.groups();
    let presets = ViewportPreset::defaults();

    rsx! {
        document::Stylesheet { href: GALLERY_STYLES }
        div {
            class: if dragging() {
                "gallery-shell gallery-shell-dragging"
            } else {
                "gallery-shell"
            },
            onpointermove: move |event| {
                if dragging() {
                    let position = event.client_coordinates().x;
                    let clamped = position.clamp(180.0, 760.0);
                    sidebar_width.set(clamped);
                }
            },
            onpointerup: move |_| dragging.set(false),
            onpointerleave: move |_| dragging.set(false),
            nav { class: "gallery-sidebar", style: "width: {sidebar_width}px",
                for group in groups {
                    section { key: "{group.name()}",
                        h2 { class: "gallery-group-title", "{group.name()}" }
                        for story in group.stories().iter().copied() {
                            {
                                let story_id = story.id();
                                let click_id = story_id.clone();
                                let is_selected = selected.read().as_deref() == Some(story_id.as_str());
                                let item_class = if is_selected {
                                    "gallery-item selected"
                                } else {
                                    "gallery-item"
                                };
                                rsx! {
                                    button {
                                        key: "{story_id}",
                                        class: item_class,
                                        onclick: move |_| {
                                            selected.set(Some(click_id.clone()));
                                            on_select.call(click_id.clone());
                                        },
                                        "{story.name()}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "gallery-resizer",
                onpointerdown: move |event| {
                    event.prevent_default();
                    dragging.set(true);
                },
            }
            div { class: "gallery-main",
                div { class: "gallery-toolbar",
                    for preset in presets {
                        button {
                            key: "{preset.label()}",
                            class: "gallery-preset",
                            onclick: move |_| {
                                width.set(preset.width());
                                height.set(preset.height());
                            },
                            "{preset.label()}"
                        }
                    }
                    label { class: "gallery-dim",
                        "W"
                        input {
                            r#type: "number",
                            value: "{width}",
                            oninput: move |event| {
                                if let Ok(value) = event.value().parse::<u32>() {
                                    width.set(value);
                                }
                            },
                        }
                    }
                    label { class: "gallery-dim",
                        "H"
                        input {
                            r#type: "number",
                            value: "{height}",
                            oninput: move |event| {
                                if let Ok(value) = event.value().parse::<u32>() {
                                    height.set(value);
                                }
                            },
                        }
                    }
                }
                div { class: "gallery-stage",
                    if let Some(story_id) = selected.read().clone() {
                        {
                            let frame_path = FramePath::new(base_path.clone());
                            let source = frame_path.src(&story_id);
                            rsx! {
                                iframe {
                                    class: "gallery-frame",
                                    src: "{source}",
                                    width: "{width}",
                                    height: "{height}",
                                }
                            }
                        }
                    } else {
                        p { "No stories registered." }
                    }
                }
            }
        }
    }
}
