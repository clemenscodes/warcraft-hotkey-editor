use tw_macro::tw;
// The coordinate follows its card's state through the `--coordinate-color` the selected
// collision surface publishes (gold when selected, the primary text colour otherwise) —
// not a `group-data-[selected]` selector reaching up to an ancestor's attribute.
classes! {
    base: tw![
        "text-xl",
        "leading-flush",
        "text-[color:var(--coordinate-color,var(--color-warcraft-text-primary))]",
        "whitespace-nowrap",
    ],
}
