use tw_macro::tw;

// The reason pill's entire look, owned here and nowhere else — a private `mod style`,
// never re-exported. Its colour is the `--reason-color` custom property each per-reason
// wrapper publishes, so one pill serves every reason. The reasons reuse this by
// composition (rendering `ReasonBadge`), never by sharing these classes.
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
        "text-(--reason-color)",
        "border-[color-mix(in_oklab,var(--reason-color)_60%,transparent)]",
        "bg-[color-mix(in_oklab,var(--reason-color)_12%,transparent)]",
    ],
}
