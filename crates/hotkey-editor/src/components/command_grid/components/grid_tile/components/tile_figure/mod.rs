mod props;
mod style;

use dioxus::prelude::*;

use style::TILE_FIGURE_STYLES;

pub use props::TileFigureProps;

#[component]
pub fn TileFigure(props: TileFigureProps) -> Element {
    let alt = props.alt;
    if let Some(src) = props.icon {
        return rsx! {
            document::Stylesheet { href: TILE_FIGURE_STYLES }
            img { class: "tile-figure", src, alt, loading: "lazy", decoding: "async" }
        };
    }
    if !props.is_focusable {
        return rsx! {};
    }
    rsx! {
        document::Stylesheet { href: TILE_FIGURE_STYLES }
        span { class: "tile-figure-label", { alt } }
    }
}
