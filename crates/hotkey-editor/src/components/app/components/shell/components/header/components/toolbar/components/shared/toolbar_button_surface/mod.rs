pub mod components;
mod logic;
mod props;
mod state;

use components::attention_surface::{AttentionSurface, AttentionSurfaceProps};
use components::clear_surface::{ClearSurface, ClearSurfaceProps};
use components::interactive_surface::{InteractiveSurface, InteractiveSurfaceProps};
use dioxus::prelude::*;
pub use props::ToolbarButtonSurfaceProps;
pub use state::SurfaceState;
use tw_macro::assert_component;
assert_component!(ToolbarButtonSurface);

/// The clickable surface of a toolbar button: the single source of truth for how a
/// toolbar action button looks. A pure dispatcher — from the resting [`SurfaceState`]
/// it renders the matching look: `InteractiveSurface` xor `AttentionSurface` xor
/// `ClearSurface`. Each look owns its own `<button>` root and its full chrome, drawn in
/// `cqi` off the container so the whole button scales as one drawing; this dispatcher
/// only builds each look's props from the shared `ToolbarButtonSurfaceProps` and renders
/// the one the state selects. Consumers swap the icon, the click handler, aria/disabled
/// state, and the resting look (the inline actions use `Interactive`; the collisions
/// button uses `Attention` / `Clear`).
#[component]
pub fn ToolbarButtonSurface(props: ToolbarButtonSurfaceProps) -> Element {
    match props.state {
        SurfaceState::Interactive => {
            let surface = InteractiveSurfaceProps::from(&props);
            rsx! {
                InteractiveSurface { ..surface }
            }
        }
        SurfaceState::Attention => {
            let surface = AttentionSurfaceProps::from(&props);
            rsx! {
                AttentionSurface { ..surface }
            }
        }
        SurfaceState::Clear => {
            let surface = ClearSurfaceProps::from(&props);
            rsx! {
                ClearSurface { ..surface }
            }
        }
    }
}
