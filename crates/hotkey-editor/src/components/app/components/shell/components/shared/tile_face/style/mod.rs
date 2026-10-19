use tw_macro::tw;

classes! {
    base: tw![
        "relative",
        "w-full",
        "aspect-square",
        "@container",
        "rounded-[1.04cqi]",
        "[&:has(.dragging-source-ghost)_.tile-badge]:invisible",
    ],
}
