use tw_macro::tw;

classes! {
    base: tw![
        "relative",
        "w-full",
        "aspect-square",
        "rounded-[1.27cqi]",
        "touch-pan-y",
        "cursor-default",
        "has-[.draggable-marker]:cursor-grab",
        "outline-none",
        "kb-focus:[box-shadow:0_0_0_0.52cqi_var(--color-warcraft-gold),0_0_3.1cqi_color-mix(in_oklab,var(--color-warcraft-gold)_55%,transparent)]",
        "[body:has(.dragging-source-ghost)_&]:cursor-grabbing",
    ],
}
