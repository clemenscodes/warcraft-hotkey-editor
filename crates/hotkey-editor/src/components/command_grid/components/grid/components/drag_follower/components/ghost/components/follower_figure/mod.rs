mod props;
mod style;

use dioxus::prelude::*;

use style::FOLLOWER_FIGURE_STYLES;

pub use props::FollowerFigureProps;

#[component]
pub fn FollowerFigure(props: FollowerFigureProps) -> Element {
    let FollowerFigureProps { src, alt } = props;
    rsx! {
        document::Stylesheet { href: FOLLOWER_FIGURE_STYLES }
        img { class: "follower-figure", src, alt, decoding: "async" }
    }
}
