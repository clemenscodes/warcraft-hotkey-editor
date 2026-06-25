use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnitCardInfoProps {
    pub display_name: String,
    pub unit_id: String,
    pub id_class: &'static str,
}

#[component]
pub fn UnitCardInfo(props: UnitCardInfoProps) -> Element {
    let display_name = props.display_name;
    let unit_id = props.unit_id;
    let id_class = props.id_class;
    rsx! {
        div { class: "unit-card-text",
            span {
                class: "unit-card-name",
                {display_name}
            }
            code { class: id_class, {unit_id} }
        }
    }
}
