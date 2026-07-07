use crate::components::app::components::shell::components::resolve_page::logic::ReasonKind;
use tw_macro::tw;

classes! {
    base: tw![
        "flex-none",
        "inline-flex",
        "items-center",
        "px-3",
        "py-1",
        "rounded-tile",
        "text-lg",
        "uppercase",
        "tracking-[0.04em]",
        "border",
        "border-solid",
        "text-shadow-drop",
        "whitespace-nowrap",
    ],
}

states! {
    ReasonKind,
    Fight => tw![
        "text-race-orc",
        "border-race-orc/60",
        "bg-race-orc/12",
    ],
    GapPull => tw![
        "text-warcraft-success",
        "border-warcraft-success/60",
        "bg-warcraft-success/12",
    ],
    Spill => tw![
        "text-race-human",
        "border-race-human/60",
        "bg-race-human/12",
    ],
    Swap => tw![
        "text-race-undead",
        "border-race-undead/60",
        "bg-race-undead/12",
    ],
    Stuck => tw![
        "text-race-orc",
        "border-race-orc/60",
        "bg-race-orc/12",
    ],
}
