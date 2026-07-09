use tw_macro::tw;

// The active variant's OWN root only — it never touches the base's classes. It fills the
// tab slot, establishes the positioning context for the accent overlay it stacks on top,
// and publishes `--label-color` so the base's label (a descendant) turns accent-coloured.
classes! {
    base: tw![
        "relative",
        "size-full",
        "[--label-color:var(--race-accent)]",
    ],
}
