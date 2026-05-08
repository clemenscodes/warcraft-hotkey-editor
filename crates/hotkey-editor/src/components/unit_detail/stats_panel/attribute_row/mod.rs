use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub(super) struct AttributeRowProps {
    pub(super) label: &'static str,
    pub(super) value: u32,
    pub(super) per_level: f32,
    pub(super) is_primary: bool,
}

#[component]
pub(super) fn AttributeRow(props: AttributeRowProps) -> Element {
    let label = props.label;
    let value = props.value;
    let per_level = props.per_level;
    let is_primary = props.is_primary;
    let row_class = if is_primary {
        "stat-row attribute-row primary"
    } else {
        "stat-row attribute-row"
    };
    let per_level_text = format!("+{per_level:.1}");
    let value_text = value.to_string();
    rsx! {
        div { class: row_class,
            span { class: "stat-row-label", {label} }
            span { class: "stat-row-value", {value_text} }
            span { class: "stat-row-gain", {per_level_text} }
        }
    }
}
