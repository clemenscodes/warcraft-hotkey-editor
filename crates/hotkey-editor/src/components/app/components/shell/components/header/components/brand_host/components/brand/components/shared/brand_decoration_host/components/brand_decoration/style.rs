use tw_macro::tw;
// Fills the host's inline size and keeps its own aspect ratio (`h-auto`), so it
// never distorts. The host owns the box; `100cqi` is the full width of the host's
// container-query context. No fixed length here — size is the host's decision.

classes! {
    base: tw![
        "block",
        "w-[100cqi]",
        "h-auto",
        "flex-none",
        "filter-[drop-shadow(0_1px_0_color-mix(in_oklab,var(--color-warcraft-shadow)_70%,transparent))]",
    ],
}
