use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub(super) struct UnitCardInfoProps {
    pub(super) display_name: String,
    pub(super) unit_id: String,
    pub(super) id_class: &'static str,
}

#[component]
pub(super) fn UnitCardInfo(props: UnitCardInfoProps) -> Element {
    let display_name = props.display_name;
    let unit_id = props.unit_id;
    let id_class = props.id_class;
    rsx! {
        div { class: "flex flex-col gap-[0.45rem] min-w-0 flex-1",
            span {
                class: "text-[1.05rem] leading-[1.25] pb-[0.1rem] overflow-hidden \
                        text-ellipsis whitespace-nowrap min-w-0 min-[1900px]:text-[1.35rem]",
                {display_name}
            }
            code { class: id_class, {unit_id} }
        }
    }
}
