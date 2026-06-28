use std::collections::HashSet;

use dioxus::prelude::*;

use crate::GALLERY_STYLES;
use crate::frame_path::FramePath;
use crate::registry::StoryRegistry;
use crate::story::Story;
use crate::view::GalleryView;
use crate::viewport::ViewportPreset;

/// A component and the subset of its stories that pass the current search,
/// precomputed so a group with no surviving stories can be skipped before its
/// header is rendered.
struct VisibleComponent {
    name: &'static str,
    collapsible: bool,
    stories: Vec<Story>,
}

#[derive(Props, Clone, PartialEq)]
pub struct GalleryProps {
    pub registry: StoryRegistry,
    pub base_path: String,
    pub initial_view: GalleryView,
    pub on_change: EventHandler<GalleryView>,
}

#[component]
pub fn Gallery(props: GalleryProps) -> Element {
    let registry = props.registry;
    let base_path = props.base_path;
    let on_change = props.on_change;
    let initial_view = props.initial_view;

    let initial_story = initial_view.story().map(str::to_string);
    let initial_query = initial_view.query().to_string();
    let initial_width = initial_view.viewport_width();
    let initial_height = initial_view.viewport_height();
    let initial_hidden = initial_view.list_hidden();

    let starting = initial_story.or_else(|| registry.first_id());
    let mut selected = use_signal::<Option<String>>(move || starting.clone());
    let mut width = use_signal::<u32>(move || initial_width);
    let mut height = use_signal::<u32>(move || initial_height);
    let mut sidebar_width = use_signal::<f64>(|| 256.0);
    let mut dragging = use_signal::<bool>(|| false);
    let mut sidebar_hidden = use_signal::<bool>(move || initial_hidden);
    let mut stage_width = use_signal::<f64>(|| 0.0);
    let mut stage_height = use_signal::<f64>(|| 0.0);
    let mut query = use_signal::<String>(move || initial_query.clone());
    let mut expanded_components = use_signal::<HashSet<String>>(HashSet::new);
    let mut collapsed_groups = use_signal::<HashSet<String>>(HashSet::new);

    use_effect(move || {
        let current_story = selected.read().clone();
        let current_query = query.read().clone();
        let current_width = width();
        let current_height = height();
        let current_hidden = sidebar_hidden();
        let view = GalleryView::new(
            current_story,
            current_query,
            current_width,
            current_height,
            current_hidden,
        );
        on_change.call(view);
    });

    let registry_for_expand = registry.clone();
    use_effect(move || {
        let current_story = selected.read().clone();
        if let Some(story_id) = current_story
            && let Some(story) = registry_for_expand.find(&story_id)
        {
            let group = story.group();
            let component = story.component();
            let key = format!("{group}/{component}");
            expanded_components.write().insert(key);
        }
    });

    let groups = registry.groups();
    let presets = ViewportPreset::defaults();
    let needle = query().to_lowercase();

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
            if !sidebar_hidden() {
                nav { class: "gallery-sidebar", style: "width: {sidebar_width}px",
                    input {
                        class: "gallery-search",
                        r#type: "search",
                        placeholder: "Search components…",
                        value: "{query}",
                        oninput: move |event| {
                            let typed = event.value();
                            query.set(typed);
                        },
                    }
                    for group in groups {
                        {
                            let group_name = group.name();
                            let group_key = group_name.to_string();
                            let group_lower = group_name.to_lowercase();
                            let searching = !needle.is_empty();
                            let group_matches = group_lower.contains(&needle);
                            let mut visible_components: Vec<VisibleComponent> = Vec::new();
                            for component in group.components() {
                                let component_name = component.name();
                                let component_lower = component_name.to_lowercase();
                                let component_matches =
                                    group_matches || component_lower.contains(&needle);
                                let visible_stories = component
                                    .stories()
                                    .iter()
                                    .copied()
                                    .filter(|story| {
                                        if needle.is_empty() || component_matches {
                                            return true;
                                        }
                                        let label = story.label().to_lowercase();
                                        label.contains(&needle)
                                    })
                                    .collect::<Vec<_>>();
                                if visible_stories.is_empty() {
                                    continue;
                                }
                                let entry = VisibleComponent {
                                    name: component_name,
                                    collapsible: component.is_collapsible(),
                                    stories: visible_stories,
                                };
                                visible_components.push(entry);
                            }
                            if visible_components.is_empty() {
                                rsx! {}
                            } else {
                                let group_collapsed = collapsed_groups.read().contains(&group_key);
                                let group_open = searching || !group_collapsed;
                                let group_chevron = if group_open { "▾" } else { "▸" };
                                let toggle_group_key = group_key.clone();
                                rsx! {
                                    section { key: "{group_name}",
                                        button {
                                            class: "gallery-group-title",
                                            onclick: move |_| {
                                                let mut set = collapsed_groups.write();
                                                if set.contains(&toggle_group_key) {
                                                    set.remove(&toggle_group_key);
                                                } else {
                                                    let key = toggle_group_key.clone();
                                                    set.insert(key);
                                                }
                                            },
                                            span { class: "gallery-chevron", "{group_chevron}" }
                                            "{group_name}"
                                        }
                                        if group_open {
                                            for entry in visible_components {
                                                {
                                                    let component_name = entry.name;
                                                    let collapsible = entry.collapsible;
                                                    let component_stories = entry.stories;
                                                    let component_key =
                                                        format!("{group_name}/{component_name}");
                                                    if !collapsible {
                                                        let story = component_stories[0];
                                                        let story_id = story.id();
                                                        let click_id = story_id.clone();
                                                        let is_selected = selected.read().as_deref()
                                                            == Some(story_id.as_str());
                                                        let item_class = if is_selected {
                                                            "gallery-item gallery-leaf selected"
                                                        } else {
                                                            "gallery-item gallery-leaf"
                                                        };
                                                        rsx! {
                                                            button {
                                                                key: "{component_key}",
                                                                class: item_class,
                                                                onclick: move |_| {
                                                                    let next = Some(click_id.clone());
                                                                    selected.set(next);
                                                                },
                                                                "{component_name}"
                                                            }
                                                        }
                                                    } else {
                                                        let selected_now = selected.read().clone();
                                                        let has_selected_child = component_stories
                                                            .iter()
                                                            .any(|story| {
                                                                let id = story.id();
                                                                selected_now.as_deref()
                                                                    == Some(id.as_str())
                                                            });
                                                        let in_set = expanded_components
                                                            .read()
                                                            .contains(&component_key);
                                                        let component_open = searching || in_set;
                                                        let chevron =
                                                            if component_open { "▾" } else { "▸" };
                                                        let header_class = if has_selected_child {
                                                            "gallery-component-header active"
                                                        } else {
                                                            "gallery-component-header"
                                                        };
                                                        let toggle_key = component_key.clone();
                                                        let first_story = component_stories.first();
                                                        let first_id =
                                                            first_story.map(|story| story.id());
                                                        rsx! {
                                                            div {
                                                                key: "{component_key}",
                                                                class: "gallery-component",
                                                                button {
                                                                    class: header_class,
                                                                    onclick: move |_| {
                                                                        let mut set =
                                                                            expanded_components.write();
                                                                        if set.contains(&toggle_key) {
                                                                            set.remove(&toggle_key);
                                                                        } else {
                                                                            let key = toggle_key.clone();
                                                                            set.insert(key);
                                                                            drop(set);
                                                                            if let Some(id) = &first_id {
                                                                                let next =
                                                                                    Some(id.clone());
                                                                                selected.set(next);
                                                                            }
                                                                        }
                                                                    },
                                                                    span {
                                                                        class: "gallery-chevron",
                                                                        "{chevron}"
                                                                    }
                                                                    "{component_name}"
                                                                }
                                                                if component_open {
                                                                    for story in component_stories {
                                                                        {
                                                                            let story_id = story.id();
                                                                            let click_id =
                                                                                story_id.clone();
                                                                            let label = story.label();
                                                                            let is_selected = selected
                                                                                .read()
                                                                                .as_deref()
                                                                                == Some(
                                                                                    story_id.as_str(),
                                                                                );
                                                                            let item_class =
                                                                                if is_selected {
                                                                                    "gallery-item gallery-variant selected"
                                                                                } else {
                                                                                    "gallery-item gallery-variant"
                                                                                };
                                                                            rsx! {
                                                                                button {
                                                                                    key: "{story_id}",
                                                                                    class: item_class,
                                                                                    onclick: move |_| {
                                                                                        let next = Some(
                                                                                            click_id
                                                                                                .clone(),
                                                                                        );
                                                                                        selected
                                                                                            .set(next);
                                                                                    },
                                                                                    "{label}"
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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
            }
            div { class: "gallery-main",
                div { class: "gallery-toolbar",
                    button {
                        class: "gallery-preset",
                        onclick: move |_| {
                            let hidden = sidebar_hidden();
                            let next = !hidden;
                            sidebar_hidden.set(next);
                        },
                        if sidebar_hidden() {
                            "Show list"
                        } else {
                            "Hide list"
                        }
                    }
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
                div {
                    class: "gallery-stage",
                    onresize: move |event| {
                        if let Ok(content_box) = event.get_content_box_size() {
                            let measured_width = content_box.width;
                            let measured_height = content_box.height;
                            stage_width.set(measured_width);
                            stage_height.set(measured_height);
                        }
                    },
                    if let Some(story_id) = selected.read().clone() {
                        {
                            let frame_path = FramePath::new(base_path.clone());
                            let source = frame_path.src(&story_id);
                            let frame_width = f64::from(width());
                            let frame_height = f64::from(height());
                            let available_width = stage_width();
                            let available_height = stage_height();
                            let width_ratio = available_width / frame_width;
                            let height_ratio = available_height / frame_height;
                            let fitted = width_ratio.min(height_ratio).min(1.0);
                            let unmeasured = available_width <= 0.0 || available_height <= 0.0;
                            let scale = if unmeasured { 1.0 } else { fitted };
                            let scaled_width = frame_width * scale;
                            let scaled_height = frame_height * scale;
                            let scaler_style = format!(
                                "width: {scaled_width}px; height: {scaled_height}px",
                            );
                            let frame_style = format!(
                                "transform: scale({scale}); transform-origin: top left",
                            );
                            rsx! {
                                div { class: "gallery-frame-scaler", style: "{scaler_style}",
                                    iframe {
                                        class: "gallery-frame",
                                        src: "{source}",
                                        style: "{frame_style}",
                                        width: "{width}",
                                        height: "{height}",
                                    }
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
