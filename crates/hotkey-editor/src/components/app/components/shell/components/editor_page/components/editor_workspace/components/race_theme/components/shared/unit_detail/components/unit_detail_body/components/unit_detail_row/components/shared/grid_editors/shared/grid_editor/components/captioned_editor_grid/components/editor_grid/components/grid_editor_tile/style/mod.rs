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
        "kb-focus:shadow-tile-focus",
        "[body:has(.dragging-source-ghost)_&]:cursor-grabbing",
    ],
}
