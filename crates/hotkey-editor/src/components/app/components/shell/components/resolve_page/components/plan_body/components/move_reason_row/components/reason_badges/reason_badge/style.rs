use super::state::ReasonBadgeColor;
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
        "tracking-label",
        "border",
        "border-solid",
        "text-shadow-drop",
        "whitespace-nowrap",
    ],
}

states! {
    ReasonBadgeColor,
    Orc => tw![
        "text-race-orc",
        "border-race-orc/60",
        "bg-race-orc/12",
    ],
    Human => tw![
        "text-race-human",
        "border-race-human/60",
        "bg-race-human/12",
    ],
    Undead => tw![
        "text-race-undead",
        "border-race-undead/60",
        "bg-race-undead/12",
    ],
    Success => tw![
        "text-warcraft-success",
        "border-warcraft-success/60",
        "bg-warcraft-success/12",
    ],
}
