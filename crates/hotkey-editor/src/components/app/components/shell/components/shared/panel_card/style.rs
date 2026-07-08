use super::state::PanelCardVariant;
use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "py-6",
        "bg-warcraft-bg-mid/45",
        "border",
        "rounded-panel",
    ],
}

states! {
    PanelCardVariant,
    Move => tw![
        "gap-5",
        "px-6",
        "box-border",
        "border-warcraft-blue-deep",
    ],
    MoveStuck => tw![
        "gap-5",
        "px-6",
        "box-border",
        "border-race-orc/50",
    ],
    Conflict => tw![
        "gap-6",
        "px-4",
        "items-center",
        "min-w-0",
        "border-warcraft-blue-deep",
    ],
}
