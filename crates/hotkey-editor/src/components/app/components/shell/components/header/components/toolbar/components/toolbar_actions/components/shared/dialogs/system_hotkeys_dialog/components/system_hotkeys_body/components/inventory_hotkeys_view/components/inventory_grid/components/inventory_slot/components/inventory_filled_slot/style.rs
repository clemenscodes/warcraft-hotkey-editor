use super::state::InventoryFilledSlotState;
use tw_macro::tw;

classes! {
    base: tw![
        "relative",
        "group/tooltip",
        "flex",
        "flex-col",
        "items-center",
        "justify-center",
        "gap-2",
        "px-2.5",
        "py-3.5",
        "cursor-pointer",
        "text-center",
        "select-none",
        "border-solid",
        "border-12",
        "bg-panel-dark",
        "[border-image-source:var(--wc3-slot-frame)]",
        "[border-image-slice:12_fill]",
        "[border-image-repeat:stretch]",
        "touch-none",
        "transition-[filter]",
        "[&:hover]:filter-[brightness(1.18)_drop-shadow(0_0_8px_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent))]",
        "focus:outline-none",
        "kb-focus:outline-none",
        "kb-focus:filter-[brightness(1.25)_drop-shadow(0_0_10px_color-mix(in_oklab,var(--color-warcraft-highlight)_55%,transparent))]",
        "data-[dragging=true]:*:invisible",
    ],
    mobile: tw![
        "mobile:border-8",
        "mobile:px-1",
        "mobile:py-2",
        "mobile:gap-1",
        "mobile:aspect-[1/0.85]",
        "mobile:min-h-0",
    ],
    tablet: tw![
        "tablet:border-8",
        "tablet:px-1",
        "tablet:py-2",
        "tablet:gap-1",
        "tablet:aspect-[1/0.85]",
        "tablet:min-h-0",
    ],
}

states! {
    InventoryFilledSlotState,
    Idle => tw![],
    Active => tw!["filter-[brightness(1.32)_drop-shadow(0_0_14px_color-mix(in_oklab,var(--color-warcraft-gold)_75%,transparent))]"],
    Conflict => tw!["filter-[drop-shadow(0_0_12px_color-mix(in_oklab,var(--color-warcraft-danger)_55%,transparent))]"],
}
