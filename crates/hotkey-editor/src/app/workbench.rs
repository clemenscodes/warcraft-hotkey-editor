use crate::app::hooks::{WorkbenchModel, use_workbench};
use crate::app::nav_params::RouteParams;
use crate::app::{FAVICON, KEYBOARD_NAVIGATION_SCRIPT, TAILWIND_STYLES};
use crate::components::dialogs::help_dialog::HelpDialog;
use crate::components::dialogs::layout_editor::LayoutEditor;
use crate::components::dialogs::preview_dialog::PreviewDialog;
use crate::components::dialogs::system_hotkeys_dialog::SystemHotkeysDialog;
use crate::components::dialogs::templates_dialog::TemplatesDialog;
use crate::components::shell::footer::Footer;
use crate::components::shell::header::Header;
use crate::components::shell::toasts::ToastMount;
use crate::components::views::collisions_page::CollisionsPage;
use crate::components::views::editor_page::EditorPage;
use crate::components::views::resolve_page::ResolvePage;
use crate::services::navigation::app_view::AppView;
use dioxus::prelude::*;

/// The one-page workbench: the whole editor, rendered from the URL's query
/// parameters. It is the target of the single `Route::Workbench` variant, so the
/// Dioxus `Router` owns history and URL synchronisation while this component keeps
/// the reactive signal model the component tree already reads from — decoding the
/// route into those signals on entry, pushing/replacing the route as they change,
/// and re-decoding on back/forward.
#[component]
pub fn Workbench(
    race: Option<String>,
    mode: Option<String>,
    unit: Option<String>,
    q: Option<String>,
    view: Option<String>,
    kind: Option<String>,
    entry: Option<String>,
) -> Element {
    let params = RouteParams {
        race,
        mode,
        unit,
        q,
        view,
        kind,
        entry,
    };
    let WorkbenchModel {
        loaded_keys,
        grid_layout,
        update_hotkeys_on_move,
        active_race,
        unit_mode,
        selected_unit_id,
        selected_slot,
        selected_from_research,
        selected_from_uprooted,
        hotkey_assign_request,
        tier_overrides,
        dragging_slot,
        drop_target_tile,
        drag_follower,
        editing_layout_cell,
        dragging_layout_cell,
        search_query,
        search_field,
        current_view,
        selected_island,
        selected_hotkey_unit,
        selected_unit_position,
        selected_move_category,
        upload_status,
        preview_open,
        system_hotkeys_open,
        help_open,
        layout_dialog_open,
        templates_dialog_open,
        collapsed_categories,
        show_abilityless_units,
        expand_variants,
        app_class,
        handle_keydown,
    } = use_workbench(params);
    rsx! {
        document::Stylesheet { href: TAILWIND_STYLES }
        document::Script { src: KEYBOARD_NAVIGATION_SCRIPT, r#type: "module" }
        document::Link { rel: "icon", r#type: "image/svg+xml", href: FAVICON }
        document::Link { rel: "icon", r#type: "image/x-icon", href: "favicon.ico" }
        document::Link { rel: "apple-touch-icon", href: "icon-192.png" }
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1, viewport-fit=cover",
        }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:title", content: "Warcraft III Hotkey Editor" }
        document::Meta {
            property: "og:description",
            content: "Visual command-card editor for Warcraft III: Reforged. \
                      Drag keys, export CustomKeys.txt — runs entirely in your browser.",
        }
        document::Meta {
            property: "og:image",
            content: "https://clemenscodes.github.io/warcraft-hotkey-editor/og-image.png",
        }
        document::Meta {
            property: "og:url",
            content: "https://clemenscodes.github.io/warcraft-hotkey-editor/",
        }
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        ToastMount {
            div { class: app_class, onkeydown: handle_keydown,
                Header { loaded_keys, upload_status, grid_layout }
                match *current_view.read() {
                    AppView::Editor => rsx! {
                        EditorPage {
                            active_race,
                            unit_mode,
                            selected_unit_id,
                            selected_slot,
                            search_query,
                            search_field,
                            show_abilityless_units,
                            expand_variants,
                            collapsed_categories,
                            selected_from_research,
                            selected_from_uprooted,
                            tier_overrides,
                            dragging_slot,
                            drop_target_tile,
                            drag_follower,
                            loaded_keys,
                            grid_layout,
                            update_hotkeys_on_move,
                            hotkey_assign_request,
                        }
                    },
                    AppView::Collisions { kind } => rsx! {
                        CollisionsPage {
                            kind,
                            loaded_keys,
                            grid_layout,
                            selected_island,
                            selected_hotkey_unit,
                            selected_unit_position,
                        }
                    },
                    AppView::Resolve => rsx! {
                        ResolvePage { loaded_keys, selected_move_category }
                    },
                }
                Footer {}
                if *preview_open.read() {
                    PreviewDialog { loaded_keys, preview_open }
                }
                if *system_hotkeys_open.read() {
                    SystemHotkeysDialog { loaded_keys, system_hotkeys_open }
                }
                if *help_open.read() {
                    HelpDialog { help_open }
                }
                TemplatesDialog {
                    loaded_keys,
                    upload_status,
                    open: templates_dialog_open,
                }
                LayoutEditor {
                    grid_layout,
                    editing_layout_cell,
                    dragging_layout_cell,
                    update_hotkeys_on_move,
                    loaded_keys,
                    open: layout_dialog_open,
                }
            }
        }
    }
}
