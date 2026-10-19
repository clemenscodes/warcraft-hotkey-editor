mod model;
mod view;

pub use view::ObjectIdView;
mod style;
use dioxus::prelude::*;
use model::ObjectIdModel;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn ObjectId(props: ObjectIdModel) -> Element {
    let text = props.text;
    rsx! {
        code {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(ObjectId);
